use super::config::*;
use super::history::*;
use iced::{
    keyboard, text_input, Application, Column, Container, Element, Length, Subscription, Text,
    TextInput,
};
use iced_native::{subscription, Event};
use rayon::prelude::*;
use std::collections::HashMap;

pub struct State {
    input: text_input::State,
    input_value: String,
    cursor: usize,
    page_number: usize,
    bins: Vec<(String, usize)>,
    filtered: Vec<(String, usize)>,
    history: HashMap<String, usize>,
    path: std::path::PathBuf,
}

#[derive(Clone, Debug)]
pub enum Message {
    InputChanged(String),
    MoveCursor(Move),
    Execute,
    Exit,
}

#[derive(Clone, Debug)]
pub enum Move {
    Up,
    Down,
}

impl Default for State {
    fn default() -> Self {
        let history_path = history_path();

        let config = read_config();
        let mut path_vec = vec![];
        match config {
            Some(config) => {
                for path in config.paths {
                    path_vec.push(std::path::PathBuf::from(path));
                }
            }
            None => {
                path_vec.push(std::path::PathBuf::from("/usr/bin"));
            }
        }

        let mut bin_vec = vec![];
        for path in path_vec {
            for bin in std::fs::read_dir(&path).unwrap() {
                let bin = bin.unwrap();
                let name = bin.file_name().into_string().unwrap();
                bin_vec.push(name);
            }
        }

        let map = if history_path.exists() {
            read_history(&history_path).unwrap().history_map
        } else {
            HashMap::new()
        };

        let mut vec = vec![];
        let mut unused_vec: Vec<(String, usize)> = vec![];
        for bin in bin_vec {
            match map.get(&bin) {
                Some(x) => vec.push((bin, *x)),
                None => unused_vec.push((bin, 0)),
            }
        }

        vec.par_sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        vec.append(&mut unused_vec);

        State {
            input: text_input::State::focused(),
            input_value: "".to_string(),
            cursor: 0,
            page_number: 0,
            bins: vec.clone(),
            filtered: vec,
            history: map,
            path: history_path,
        }
    }
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (Self::default(), iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Launcher - Iced")
    }

    fn view(&mut self) -> Element<Message> {
        let text_input = TextInput::new(
            &mut self.input,
            "",
            &self.input_value,
            Message::InputChanged,
        )
        .style(super::style::TextInput);

        let bins_list: Element<Message> = {
            self.filtered
                .iter()
                .skip(self.page_number * 7)
                .take(7)
                .enumerate()
                .fold(Column::new(), |column, (i, item)| {
                    if (i + (self.page_number * 7)) == self.cursor {
                        column.push(Element::new(Text::new(&item.0).color([1.0, 0.5, 0.0])))
                    } else {
                        column.push(Element::new(Text::new(&item.0)))
                    }
                })
                .into()
        };

        let content = Column::new()
            .padding(17)
            .spacing(5)
            .push(text_input)
            .push(bins_list);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(super::style::Container)
            .into()
    }

    fn update(
        &mut self,
        message: Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Message> {
        let len = self.filtered.len();

        match message {
            Message::InputChanged(words) => {
                self.input_value = words;
                self.filtered = self
                    .bins
                    .par_iter()
                    .filter(|&item| (*item.0).contains(&self.input_value))
                    .cloned()
                    .collect();
                self.cursor = 0;
                self.page_number = 0;
            }
            Message::MoveCursor(mv) => match mv {
                Move::Up => {
                    if self.cursor > 0 {
                        if self.cursor % 7 == 0 {
                            self.page_number -= 1;
                        }
                        self.cursor -= 1;
                    } else {
                    }
                }
                Move::Down => {
                    if self.cursor < len - 1 {
                        if (self.cursor + 1) % 7 == 0 {
                            self.page_number += 1;
                        }
                        self.cursor += 1;
                    } else {
                    }
                }
            },
            Message::Execute => {
                let bin = self.filtered.get(self.cursor);
                if let Some(bin) = bin {
                    std::process::exit(match launch_app(&bin.0) {
                        Ok(_) => {
                            let x = self.history.entry(bin.0.clone()).or_insert(0);
                            *x += 1;
                            update_history(&self.history, &self.path).unwrap();
                            0
                        }
                        Err(e) => {
                            eprintln!("error: {:?}", e);
                            1
                        }
                    });
                }
            }
            Message::Exit => {
                std::process::exit(0);
            }
        }
        iced::Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                modifiers: _,
                key_code,
            }) => handle_key(key_code),
            _ => None,
        })
    }
}

fn handle_key(key_code: keyboard::KeyCode) -> Option<Message> {
    match key_code {
        keyboard::KeyCode::Up => Some(Message::MoveCursor(Move::Up)),
        keyboard::KeyCode::Down | keyboard::KeyCode::Tab => Some(Message::MoveCursor(Move::Down)),
        keyboard::KeyCode::Enter => Some(Message::Execute),
        keyboard::KeyCode::Escape => Some(Message::Exit),
        _ => None,
    }
}

fn launch_app(bin: &str) -> std::io::Result<std::process::Child> {
    std::process::Command::new(bin).spawn()
}

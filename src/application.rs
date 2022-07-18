use super::config::*;
use super::generate::generate_bin_vec;
use super::history::*;
use iced::{
    keyboard, text_input, Application, Column, Container, Element, Length, Subscription, Text,
    TextInput,
};
use iced_native::{subscription, Event};
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
        let path_vec = generate_path_vec();
        let bin_vec = generate_bin_vec(path_vec).unwrap_or_default();

        let history_path = history_path().unwrap_or_default();
        let mut history_map = if history_path.exists() {
            read_history(&history_path)
                .unwrap_or(History {
                    history_map: HashMap::new(),
                })
                .history_map
        } else {
            HashMap::new()
        };
        let history_map_clone = history_map.clone();

        let mut used_bins = vec![];
        let mut unused_bins: Vec<(String, usize)> = vec![];
        for bin in bin_vec {
            if !history_map.is_empty() {
                match history_map.get(&bin) {
                    Some(x) => {
                        used_bins.push((bin.clone(), *x));
                        let _removed = history_map.remove(&bin);
                    }
                    None => unused_bins.push((bin, 0)),
                }
            } else {
                unused_bins.push((bin, 0));
            }
        }

        used_bins.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        used_bins.append(&mut unused_bins);

        State {
            input: text_input::State::focused(),
            input_value: "".to_string(),
            cursor: 0,
            page_number: 0,
            bins: used_bins.clone(),
            filtered: used_bins,
            history: history_map_clone,
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
        String::from("feu")
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

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        let len = self.filtered.len();

        match message {
            Message::InputChanged(words) => {
                self.input_value = words;

                let mut new_filtered = vec![];
                for bin in &self.bins {
                    if bin.0.contains(&self.input_value) {
                        new_filtered.push((bin.0.clone(), bin.1));
                    }
                }
                self.filtered = new_filtered;

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
                            if let Err(e) = update_history(&self.history, &self.path) {
                                eprintln!("Error when updating history: {}", e);
                            }
                            0
                        }
                        Err(_) => {
                            eprintln!("Error when launching {}.", bin.0);
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

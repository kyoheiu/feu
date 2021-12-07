use super::history::*;
use iced::{
    keyboard, text_input, Application, Column, Container, Element, Length, Subscription, Text,
    TextInput,
};
use iced_native::{subscription, Event};
use std::collections::HashMap;

pub struct Lists {
    input: text_input::State,
    input_value: String,
    cursor: usize,
    page_number: usize,
    bins: Vec<(String, usize)>,
    filtered: Vec<(String, usize)>,
}

#[derive(Clone, Debug)]
pub enum Message {
    InputChanged(String),
    MoveCursor(Move),
    Execute,
}

#[derive(Clone, Debug)]
pub enum Move {
    Up,
    Down,
}

impl Default for Lists {
    fn default() -> Self {
        let mut bin_vec = vec![];
        for bin in std::fs::read_dir("/usr/bin").unwrap() {
            let bin = bin.unwrap();
            let name = bin.file_name().into_string().unwrap();
            bin_vec.push(name);
        }

        let map = if !history_path().exists() {
            let mut map: HashMap<String, usize> = HashMap::new();
            for bin in &bin_vec {
                let _ = map.insert(bin.to_string(), 0);
            }
            update_history(map.clone()).unwrap();
            map
        } else {
            let mut map = read_history().unwrap().history_map;
            for bin in bin_vec {
                map.entry(bin).or_insert(0);
            }
            map
        };

        let mut bin_vec = map.into_iter().collect::<Vec<(String, usize)>>();
        bin_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        Lists {
            input: text_input::State::focused(),
            input_value: "".to_string(),
            cursor: 0,
            page_number: 0,
            bins: bin_vec.clone(),
            filtered: bin_vec,
        }
    }
}

impl Application for Lists {
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
        );

        let bins_list: Element<Message> = {
            self.filtered
                .iter()
                .skip(self.page_number * 7)
                .take(7)
                .enumerate()
                .fold(Column::new(), |column, (i, item)| {
                    if (i + (self.page_number * 7)) == self.cursor {
                        column.push(Element::new(
                            Text::new(item.0.clone()).color([1.0, 0.5, 0.0]),
                        ))
                    } else {
                        column.push(Element::new(Text::new(item.0.clone())))
                    }
                })
                .into()
        };

        let content = Column::new().padding(20).push(text_input).push(bins_list);

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
                    .iter()
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
                            let mut map = self
                                .bins
                                .clone()
                                .into_iter()
                                .collect::<HashMap<String, usize>>();
                            if let Some(x) = map.get_mut(&bin.0) {
                                *x += 1;
                            }
                            update_history(map).unwrap();
                            0
                        }
                        Err(e) => {
                            eprintln!("error: {:?}", e);
                            1
                        }
                    });
                }
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
        _ => None,
    }
}

fn launch_app(bin: &str) -> std::io::Result<std::process::Child> {
    std::process::Command::new(bin).spawn()
}

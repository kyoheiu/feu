use super::config::*;
use super::errors::*;
use super::history::*;
use iced::{
    keyboard, text_input, Application, Column, Container, Element, Length, Rule, Subscription,
    Text, TextInput,
};
use iced_native::{subscription, Event};
use std::collections::BTreeMap;
use std::process::{Command, Stdio};

const PADDING: u16 = 17;
const SPACING: u16 = 5;
const COLUMNS: usize = 7;
const RULE_HEIGHT: u16 = 10;
const HIGHLIGHT: [f32; 4] = [0.72, 0.84, 0.227, 1.0]; //#B7D63A

pub struct State {
    input: text_input::State,
    input_value: String,
    cursor: usize,
    page_number: usize,
    bins: Vec<(String, usize)>,
    filtered: Vec<(String, usize)>,
    history: BTreeMap<String, usize>,
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
    // Right,
    // Left,
}

impl Default for State {
    fn default() -> Self {
        let bin_source = generate_bin_set();
        if bin_source.is_err() {
            eprintln!("{:?}", bin_source.as_ref().unwrap_err());
        }
        let bin_source = bin_source.unwrap();

        let history_path = history_path().unwrap_or_default();
        let history_map = if history_path.exists() {
            read_history(&history_path)
                .unwrap_or(History {
                    history_map: BTreeMap::new(),
                })
                .history_map
        } else {
            BTreeMap::new()
        };
        let history_map_clone = history_map.clone();

        let mut used_bins = vec![];
        let mut unused_bins: Vec<(String, usize)> = vec![];

        if !history_map.is_empty() {
            for b in bin_source {
                match history_map.get(&b) {
                    Some(i) => {
                        used_bins.push((b, *i));
                    }
                    None => {
                        unused_bins.push((b, 0));
                    }
                }
            }
            used_bins.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            used_bins.append(&mut unused_bins);
        } else {
            used_bins = bin_source.iter().map(|x| (x.to_string(), 0)).collect();
        }

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

        let rule = Rule::horizontal(RULE_HEIGHT).style(super::style::Rule);

        let bins_list: Element<Message> = {
            self.filtered
                .iter()
                .skip(self.page_number * COLUMNS)
                .take(7)
                .enumerate()
                .fold(Column::new(), |column, (i, item)| {
                    if (i + (self.page_number * COLUMNS)) == self.cursor {
                        column.push(Element::new(Text::new(&item.0).color(HIGHLIGHT)))
                    } else {
                        column.push(Element::new(Text::new(&item.0)))
                    }
                })
                .into()
        };

        let content = Column::new()
            .padding(PADDING)
            .spacing(SPACING)
            .push(text_input)
            .push(rule)
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
                        if self.cursor % COLUMNS == 0 {
                            self.page_number -= 1;
                        }
                        self.cursor -= 1;
                    } else {
                    }
                }
                Move::Down => {
                    if self.cursor < len - 1 {
                        if (self.cursor + 1) % COLUMNS == 0 {
                            self.page_number += 1;
                        }
                        self.cursor += 1;
                    } else {
                    }
                } // Move::Right => {
                  //     if len > COLUMNS && self.cursor < len - COLUMNS {
                  //         self.cursor += COLUMNS;
                  //         self.page_number += 1;
                  //     } else {
                  //     }
                  // }
                  // Move::Left => {
                  //     if self.cursor > COLUMNS - 1 {
                  //         self.cursor -= COLUMNS;
                  //         self.page_number -= 1;
                  //     } else {
                  //     }
                  // }
            },
            Message::Execute => {
                let bin = self.filtered.get(self.cursor);
                if let Some(bin) = bin {
                    std::process::exit(match launch_app(&bin.0) {
                        Ok(_) => {
                            let x = self.history.entry(bin.0.clone()).or_insert(0);
                            *x += 1;
                            if let Err(e) = update_history(&self.history, &self.path) {
                                eprintln!("Error when updating history: {:#?}", e);
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
        // keyboard::KeyCode::Right => Some(Message::MoveCursor(Move::Right)),
        // keyboard::KeyCode::Left => Some(Message::MoveCursor(Move::Left)),
        keyboard::KeyCode::Enter => Some(Message::Execute),
        keyboard::KeyCode::Escape => Some(Message::Exit),
        _ => None,
    }
}

fn launch_app(bin: &str) -> Result<(), FeuError> {
    match unsafe { nix::unistd::fork() } {
        Ok(result) => match result {
            nix::unistd::ForkResult::Parent { child } => {
                nix::sys::wait::waitpid(Some(child), None)?;
                Ok(())
            }
            nix::unistd::ForkResult::Child => {
                nix::unistd::setsid()?;
                let mut ex = Command::new(bin);
                ex.stdout(Stdio::null())
                    .stdin(Stdio::null())
                    .spawn()
                    .and(Ok(()))
                    .map_err(|_| FeuError("Cannot spawn a new process.".to_string()))?;
                drop(ex);
                std::process::exit(0);
            }
        },
        Err(_e) => Err(FeuError("Cannot fork.".to_string())),
    }
}

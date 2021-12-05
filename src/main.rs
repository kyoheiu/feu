use iced::scrollable::{self, Scrollable};
use iced::{
    keyboard, text_input, Application, Column, Container, Element, Settings, Subscription, Text,
    TextInput,
};
use iced_native::{event, subscription, Event};

struct Lists {
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    cursor: usize,
    bins: Vec<String>,
    filtered: Vec<String>,
}

#[derive(Clone, Debug)]
enum Message {
    InputChanged(String),
    MoveCursor(Move),
    Execute,
}

#[derive(Clone, Debug)]
enum Move {
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
        bin_vec.sort();

        Lists {
            scroll: scrollable::State::new(),
            input: text_input::State::focused(),
            input_value: "".to_string(),
            cursor: 1,
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
        let bins_list: Element<Message> = {
            self.filtered
                .iter()
                .take(5)
                .enumerate()
                .fold(Scrollable::new(&mut self.scroll), |column, (i, item)| {
                    if i + 1 == self.cursor {
                        column.push(Element::new(Text::new(item).color([0.5, 0.5, 0.5])))
                    } else {
                        column.push(Element::new(Text::new(item)))
                    }
                })
                .into()
        };

        let content = Column::new()
            .padding(20)
            .push(TextInput::new(
                &mut self.input,
                "Run!",
                &self.input_value,
                Message::InputChanged,
            ))
            .push(bins_list);

        Container::new(content).into()
    }

    fn update(
        &mut self,
        message: Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Message> {
        let len = self.bins.len();

        match message {
            Message::InputChanged(words) => {
                self.input_value = words;
                self.filtered = self
                    .bins
                    .iter()
                    .filter(|&item| (*item).contains(&self.input_value))
                    .map(|item| item.to_string())
                    .collect();
            }
            Message::MoveCursor(mv) => match mv {
                Move::Up => {
                    if self.cursor > 1 {
                        self.cursor -= 1;
                    } else {
                    }
                }
                Move::Down => {
                    if self.cursor < len {
                        self.cursor += 1;
                    } else {
                    }
                }
                _ => {}
            },
            Message::Execute => {}
        }
        iced::Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| {
            if let event::Status::Captured = status {
                return None;
            }

            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    modifiers,
                    key_code,
                }) => handle_key(key_code),
                _ => None,
            }
        })
    }
}

fn handle_key(key_code: keyboard::KeyCode) -> Option<Message> {
    match key_code {
        keyboard::KeyCode::Up => Some(Message::MoveCursor(Move::Up)),
        keyboard::KeyCode::Down => Some(Message::MoveCursor(Move::Down)),
        _ => None,
    }
}

fn main() -> iced::Result {
    let window_setting = iced::window::Settings {
        size: (500, 250),
        min_size: None,
        max_size: None,
        resizable: false,
        decorations: false,
        transparent: true,
        always_on_top: true,
        icon: None,
    };
    let setting = Settings {
        window: window_setting,
        ..Default::default()
    };
    Lists::run(setting)
}

use iced::{text_input, Column, Element, Sandbox, Settings, Text, TextInput};

#[derive(Default)]
struct Lists {
    input: text_input::State,
    input_value: String,
    cursor: usize,
    bins: Vec<String>,
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

impl Sandbox for Lists {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Launcher - Iced")
    }

    fn view(&mut self) -> Element<Message> {
        let input = TextInput::new(
            &mut self.input,
            "Run!",
            &self.input_value,
            Message::InputChanged,
        );

        let bins_list: Element<Message> = {
            let mut bins = vec![];
            for bin in std::fs::read_dir("/usr/bin").unwrap() {
                let bin = bin.unwrap();
                let name = bin.file_name().into_string().unwrap();
                bins.push(name);
            }
            self.bins = bins;

            self.bins
                .iter()
                .take(10)
                .fold(Column::new(), |column, item| {
                    column.push(Element::new(Text::new(item)))
                })
                .into()
        };

        Column::new()
            .padding(20)
            .push(TextInput::new(
                &mut self.input,
                "Run!",
                &self.input_value,
                Message::InputChanged,
            ))
            .push(bins_list)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(filter) => {
                self.input_value = filter;
                // self.bins = self
                //     .bins
                //     .iter()
                //     .filter(|item| (*item).contains(&filter))
                //     .map(|item| item.to_string())
                //     .collect();
            }
            Message::MoveCursor(mv) => {}
            Message::Execute => {}
        }
    }
}

fn main() -> iced::Result {
    let window_setting = iced::window::Settings {
        size: (800, 400),
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

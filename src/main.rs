use iced::{text_input, Column, Element, Sandbox, Settings, Text, TextInput};

struct Lists {
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
            input: text_input::State::focused(),
            input_value: "".to_string(),
            cursor: 1,
            bins: bin_vec.clone(),
            filtered: bin_vec,
        }
    }
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
            self.filtered
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
            Message::InputChanged(words) => {
                self.input_value = words;
                self.filtered = self
                    .bins
                    .iter()
                    .filter(|&item| (*item).contains(&self.input_value))
                    .map(|item| item.to_string())
                    .collect();
            }
            Message::MoveCursor(mv) => {}
            Message::Execute => {}
        }
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

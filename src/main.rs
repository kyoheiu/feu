mod application;
mod config;
mod errors;
mod history;
mod style;

use iced::window::Position;
use iced::Application;

const SIZE: (u32, u32) = (400, 224);

fn main() -> iced::Result {
    let window_setting = iced::window::Settings {
        size: SIZE,
        position: Position::Default,
        min_size: None,
        max_size: None,
        resizable: false,
        decorations: false,
        transparent: false,
        always_on_top: true,
        icon: None,
    };
    let setting = iced::Settings {
        window: window_setting,
        ..Default::default()
    };
    application::State::run(setting)
}

mod application;
mod config;
mod errors;
mod generate;
mod history;
mod style;

use iced::window::Position;
use iced::Application;

fn main() -> iced::Result {
    let window_setting = iced::window::Settings {
        size: (400, 200),
        position: Position::Default,
        min_size: None,
        max_size: None,
        resizable: false,
        decorations: false,
        transparent: true,
        always_on_top: true,
        icon: None,
    };
    let setting = iced::Settings {
        window: window_setting,
        ..Default::default()
    };
    application::State::run(setting)
}

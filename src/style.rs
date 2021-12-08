use iced::{container, text_input, Background, Color};

pub const BACKGROUND_R: u8 = 0x36;
pub const BACKGROUND_G: u8 = 0x39;
pub const BACKGROUND_B: u8 = 0x3F;
pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(
                BACKGROUND_R,
                BACKGROUND_G,
                BACKGROUND_B,
            ))),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub struct TextInput;

impl text_input::StyleSheet for TextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::WHITE,
            ..Default::default()
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style { ..self.active() }
    }

    fn placeholder_color(&self) -> Color {
        Color::WHITE
    }

    fn value_color(&self) -> Color {
        Color::BLACK
    }

    fn selection_color(&self) -> Color {
        Color::WHITE
    }
}

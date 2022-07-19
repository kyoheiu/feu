use iced::{container, rule, text_input, Background, Color};

const BACKGROUND: [f32; 3] = [0.137, 0.113, 0.235]; //#231D3C
const BORDER: [f32; 3] = [0.79, 0.8, 1.0]; //#CACCFF
const TEXT: [f32; 3] = [0.92, 0.92, 0.937]; //#DCDCEF
pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(
                BACKGROUND[0],
                BACKGROUND[1],
                BACKGROUND[2],
            ))),
            text_color: Some(Color::from_rgb(TEXT[0], TEXT[1], TEXT[2])),
            border_width: 1.0,
            border_color: Color::from_rgb(BORDER[0], BORDER[1], BORDER[2]),
            ..container::Style::default()
        }
    }
}

pub struct TextInput;

impl text_input::StyleSheet for TextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::from_rgb(
                BACKGROUND[0],
                BACKGROUND[1],
                BACKGROUND[2],
            )),
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
        Color::from_rgb(TEXT[0], TEXT[1], TEXT[2])
    }

    fn selection_color(&self) -> Color {
        Color::BLACK
    }
}

pub struct Rule;

impl rule::StyleSheet for Rule {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: Color::from_rgb(BORDER[0], BORDER[1], BORDER[2]),
            width: 1,
            radius: 0.0,
            ..rule::Style::default()
        }
    }
}

use iced::widget::text::{Appearance, StyleSheet};
use iced::Color;

#[derive(Debug, Clone, Copy)]
pub struct CustomTextStyle {
    pub appearance: Appearance,
}

impl CustomTextStyle {
    pub const fn new() -> Self {
        Self {
            appearance: Appearance {
                color: Some(Color::BLACK),
            },
        }
    }

    pub const fn color(mut self, color: Color) -> Self {
        self.appearance.color = Some(color);
        self
    }

    pub fn as_custom(&self) -> iced::theme::Text {
        iced::theme::Text::Color(self.appearance.color.unwrap_or(Color::TRANSPARENT))
    }
}

impl StyleSheet for CustomTextStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: Self::Style) -> Appearance {
        self.appearance
    }
}

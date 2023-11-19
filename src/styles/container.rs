use iced::widget::container::{Appearance, StyleSheet};
use iced::{Background, BorderRadius};

#[derive(Debug, Clone, Copy)]
pub struct CustomContainerStyle {
    pub appearance: Appearance,
}

impl CustomContainerStyle {
    pub fn new() -> Self {
        Self {
            appearance: Appearance {
                text_color: None,
                background: None,
                border_radius: Default::default(),
                border_width: 0.0,
                border_color: Default::default(),
            },
        }
    }

    pub fn transparent(theme: &iced::Theme) -> Self {
        Self {
            appearance: theme.appearance(&iced::theme::Container::Transparent),
        }
    }

    pub fn boxx(theme: &iced::Theme) -> Self {
        Self {
            appearance: theme.appearance(&iced::theme::Container::Box),
        }
    }

    pub const fn background(mut self, background: Option<Background>) -> Self {
        self.appearance.background = background;
        self
    }

    pub const fn background_color(mut self, color: iced::Color) -> Self {
        self.appearance.background = Some(Background::Color(color));
        self
    }

    pub const fn border_radius(mut self, radius: BorderRadius) -> Self {
        self.appearance.border_radius = radius;
        self
    }

    pub const fn border_width(mut self, width: f32) -> Self {
        self.appearance.border_width = width;
        self
    }

    pub const fn border_color(mut self, color: iced::Color) -> Self {
        self.appearance.border_color = color;
        self
    }

    pub const fn text_color(mut self, color: Option<iced::Color>) -> Self {
        self.appearance.text_color = color;
        self
    }

    pub fn as_custom(&self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(*self))
    }
}

impl StyleSheet for CustomContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        self.appearance
    }
}

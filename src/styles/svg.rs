use iced::widget::svg::{Appearance, StyleSheet};

#[derive(Debug, Clone, Copy)]
pub struct CustomSvgStyle {
  pub appearance: Appearance,
}

impl CustomSvgStyle {
  pub fn new() -> Self {
    Self {
      appearance: Appearance {
        color: Default::default(),
      },
    }
  }

  pub const fn color(mut self, color: Option<iced::Color>) -> Self {
    self.appearance.color = color;
    self
  }

  pub fn as_custom(&self) -> iced::theme::Svg {
    iced::theme::Svg::Custom(Box::new(*self))
  }
}

impl StyleSheet for CustomSvgStyle {
  type Style = iced::Theme;

  fn appearance(&self, _style: &Self::Style) -> Appearance {
    self.appearance
  }
}

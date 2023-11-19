use iced::widget::text_input::{Appearance, StyleSheet};
use iced::{Background, BorderRadius, Color};

#[derive(Debug, Clone, Copy)]
pub struct CustomTextInputStyle {
  pub active: Appearance,
  pub focused: Appearance,
  pub hovered: Appearance,
  pub disabled: Appearance,
  pub placeholder_color: Color,
  pub value_color: Color,
  pub disabled_color: Color,
  pub selection_color: Color,
  pub current_state: TextInputState,
}

#[derive(Default, Copy, Clone, Debug)]
pub enum TextInputState {
  #[default]
  Active,
  Hovered,
  Disabled,
  Focused,
}

impl CustomTextInputStyle {
  pub fn new() -> Self {
    let default = Appearance {
      background: Background::Color(Color::BLACK),
      border_radius: BorderRadius::from(0.0),
      border_width: 0.0,
      border_color: Color::WHITE,
      icon_color: Color::WHITE,
    };
    Self {
      active: default,
      hovered: default,
      disabled: default,
      focused: default,
      placeholder_color: Default::default(),
      value_color: Default::default(),
      disabled_color: Default::default(),
      selection_color: Default::default(),
      current_state: Default::default(),
    }
  }

  pub fn default(theme: &iced::Theme) -> Self {
    Self {
      active: theme.active(&iced::theme::TextInput::Default),
      hovered: theme.hovered(&iced::theme::TextInput::Default),
      disabled: theme.disabled(&iced::theme::TextInput::Default),
      placeholder_color: theme.placeholder_color(&iced::theme::TextInput::Default),
      value_color: theme.value_color(&iced::theme::TextInput::Default),
      disabled_color: theme.disabled_color(&iced::theme::TextInput::Default),
      focused: theme.focused(&iced::theme::TextInput::Default),
      selection_color: theme.selection_color(&iced::theme::TextInput::Default),
      current_state: Default::default(),
    }
  }

  pub fn active(mut self) -> Self {
    self.current_state = TextInputState::Active;
    self
  }

  pub fn hovered(mut self) -> Self {
    self.current_state = TextInputState::Hovered;
    self
  }

  pub fn disabled(mut self) -> Self {
    self.current_state = TextInputState::Disabled;
    self
  }

  pub fn focused(mut self) -> Self {
    self.current_state = TextInputState::Focused;
    self
  }

  pub fn background(mut self, background: Background) -> Self {
    match self.current_state {
      TextInputState::Active => self.active.background = background,
      TextInputState::Hovered => self.hovered.background = background,
      TextInputState::Disabled => self.disabled.background = background,
      TextInputState::Focused => self.active.background = background,
    }
    self
  }

  pub fn background_color(mut self, color: Color) -> Self {
    match self.current_state {
      TextInputState::Active => self.active.background = Background::Color(color),
      TextInputState::Hovered => self.hovered.background = Background::Color(color),
      TextInputState::Disabled => self.disabled.background = Background::Color(color),
      TextInputState::Focused => self.focused.background = Background::Color(color),
    }
    self
  }

  pub fn border_radius(mut self, radius: BorderRadius) -> Self {
    match self.current_state {
      TextInputState::Active => self.active.border_radius = radius,
      TextInputState::Hovered => self.hovered.border_radius = radius,
      TextInputState::Disabled => self.disabled.border_radius = radius,
      TextInputState::Focused => self.focused.border_radius = radius,
    }
    self
  }

  pub fn border_width(mut self, width: f32) -> Self {
    match self.current_state {
      TextInputState::Active => self.active.border_width = width,
      TextInputState::Hovered => self.hovered.border_width = width,
      TextInputState::Disabled => self.disabled.border_width = width,
      TextInputState::Focused => self.focused.border_width = width,
    }
    self
  }

  pub fn border_color(mut self, color: Color) -> Self {
    match self.current_state {
      TextInputState::Active => self.active.border_color = color,
      TextInputState::Hovered => self.hovered.border_color = color,
      TextInputState::Disabled => self.disabled.border_color = color,
      TextInputState::Focused => self.focused.border_color = color,
    }
    self
  }

  pub fn icon_color(mut self, color: Color) -> Self {
    match self.current_state {
      TextInputState::Active => self.active.icon_color = color,
      TextInputState::Hovered => self.hovered.icon_color = color,
      TextInputState::Disabled => self.disabled.icon_color = color,
      TextInputState::Focused => self.focused.icon_color = color,
    }
    self
  }

  pub fn placeholder_color(mut self, color: Color) -> Self {
    self.placeholder_color = color;
    self
  }

  pub fn value_color(mut self, color: Color) -> Self {
    self.value_color = color;
    self
  }

  pub fn disabled_color(mut self, color: Color) -> Self {
    self.disabled_color = color;
    self
  }

  pub fn selection_color(mut self, color: Color) -> Self {
    self.selection_color = color;
    self
  }

  pub fn as_custom(&self) -> iced::theme::TextInput {
    iced::theme::TextInput::Custom(Box::new(*self))
  }
}

impl StyleSheet for CustomTextInputStyle {
  type Style = iced::Theme;

  fn active(&self, _style: &Self::Style) -> Appearance {
    self.active
  }

  fn focused(&self, _style: &Self::Style) -> Appearance {
    self.focused
  }

  fn placeholder_color(&self, _style: &Self::Style) -> Color {
    self.placeholder_color
  }

  fn value_color(&self, _style: &Self::Style) -> Color {
    self.value_color
  }

  fn disabled_color(&self, _style: &Self::Style) -> Color {
    self.disabled_color
  }

  fn selection_color(&self, _style: &Self::Style) -> Color {
    self.selection_color
  }

  fn hovered(&self, _style: &Self::Style) -> Appearance {
    self.hovered
  }

  fn disabled(&self, _style: &Self::Style) -> Appearance {
    self.disabled
  }
}

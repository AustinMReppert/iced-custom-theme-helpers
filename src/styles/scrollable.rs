use iced::widget::scrollable::{Scrollbar, Scroller, StyleSheet};
use iced::{Background, BorderRadius, Color};

#[derive(Debug, Clone, Copy)]
pub struct CustomScrollableStyle {
  pub active: Scrollbar,
  pub hovered: Scrollbar,
  pub hovered_over_scrollbar: Scrollbar,
  pub dragging: Scrollbar,
  pub active_horizontal: Scrollbar,
  pub hovered_horizontal: Scrollbar,
  pub hovered_horizontal_over_scrollbar: Scrollbar,
  pub dragging_horizontal: Scrollbar,
  pub current_state: ScrollableState,
}

#[derive(Default, Copy, Clone, Debug)]
pub enum ScrollableState {
  #[default]
  Active,
  Hovered,
  HoveredOverScrollbar,
  Dragging,
  ActiveHorizontal,
  HoveredHorizontal,
  HoveredHorizontalOverScrollbar,
  DraggingHorizontal
}

impl CustomScrollableStyle {
  pub fn new() -> Self {
    let default = Scrollbar {
      background: None,
      border_radius: Default::default(),
      border_width: 0.0,
      border_color: Default::default(),
      scroller: Scroller {
        color: Default::default(),
        border_radius: Default::default(),
        border_width: 0.0,
        border_color: Default::default(),
      },
    };
    Self {
      active: default,
      hovered: default,
      hovered_over_scrollbar: default,
      dragging: default,
      active_horizontal: default,
      hovered_horizontal: default,
      hovered_horizontal_over_scrollbar: default,
      dragging_horizontal: default,
      current_state: Default::default(),
    }
  }

  pub fn default(theme: &iced::Theme) -> Self {
    Self {
      active: theme.active(&iced::theme::Scrollable::Default),
      hovered: theme.hovered(&iced::theme::Scrollable::Default, false),
      hovered_over_scrollbar: theme.hovered(&iced::theme::Scrollable::Default, true),
      dragging: theme.dragging(&iced::theme::Scrollable::Default),
      active_horizontal: theme.active_horizontal(&iced::theme::Scrollable::Default),
      hovered_horizontal: theme.hovered_horizontal(&iced::theme::Scrollable::Default, false),
      hovered_horizontal_over_scrollbar: theme.hovered_horizontal(&iced::theme::Scrollable::Default, true),
      dragging_horizontal: theme.dragging_horizontal(&iced::theme::Scrollable::Default),
      current_state: Default::default(),
    }
  }

  pub const fn active(mut self) -> Self {
    self.current_state = ScrollableState::Active;
    self
  }

  pub const fn hovered(mut self) -> Self {
    self.current_state = ScrollableState::Hovered;
    self
  }

  pub const fn hovered_over_scrollbar(mut self) -> Self {
    self.current_state = ScrollableState::HoveredOverScrollbar;
    self
  }

  pub const fn dragging(mut self) -> Self {
    self.current_state = ScrollableState::Dragging;
    self
  }

  pub const fn active_horizontal(mut self) -> Self {
    self.current_state = ScrollableState::ActiveHorizontal;
    self
  }

  pub const fn hovered_horizontal(mut self) -> Self {
    self.current_state = ScrollableState::HoveredHorizontal;
    self
  }

  pub const fn hovered_horizontal_over_scrollbar(mut self) -> Self {
    self.current_state = ScrollableState::HoveredHorizontalOverScrollbar;
    self
  }

  pub const fn dragging_horizontal(mut self) -> Self {
    self.current_state = ScrollableState::DraggingHorizontal;
    self
  }

  pub const fn background(mut self, background: Option<Background>) -> Self {
    match self.current_state {
      ScrollableState::Active => self.active.background = background,
      ScrollableState::Hovered => self.hovered.background = background,
      ScrollableState::HoveredOverScrollbar => {}
      ScrollableState::Dragging => self.dragging.background = background,
      ScrollableState::ActiveHorizontal => self.active_horizontal.background = background,
      ScrollableState::HoveredHorizontal => self.hovered_horizontal.background = background,
      ScrollableState::HoveredHorizontalOverScrollbar => self.hovered_horizontal_over_scrollbar.background = background,
      ScrollableState::DraggingHorizontal => self.dragging.background = background,
    }
    self
  }

  pub const fn background_color(mut self, color: Color) -> Self {
    match self.current_state {
      ScrollableState::Active => self.active.background = Some(Background::Color(color)),
      ScrollableState::Hovered => self.hovered.background = Some(Background::Color(color)),
      ScrollableState::HoveredOverScrollbar => self.hovered_over_scrollbar.background = Some(Background::Color(color)),
      ScrollableState::Dragging => self.dragging.background = Some(Background::Color(color)),
      ScrollableState::ActiveHorizontal => self.active_horizontal.background = Some(Background::Color(color)),
      ScrollableState::HoveredHorizontal => self.hovered_horizontal.background = Some(Background::Color(color)),
      ScrollableState::HoveredHorizontalOverScrollbar => self.hovered_horizontal_over_scrollbar.background = Some(Background::Color(color)),
      ScrollableState::DraggingHorizontal => self.dragging.background = Some(Background::Color(color)),
    }
    self
  }

  pub const fn border_radius(mut self, radius: BorderRadius) -> Self {
    match self.current_state {
      ScrollableState::Active => self.active.border_radius = radius,
      ScrollableState::Hovered => self.hovered.border_radius = radius,
      ScrollableState::HoveredOverScrollbar => self.hovered_over_scrollbar.border_radius = radius,
      ScrollableState::Dragging => self.dragging.border_radius = radius,
      ScrollableState::ActiveHorizontal => self.active_horizontal.border_radius = radius,
      ScrollableState::HoveredHorizontal => self.hovered_horizontal.border_radius = radius,
      ScrollableState::HoveredHorizontalOverScrollbar => self.hovered_horizontal_over_scrollbar.border_radius = radius,
      ScrollableState::DraggingHorizontal => self.dragging.border_radius = radius,
    }
    self
  }

  pub const fn border_width(mut self, width: f32) -> Self {
    match self.current_state {
      ScrollableState::Active => self.active.border_width = width,
      ScrollableState::Hovered => self.hovered.border_width = width,
      ScrollableState::HoveredOverScrollbar => self.hovered_over_scrollbar.border_width = width,
      ScrollableState::Dragging => self.dragging.border_width = width,
      ScrollableState::ActiveHorizontal => self.active_horizontal.border_width = width,
      ScrollableState::HoveredHorizontal => self.hovered_horizontal.border_width = width,
      ScrollableState::HoveredHorizontalOverScrollbar => self.hovered_horizontal_over_scrollbar.border_width = width,
      ScrollableState::DraggingHorizontal => self.dragging.border_width = width,
    }
    self
  }

  pub const fn border_color(mut self, color: Color) -> Self {
    match self.current_state {
      ScrollableState::Active => self.active.border_color = color,
      ScrollableState::Hovered => self.hovered.border_color = color,
      ScrollableState::HoveredOverScrollbar => self.hovered_over_scrollbar.border_color = color,
      ScrollableState::Dragging => self.dragging.border_color = color,
      ScrollableState::ActiveHorizontal => self.active_horizontal.border_color = color,
      ScrollableState::HoveredHorizontal => self.hovered_horizontal.border_color = color,
      ScrollableState::HoveredHorizontalOverScrollbar => self.hovered_horizontal_over_scrollbar.border_color = color,
      ScrollableState::DraggingHorizontal => self.dragging.border_color = color,
    }
    self
  }

  pub const fn scroller_color(mut self, color: Color) -> Self {
    match self.current_state {
      ScrollableState::Active => self.active.scroller.color = color,
      ScrollableState::Hovered => self.hovered.scroller.color = color,
      ScrollableState::HoveredOverScrollbar => self.hovered_over_scrollbar.scroller.color = color,
      ScrollableState::Dragging => self.dragging.scroller.color = color,
      ScrollableState::ActiveHorizontal => self.active_horizontal.scroller.color = color,
      ScrollableState::HoveredHorizontal => self.hovered_horizontal.scroller.color = color,
      ScrollableState::HoveredHorizontalOverScrollbar => self.hovered_horizontal_over_scrollbar.scroller.color = color,
      ScrollableState::DraggingHorizontal => self.dragging.scroller.color = color,
    }
    self
  }

  pub const fn scroller_border_radius(mut self, radius: BorderRadius) -> Self {
    match self.current_state {
      ScrollableState::Active => self.active.scroller.border_radius = radius,
      ScrollableState::Hovered => self.hovered.scroller.border_radius = radius,
      ScrollableState::HoveredOverScrollbar => self.hovered_over_scrollbar.scroller.border_radius = radius,
      ScrollableState::Dragging => self.dragging.scroller.border_radius = radius,
      ScrollableState::ActiveHorizontal => self.active_horizontal.scroller.border_radius = radius,
      ScrollableState::HoveredHorizontal => self.hovered_horizontal.scroller.border_radius = radius,
      ScrollableState::HoveredHorizontalOverScrollbar => self.hovered_horizontal_over_scrollbar.scroller.border_radius = radius,
      ScrollableState::DraggingHorizontal => self.dragging.scroller.border_radius = radius,
    }
    self
  }

  pub const fn scroller_border_width(mut self, width: f32) -> Self {
    match self.current_state {
      ScrollableState::Active => self.active.scroller.border_width = width,
      ScrollableState::Hovered => self.hovered.scroller.border_width = width,
      ScrollableState::HoveredOverScrollbar => self.hovered_over_scrollbar.scroller.border_width = width,
      ScrollableState::Dragging => self.dragging.scroller.border_width = width,
      ScrollableState::ActiveHorizontal => self.active_horizontal.scroller.border_width = width,
      ScrollableState::HoveredHorizontal => self.hovered_horizontal.scroller.border_width = width,
      ScrollableState::HoveredHorizontalOverScrollbar => self.hovered_horizontal_over_scrollbar.scroller.border_width = width,
      ScrollableState::DraggingHorizontal => self.dragging.scroller.border_width = width,
    }
    self
  }

  pub const fn scroller_border_color(mut self, color: Color) -> Self {
    match self.current_state {
      ScrollableState::Active => self.active.scroller.border_color = color,
      ScrollableState::Hovered => self.hovered.scroller.border_color = color,
      ScrollableState::HoveredOverScrollbar => self.hovered_over_scrollbar.scroller.border_color = color,
      ScrollableState::Dragging => self.dragging.scroller.border_color = color,
      ScrollableState::ActiveHorizontal => self.active_horizontal.scroller.border_color = color,
      ScrollableState::HoveredHorizontal => self.hovered_horizontal.scroller.border_color = color,
      ScrollableState::HoveredHorizontalOverScrollbar => self.hovered_horizontal_over_scrollbar.scroller.border_color = color,
      ScrollableState::DraggingHorizontal => self.dragging.scroller.border_color = color,
    }
    self
  }

  pub fn as_custom(&self) -> iced::theme::Scrollable {
    iced::theme::Scrollable::Custom(Box::new(*self))
  }
}

impl StyleSheet for CustomScrollableStyle {
  type Style = iced::Theme;

  fn active(&self, _style: &Self::Style) -> Scrollbar {
    self.active
  }

  fn hovered(&self, _style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
    if is_mouse_over_scrollbar {
      self.hovered_over_scrollbar
    } else {
      self.hovered
    }
  }


  fn dragging(&self, _style: &Self::Style) -> Scrollbar {
    self.dragging
  }

  fn active_horizontal(&self, _style: &Self::Style) -> Scrollbar {
    self.active_horizontal
  }

  fn hovered_horizontal(&self, _style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
    if is_mouse_over_scrollbar {
      self.hovered_horizontal_over_scrollbar
    } else {
      self.hovered_horizontal
    }
  }

  fn dragging_horizontal(&self, _style: &Self::Style) -> Scrollbar {
    self.dragging_horizontal
  }
}
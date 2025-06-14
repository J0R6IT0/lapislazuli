use crate::components::button::Button;
use gpui::ElementId;

mod button;

pub fn button(id: impl Into<ElementId>) -> Button {
    Button::new(id)
}

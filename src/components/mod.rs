use crate::components::input::{InputState, TextInput};
use gpui::*;

mod button;
pub mod input;
pub mod progress;
mod separator;

pub use button::Button;
pub use separator::Separator;

pub fn button(id: impl Into<ElementId>) -> Button {
    Button::new(id)
}

pub fn text_input(state: Entity<InputState>) -> TextInput {
    TextInput::new(state)
}

pub fn separator() -> Separator {
    Separator::new()
}

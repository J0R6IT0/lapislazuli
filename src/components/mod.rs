use gpui::*;

mod button;
pub mod input;
mod progress;
mod separator;

pub use button::Button;
pub use progress::Progress;
pub use separator::Separator;

use crate::components::input::{InputState, TextInput};

pub fn button(id: impl Into<ElementId>) -> Button {
    Button::new(id)
}

pub fn text_input(state: Entity<InputState>) -> TextInput {
    TextInput::new(state)
}

pub fn progress() -> Progress {
    Progress::new()
}

pub fn separator() -> Separator {
    Separator::new()
}

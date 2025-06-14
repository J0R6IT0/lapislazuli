use crate::components::{button::Button, progress::Progress};
use gpui::ElementId;

mod button;
mod progress;

pub fn button(id: impl Into<ElementId>) -> Button {
    Button::new(id)
}

pub fn progress() -> Progress {
    Progress::new()
}

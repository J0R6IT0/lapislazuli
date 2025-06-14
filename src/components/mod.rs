use crate::components::{button::Button, progress::Progress, separator::Separator};
use gpui::ElementId;

mod button;
mod progress;
mod separator;

pub fn button(id: impl Into<ElementId>) -> Button {
    Button::new(id)
}

pub fn progress() -> Progress {
    Progress::new()
}

pub fn separator() -> Separator {
    Separator::new()
}

use gpui::*;

/// Shorthand for creating a `Div` element with a text child.
pub fn span(text: impl Into<SharedString>) -> Div {
    div().child(text.into())
}

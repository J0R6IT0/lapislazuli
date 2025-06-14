use gpui::*;

pub fn span(text: impl Into<SharedString>) -> Div {
    div().child(text.into())
}

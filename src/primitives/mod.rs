use gpui::{
    Div, InteractiveElement, ParentElement, SharedString, Stateful, StatefulInteractiveElement,
    Styled, div,
};

mod button;
pub mod text_field;

pub use button::*;

/// Shorthand for creating a vertical flex `Div` element.
pub fn v_flex() -> Div {
    div().flex().flex_col()
}

/// Shorthand for creating a horizontal flex `Div` element.
pub fn h_flex() -> Div {
    div().flex().flex_row()
}

/// Shorthand for creating a vertical flex `Div` element with center alignment.
pub fn v_flex_center() -> Div {
    v_flex().justify_center().items_center()
}

/// Shorthand for creating a horizontal flex `Div` element with center alignment.
pub fn h_flex_center() -> Div {
    h_flex().justify_center().items_center()
}

/// Shorthand for creating a `Div` element with a text child.
pub fn span(text: impl Into<SharedString>) -> Div {
    div().child(text.into())
}

/// Shorthand for creating a `Div` element with an anchor (`<a>`) behavior.
pub fn a(href: impl Into<SharedString>) -> Stateful<Div> {
    let href: SharedString = href.into();
    div().id("a").on_click(move |_, _, app| {
        app.open_url(&href);
    })
}

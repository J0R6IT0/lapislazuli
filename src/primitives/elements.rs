use gpui::*;

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

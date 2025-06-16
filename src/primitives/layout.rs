use gpui::*;

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

use gpui::*;

/// Shorthand for creating a vertical flex `Div` element.
pub fn v_flex() -> Div {
    div().flex().flex_col()
}

/// Shorthand for creating a horizontal flex `Div` element.
pub fn h_flex() -> Div {
    div().flex().flex_row()
}

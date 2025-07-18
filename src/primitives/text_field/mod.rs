use crate::{Disableable, primitives::h_flex_center};
use gpui::{
    App, CursorStyle, Div, Entity, Focusable, InteractiveElement, Interactivity, IntoElement,
    MouseButton, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
    prelude::FluentBuilder,
};

mod actions;
mod cursor;
mod element;
mod events;
mod history;
mod state;
#[cfg(test)]
mod tests;
mod text_ops;

pub use actions::init;
pub use events::*;
pub use state::*;

/// Context identifier for text field key bindings
const CONTEXT: &str = "lp-text-field";

pub fn text_field(state: Entity<TextFieldState>) -> TextField {
    TextField {
        base: h_flex_center().cursor(CursorStyle::IBeam),
        state,
        disabled: false,
    }
}

#[derive(IntoElement)]
pub struct TextField {
    base: Div,
    state: Entity<TextFieldState>,
    disabled: bool,
}

impl Styled for TextField {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for TextField {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl Disableable for TextField {
    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for TextField {
    fn render(self, window: &mut Window, app: &mut App) -> impl IntoElement {
        self.base
            .when(!self.disabled, |this| {
                this.key_context(CONTEXT)
                    .track_focus(&self.state.focus_handle(app))
                    .on_action(window.listener_for(&self.state, TextFieldState::backspace))
                    .on_action(window.listener_for(&self.state, TextFieldState::delete))
                    .on_action(window.listener_for(&self.state, TextFieldState::left))
                    .on_action(window.listener_for(&self.state, TextFieldState::right))
                    .on_action(window.listener_for(&self.state, TextFieldState::select_left))
                    .on_action(window.listener_for(&self.state, TextFieldState::select_right))
                    .on_action(window.listener_for(&self.state, TextFieldState::select_all))
                    .on_action(window.listener_for(&self.state, TextFieldState::home))
                    .on_action(window.listener_for(&self.state, TextFieldState::end))
                    .on_action(
                        window.listener_for(&self.state, TextFieldState::show_character_palette),
                    )
                    .on_action(window.listener_for(&self.state, TextFieldState::paste))
                    .on_action(window.listener_for(&self.state, TextFieldState::cut))
                    .on_action(window.listener_for(&self.state, TextFieldState::copy))
                    .on_action(window.listener_for(&self.state, TextFieldState::delete_word_left))
                    .on_action(window.listener_for(&self.state, TextFieldState::delete_word_right))
                    .on_action(
                        window.listener_for(&self.state, TextFieldState::delete_to_beginning),
                    )
                    .on_action(window.listener_for(&self.state, TextFieldState::delete_to_end))
                    .on_action(window.listener_for(&self.state, TextFieldState::word_left))
                    .on_action(window.listener_for(&self.state, TextFieldState::word_right))
                    .on_action(window.listener_for(&self.state, TextFieldState::select_word_left))
                    .on_action(window.listener_for(&self.state, TextFieldState::select_word_right))
                    .on_action(window.listener_for(&self.state, TextFieldState::select_to_home))
                    .on_action(window.listener_for(&self.state, TextFieldState::select_to_end))
                    .on_action(window.listener_for(&self.state, TextFieldState::undo))
                    .on_action(window.listener_for(&self.state, TextFieldState::redo))
                    .on_action(window.listener_for(&self.state, TextFieldState::enter))
                    .on_mouse_down(
                        MouseButton::Left,
                        window.listener_for(&self.state, TextFieldState::on_mouse_down),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        window.listener_for(&self.state, TextFieldState::on_mouse_up),
                    )
                    .on_mouse_up_out(
                        MouseButton::Left,
                        window.listener_for(&self.state, TextFieldState::on_mouse_up),
                    )
                    .on_mouse_move(window.listener_for(&self.state, TextFieldState::on_mouse_move))
            })
            .on_scroll_wheel(window.listener_for(&self.state, TextFieldState::on_scroll_wheel))
            .child(self.state.clone())
    }
}

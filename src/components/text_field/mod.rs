use crate::primitives::h_flex_center;
use gpui::{
    AnyElement, App, CursorStyle, Div, Entity, Focusable, InteractiveElement, Interactivity,
    IntoElement, KeyBinding, MouseButton, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window, actions, prelude::FluentBuilder,
};

mod cursor;
mod element;
mod history;
mod state;
#[cfg(test)]
mod tests;
mod text_ops;

pub use state::*;

/// Context identifier for input key bindings
const CONTEXT: &str = "input";

pub fn text_field(state: Entity<TextFieldState>) -> TextField {
    TextField {
        base: h_flex_center().cursor(CursorStyle::IBeam),
        state,
        leading: None,
    }
}

/// Initialize input key bindings and actions
pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("left", Left, Some(CONTEXT)),
        KeyBinding::new("right", Right, Some(CONTEXT)),
        KeyBinding::new("home", Home, Some(CONTEXT)),
        KeyBinding::new("end", End, Some(CONTEXT)),
        KeyBinding::new("alt-left", WordLeft, Some(CONTEXT)),
        KeyBinding::new("alt-right", WordRight, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-a", Home, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-left", Home, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-e", End, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-right", End, Some(CONTEXT)),
        KeyBinding::new("shift-left", SelectLeft, Some(CONTEXT)),
        KeyBinding::new("shift-right", SelectRight, Some(CONTEXT)),
        KeyBinding::new("cmd-a", SelectAll, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some(CONTEXT)),
        KeyBinding::new("alt-shift-left", SelectWordLeft, Some(CONTEXT)),
        KeyBinding::new("alt-shift-right", SelectWordRight, Some(CONTEXT)),
        KeyBinding::new("shift-home", SelectToHome, Some(CONTEXT)),
        KeyBinding::new("shift-end", SelectToEnd, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-left", SelectToHome, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-right", SelectToEnd, Some(CONTEXT)),
        KeyBinding::new("backspace", Backspace, Some(CONTEXT)),
        KeyBinding::new("delete", Delete, Some(CONTEXT)),
        KeyBinding::new("alt-backspace", DeleteWordLeft, Some(CONTEXT)),
        KeyBinding::new("alt-delete", DeleteWordRight, Some(CONTEXT)),
        KeyBinding::new("cmd-backspace", DeleteToBeginning, Some(CONTEXT)),
        KeyBinding::new("cmd-delete", DeleteToEnd, Some(CONTEXT)),
        KeyBinding::new("cmd-c", Copy, Some(CONTEXT)),
        KeyBinding::new("cmd-v", Paste, Some(CONTEXT)),
        KeyBinding::new("cmd-x", Cut, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some(CONTEXT)),
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-z", Undo, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-y", Redo, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-shift-z", Redo, Some(CONTEXT)),
        KeyBinding::new("cmd-z", Undo, Some(CONTEXT)),
        KeyBinding::new("cmd-shift-z", Redo, Some(CONTEXT)),
        KeyBinding::new("enter", Enter, Some(CONTEXT)),
    ]);
}

actions!(
    input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        ShowCharacterPalette,
        Copy,
        Paste,
        Cut,
        DeleteWordLeft,
        DeleteWordRight,
        DeleteToBeginning,
        DeleteToEnd,
        WordLeft,
        WordRight,
        SelectWordLeft,
        SelectWordRight,
        SelectToHome,
        SelectToEnd,
        Undo,
        Redo,
        Enter,
    ]
);

#[derive(IntoElement)]
pub struct TextField {
    base: Div,
    state: Entity<TextFieldState>,
    leading: Option<AnyElement>,
}

impl TextField {
    pub fn leading(mut self, element: impl IntoElement) -> Self {
        self.leading = Some(element.into_any_element());
        self
    }
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

impl RenderOnce for TextField {
    fn render(self, window: &mut Window, app: &mut App) -> impl IntoElement {
        self.base
            .key_context(CONTEXT)
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
            .on_action(window.listener_for(&self.state, TextFieldState::show_character_palette))
            .on_action(window.listener_for(&self.state, TextFieldState::paste))
            .on_action(window.listener_for(&self.state, TextFieldState::cut))
            .on_action(window.listener_for(&self.state, TextFieldState::copy))
            .on_action(window.listener_for(&self.state, TextFieldState::delete_word_left))
            .on_action(window.listener_for(&self.state, TextFieldState::delete_word_right))
            .on_action(window.listener_for(&self.state, TextFieldState::delete_to_beginning))
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
            .on_scroll_wheel(window.listener_for(&self.state, TextFieldState::on_scroll_wheel))
            .when_some(self.leading, |this, leading| this.child(leading))
            .child(self.state.clone())
    }
}

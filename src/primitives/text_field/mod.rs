use crate::{
    Disableable,
    primitives::{h_flex_center, text_field::state::TextFieldState},
};
use gpui::{
    App, AppContext, CursorStyle, Div, ElementId, Entity, Focusable, Hsla, InteractiveElement,
    Interactivity, IntoElement, MouseButton, ParentElement, RenderOnce, SharedString, Stateful,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, prelude::FluentBuilder,
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

pub(super) use actions::init;
pub use events::*;

/// Context identifier for text field key bindings
const CONTEXT: &str = "lp-text-field";

pub fn text_field(id: impl Into<ElementId>) -> TextField {
    let id = id.into();
    TextField {
        id: id.clone(),
        base: h_flex_center().id(id).cursor(CursorStyle::IBeam),
        disabled: false,
        value: None,
        on_input: None,
        on_change: None,
        placeholder: None,
        placeholder_color: None,
        selection_color: None,
        masked: false,
        mask: None,
        max_length: None,
        validator: None,
        tab_index: 0,
        tab_stop: true,
    }
}

#[derive(IntoElement)]
pub struct TextField {
    id: ElementId,
    base: Stateful<Div>,
    disabled: bool,
    value: Option<SharedString>,
    on_input: Option<Box<dyn Fn(&InputEvent, &mut Window, &mut App) + 'static>>,
    on_change: Option<Box<dyn Fn(&ChangeEvent, &mut Window, &mut App) + 'static>>,
    placeholder: Option<SharedString>,
    placeholder_color: Option<Hsla>,
    selection_color: Option<Hsla>,
    masked: bool,
    mask: Option<SharedString>,
    max_length: Option<usize>,
    validator: Option<Box<dyn Fn(SharedString) -> bool + 'static>>,
    tab_index: isize,
    tab_stop: bool,
}

impl TextField {
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn on_input(
        mut self,
        callback: impl Fn(&InputEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_input = Some(Box::new(callback));
        self
    }

    pub fn on_change(
        mut self,
        callback: impl Fn(&ChangeEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(callback));
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn placeholder_color(mut self, color: impl Into<Hsla>) -> Self {
        self.placeholder_color = Some(color.into());
        self
    }

    pub fn selection_color(mut self, color: impl Into<Hsla>) -> Self {
        self.selection_color = Some(color.into());
        self
    }

    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }

    pub fn mask(mut self, mask: impl Into<SharedString>) -> Self {
        self.mask = Some(mask.into());
        self
    }

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn validator(mut self, validator: impl Fn(SharedString) -> bool + 'static) -> Self {
        self.validator = Some(Box::new(validator));
        self
    }

    pub fn tab_stop(mut self, tab_stop: bool) -> Self {
        self.tab_stop = tab_stop;
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = tab_index;
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

impl StatefulInteractiveElement for TextField {}

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
        let state = window
            .use_keyed_state(self.id, app, |window, app| {
                app.new(|cx| TextFieldState::new(window, cx))
            })
            .update(app, |state, _| state.clone());

        let mut focus_handle = state.focus_handle(app);
        if focus_handle.tab_stop != self.tab_stop {
            focus_handle = focus_handle.tab_stop(self.tab_stop);
        }
        if focus_handle.tab_index != self.tab_index {
            focus_handle = focus_handle.tab_index(self.tab_index);
        }

        state.update(app, |state, _cx| {
            state.set_value(self.value);
            state.on_input = self.on_input;
            state.on_change = self.on_change;
            state.set_placeholder(self.placeholder);
            state.set_placeholder_color(self.placeholder_color);
            state.set_selection_color(self.selection_color);
            state.set_masked(self.masked);
            state.set_mask(self.mask);
            state.max_length = self.max_length;
            state.validator = self.validator;
        });

        self.base
            .when(!self.disabled, |this| {
                this.key_context(CONTEXT)
                    .track_focus(&focus_handle)
                    .on_action(window.listener_for(&state, TextFieldState::backspace))
                    .on_action(window.listener_for(&state, TextFieldState::delete))
                    .on_action(window.listener_for(&state, TextFieldState::left))
                    .on_action(window.listener_for(&state, TextFieldState::right))
                    .on_action(window.listener_for(&state, TextFieldState::select_left))
                    .on_action(window.listener_for(&state, TextFieldState::select_right))
                    .on_action(window.listener_for(&state, TextFieldState::select_all))
                    .on_action(window.listener_for(&state, TextFieldState::home))
                    .on_action(window.listener_for(&state, TextFieldState::end))
                    .on_action(window.listener_for(&state, TextFieldState::show_character_palette))
                    .on_action(window.listener_for(&state, TextFieldState::paste))
                    .on_action(window.listener_for(&state, TextFieldState::cut))
                    .on_action(window.listener_for(&state, TextFieldState::copy))
                    .on_action(window.listener_for(&state, TextFieldState::delete_word_left))
                    .on_action(window.listener_for(&state, TextFieldState::delete_word_right))
                    .on_action(window.listener_for(&state, TextFieldState::delete_to_beginning))
                    .on_action(window.listener_for(&state, TextFieldState::delete_to_end))
                    .on_action(window.listener_for(&state, TextFieldState::word_left))
                    .on_action(window.listener_for(&state, TextFieldState::word_right))
                    .on_action(window.listener_for(&state, TextFieldState::select_word_left))
                    .on_action(window.listener_for(&state, TextFieldState::select_word_right))
                    .on_action(window.listener_for(&state, TextFieldState::select_to_home))
                    .on_action(window.listener_for(&state, TextFieldState::select_to_end))
                    .on_action(window.listener_for(&state, TextFieldState::undo))
                    .on_action(window.listener_for(&state, TextFieldState::redo))
                    .on_action(window.listener_for(&state, TextFieldState::enter))
                    .on_mouse_down(
                        MouseButton::Left,
                        window.listener_for(&state, TextFieldState::on_mouse_down),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        window.listener_for(&state, TextFieldState::on_mouse_up),
                    )
                    .on_mouse_up_out(
                        MouseButton::Left,
                        window.listener_for(&state, TextFieldState::on_mouse_up),
                    )
                    .on_mouse_move(window.listener_for(&state, TextFieldState::on_mouse_move))
            })
            .on_scroll_wheel(window.listener_for(&state, TextFieldState::on_scroll_wheel))
            .child(state.clone())
    }
}

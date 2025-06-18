use crate::{
    components::input::state::{CONTEXT, InputState},
    primitives::h_flex_center,
};
use gpui::{prelude::FluentBuilder, *};

#[derive(IntoElement)]
pub struct TextInput {
    base: Div,
    state: Entity<InputState>,
    leading: Option<AnyElement>,
}

impl TextInput {
    pub fn new(state: Entity<InputState>) -> Self {
        Self {
            base: h_flex_center().cursor(CursorStyle::IBeam),
            state,
            leading: None,
        }
    }

    pub fn set_placeholder_color(self, color: impl Into<Hsla>, cx: &mut impl AppContext) -> Self {
        self.state.update(cx, |this, cx| {
            this.placeholder_color = color.into();
            cx.notify();
        });
        self
    }

    pub fn leading(mut self, element: impl IntoElement) -> Self {
        self.leading = Some(element.into_any_element());
        self
    }
}

impl Styled for TextInput {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for TextInput {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for TextInput {
    fn render(self, window: &mut Window, app: &mut App) -> impl IntoElement {
        self.base
            .key_context(CONTEXT)
            .track_focus(&self.state.focus_handle(app))
            .on_action(window.listener_for(&self.state, InputState::backspace))
            .on_action(window.listener_for(&self.state, InputState::delete))
            .on_action(window.listener_for(&self.state, InputState::left))
            .on_action(window.listener_for(&self.state, InputState::right))
            .on_action(window.listener_for(&self.state, InputState::select_left))
            .on_action(window.listener_for(&self.state, InputState::select_right))
            .on_action(window.listener_for(&self.state, InputState::select_all))
            .on_action(window.listener_for(&self.state, InputState::home))
            .on_action(window.listener_for(&self.state, InputState::end))
            .on_action(window.listener_for(&self.state, InputState::show_character_palette))
            .on_action(window.listener_for(&self.state, InputState::paste))
            .on_action(window.listener_for(&self.state, InputState::cut))
            .on_action(window.listener_for(&self.state, InputState::copy))
            .on_action(window.listener_for(&self.state, InputState::delete_word_left))
            .on_action(window.listener_for(&self.state, InputState::delete_word_right))
            .on_action(window.listener_for(&self.state, InputState::delete_to_beginning))
            .on_action(window.listener_for(&self.state, InputState::delete_to_end))
            .on_action(window.listener_for(&self.state, InputState::word_left))
            .on_action(window.listener_for(&self.state, InputState::word_right))
            .on_action(window.listener_for(&self.state, InputState::select_word_left))
            .on_action(window.listener_for(&self.state, InputState::select_word_right))
            .on_action(window.listener_for(&self.state, InputState::select_to_home))
            .on_action(window.listener_for(&self.state, InputState::select_to_end))
            .on_mouse_down(
                MouseButton::Left,
                window.listener_for(&self.state, InputState::on_mouse_down),
            )
            .on_mouse_up(
                MouseButton::Left,
                window.listener_for(&self.state, InputState::on_mouse_up),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                window.listener_for(&self.state, InputState::on_mouse_up),
            )
            .on_mouse_move(window.listener_for(&self.state, InputState::on_mouse_move))
            .on_scroll_wheel(window.listener_for(&self.state, InputState::on_scroll_wheel))
            .when_some(self.leading, |this, leading| this.child(leading))
            .child(self.state.clone())
    }
}

use gpui::*;
use lapislazuli::{
    Disableable,
    components::{
        button,
        input::{InputState, init},
        progress, separator, text_input,
    },
    primitives::{a, h_flex_center, span, v_flex},
};

struct Showcase {
    progress_value: f32,
    text_state: Entity<InputState>,
    focus_handle: FocusHandle,
}

impl Focusable for Showcase {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Showcase {
    fn new(window: &mut Window, app: &mut App) -> Entity<Self> {
        init(app);

        let text_state = app.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Type here...")
                .placeholder_color(rgb(0x726f76))
        });

        app.new(|cx| Self {
            text_state,
            focus_handle: cx.focus_handle(),
            progress_value: 50.0,
        })
    }

    fn increment_progress<T>(&mut self, _event: &T, _window: &mut Window, cx: &mut Context<Self>) {
        self.progress_value += 2.0;
        cx.notify();
    }

    fn decrement_progress<T>(&mut self, _event: &T, _window: &mut Window, cx: &mut Context<Self>) {
        self.progress_value -= 2.0;
        cx.notify();
    }
}

impl Render for Showcase {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .font_family(".SystemUIFont")
            .track_focus(&self.focus_handle(cx))
            .p(rems(2.0))
            .gap(rems(1.0))
            .child(
                button("more")
                    .bg(rgb(0xFFFFFF))
                    .w(rems(10.0))
                    .h(rems(2.0))
                    .child(span("More").text_color(rgb(0x0000FF)))
                    .on_click(cx.listener(Self::increment_progress))
                    .when_disabled(|this| this.bg(rgb(0xCCCCCC))),
            )
            .child(
                button("much more")
                    .bg(rgb(0xFFFFFF))
                    .w(rems(10.0))
                    .h(rems(2.0))
                    .disabled(true)
                    .child(span("Much more (disabled)").text_color(rgb(0xFF0000)))
                    .on_click(|_event, _window, _app| {
                        println!("Button 2 clicked!");
                    })
                    .when_disabled(|this| this.bg(rgb(0xCCCCCC))),
            )
            .child(
                button("less")
                    .bg(rgb(0xFFFFFF))
                    .w(rems(10.0))
                    .h(rems(2.0))
                    .child(span("Less").text_color(rgb(0x0000FF)))
                    .on_click(cx.listener(Self::decrement_progress))
                    .when_disabled(|this| this.bg(rgb(0xCCCCCC))),
            )
            .child(separator().bg(rgb(0xbcbcbc)).when_horizontal_else(
                |this| this.w(rems(10.)).h(px(1.)),
                |this| this.h(rems(10.)).w(px(1.)),
            ))
            .child(
                progress()
                    .bg(rgb(0xEEEEEE))
                    .w(rems(10.0))
                    .p(rems(0.5))
                    .value(self.progress_value)
                    .child(span("Progress").text_color(rgb(0x000000)))
                    .track(|track, _| {
                        track
                            .bg(rgb(0xCCCCCC))
                            .h(rems(1.0))
                            .w_full()
                            .fill(|fill, progress| {
                                fill.bg(rgb(0x00FF00)).h(rems(1.0)).w(relative(progress))
                            })
                    }),
            )
            .child(
                a("https://github.com/J0R6IT0/lapislazuli")
                    .child("Source Code!!")
                    .cursor_pointer()
                    .text_color(rgb(0x0000FF)),
            )
            .child(
                text_input(self.text_state.clone())
                    .border_color(rgb(0x373737))
                    .text_color(rgb(0xbab6be))
                    .h(px(48.))
                    .pr(px(16.))
                    .pl(px(8.))
                    .border_1()
                    .max_w(rems(15.))
                    .rounded_md()
                    .gap(px(8.))
                    .left_click_clear(true)
                    .leading(
                        h_flex_center()
                            .h(px(32.))
                            .w(px(32.))
                            .bg(rgb(0x373737))
                            .line_height(px(16.))
                            .child(span("Ee").text_color(rgb(0xFFFFFF))),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), Showcase::new)
            .unwrap();
    });
}

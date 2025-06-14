use gpui::*;
use lapislazuli::{
    Disableable,
    components::{button, progress, separator},
    primitives::{a, span, v_flex},
};

struct Showcase {
    progress_value: f32,
}

impl Showcase {
    fn new(_window: &mut Window, app: &mut App) -> Entity<Self> {
        app.new(|cx| Self {
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
                                fill.bg(rgb(0x00FF00))
                                    .h(rems(1.0))
                                    .w(DefiniteLength::Fraction(progress))
                            })
                    }),
            )
            .child(
                a("https://github.com/J0R6IT0/lapislazuli")
                    .child("Source Code!!")
                    .cursor_pointer()
                    .text_color(rgb(0x0000FF)),
            )
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), Showcase::new)
            .unwrap();
    });
}

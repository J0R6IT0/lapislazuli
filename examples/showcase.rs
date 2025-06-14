use gpui::*;
use lapislazuli::{
    Disableable,
    components::button,
    primitives::{span, v_flex},
};

struct Showcase {}

impl Showcase {
    fn new(_window: &mut Window, app: &mut App) -> Entity<Self> {
        app.new(|cx| Self {})
    }
}

impl Render for Showcase {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap(rems(1.0))
            .child(
                button("button1")
                    .bg(rgb(0xFFFFFF))
                    .w(rems(10.0))
                    .h(rems(2.0))
                    .child(span("Button 1").text_color(rgb(0x0000FF)))
                    .on_click(|_event, _window, _app| {
                        println!("Button 1 clicked!");
                    }),
            )
            .child(
                button("button2")
                    .bg(rgb(0xFFFFFF))
                    .w(rems(10.0))
                    .h(rems(2.0))
                    .disabled(true)
                    .child(span("Button 2 (disabled)").text_color(rgb(0xFF0000)))
                    .on_click(|_event, _window, _app| {
                        println!("Button 2 clicked!");
                    }),
            )
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), Showcase::new)
            .unwrap();
    });
}

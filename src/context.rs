use crate::primitives::init;
use gpui::{
    AnyView, App, AppContext, Context, Entity, InteractiveElement, IntoElement, KeyBinding,
    ParentElement, Render, Styled, Window, actions, div,
};

actions!(global, [Tab, TabPrev]);

pub struct LapislazuliProvider {
    view: AnyView,
}

impl LapislazuliProvider {
    pub fn new(view: impl Into<AnyView>, _window: &mut Window, app: &mut App) -> Entity<Self> {
        init(app);
        app.bind_keys([
            KeyBinding::new("tab", Tab, None),
            KeyBinding::new("shift-tab", TabPrev, None),
        ]);

        let view = view.into();
        app.new(|_cx| LapislazuliProvider { view })
    }

    fn on_tab(&mut self, _: &Tab, window: &mut Window, _: &mut Context<Self>) {
        window.focus_next();
    }

    fn on_tab_prev(&mut self, _: &TabPrev, window: &mut Window, _: &mut Context<Self>) {
        window.focus_prev();
    }
}

impl Render for LapislazuliProvider {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(self.view.clone())
            .id("lapislazuli-provider")
            .on_action(cx.listener(Self::on_tab))
            .on_action(cx.listener(Self::on_tab_prev))
    }
}

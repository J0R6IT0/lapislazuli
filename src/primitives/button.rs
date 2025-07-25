use crate::{AutoFocusable, Disableable};
use gpui::{
    AnyElement, App, ClickEvent, Div, ElementId, InteractiveElement, Interactivity, IntoElement,
    ParentElement, RenderOnce, Stateful, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, prelude::FluentBuilder,
};
use smallvec::SmallVec;
use std::rc::Rc;

pub fn button(id: impl Into<ElementId>) -> Button {
    let id = id.into();
    Button {
        id: id.clone(),
        base: div().id(id),
        disabled: false,
        children: SmallVec::new(),
        on_click: None,
        auto_focus: false,
        tab_index: 0,
        tab_stop: true,
    }
}

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    base: Stateful<Div>,
    disabled: bool,
    children: SmallVec<[AnyElement; 2]>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    auto_focus: bool,
    tab_index: isize,
    tab_stop: bool,
}

impl Button {
    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
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

impl Disableable for Button {
    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl AutoFocusable for Button {
    fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Button {}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, app: &mut App) -> impl IntoElement {
        let mut focus_handle = window
            .use_keyed_state(self.id, app, |window, app| {
                let focus_handle = app.focus_handle();
                if self.auto_focus {
                    focus_handle.focus(window);
                }
                focus_handle
            })
            .read(app)
            .clone();

        if focus_handle.tab_stop != self.tab_stop {
            focus_handle = focus_handle.tab_stop(self.tab_stop);
        }

        if focus_handle.tab_index != self.tab_index {
            focus_handle = focus_handle.tab_index(self.tab_index);
        }

        self.base
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.track_focus(&focus_handle)
                        .map(|this| {
                            let on_click = on_click.clone();
                            this.on_key_up(move |event, window, app| {
                                if event.keystroke.key == "space" {
                                    (on_click)(&ClickEvent::default(), window, app);
                                }
                            })
                        })
                        .map(|this| {
                            let on_click = on_click.clone();
                            this.on_key_down(move |event, window, app| {
                                if event.keystroke.key == "enter" {
                                    (on_click)(&ClickEvent::default(), window, app);
                                }
                            })
                        })
                        .on_click(move |event, window, app| (on_click)(event, window, app))
                },
            )
            .children(self.children)
    }
}

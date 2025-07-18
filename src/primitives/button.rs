use crate::Disableable;
use gpui::{prelude::FluentBuilder, *};
use smallvec::SmallVec;

pub fn button(id: impl Into<ElementId>) -> Button {
    Button {
        base: div().id(id),
        disabled: false,
        children: SmallVec::new(),
        on_click: None,
        stop_propagation: true,
    }
}

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Button {
    base: Stateful<Div>,
    disabled: bool,
    children: SmallVec<[AnyElement; 2]>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    stop_propagation: bool,
}

impl Button {
    pub fn stop_propagation(mut self, stop: bool) -> Self {
        self.stop_propagation = stop;
        self
    }

    pub fn on_click<F>(mut self, on_click: F) -> Self
    where
        F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Box::new(on_click));
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
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    let stop_propagation = self.stop_propagation;
                    this.on_mouse_down(MouseButton::Left, move |_, window, app| {
                        window.prevent_default();
                        if stop_propagation {
                            app.stop_propagation();
                        }
                    })
                    .on_click(on_click)
                },
            )
            .children(self.children)
    }
}

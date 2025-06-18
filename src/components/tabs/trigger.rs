use crate::{Disableable, Selectable};
use gpui::{prelude::FluentBuilder, *};
use smallvec::SmallVec;
use std::rc::Rc;

#[derive(IntoElement)]
pub struct TabsTrigger {
    base: Div,
    id: ElementId,
    children: SmallVec<[AnyElement; 1]>,
    pub(super) disabled: bool,
    pub(super) selected: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    when_selected_handler: Option<Box<dyn FnOnce(Self) -> Self>>,
}

impl TabsTrigger {
    pub fn new() -> Self {
        Self {
            base: div(),
            children: SmallVec::new(),
            disabled: false,
            selected: false,
            on_click: None,
            id: 0.into(),
            when_selected_handler: None,
        }
    }

    pub(super) fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub(super) fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    pub fn when_selected(mut self, handler: impl FnOnce(Self) -> Self + 'static) -> Self {
        self.when_selected_handler = Some(Box::new(handler));
        self
    }
}

impl ParentElement for TabsTrigger {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(children);
    }
}

impl Styled for TabsTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl Disableable for TabsTrigger {
    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Selectable for TabsTrigger {
    fn element_id(&self) -> &ElementId {
        &self.id
    }

    fn is_selected(&self) -> bool {
        self.selected
    }

    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl InteractiveElement for TabsTrigger {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for TabsTrigger {}

impl RenderOnce for TabsTrigger {
    fn render(mut self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        if self.selected {
            if let Some(handler) = self.when_selected_handler.take() {
                self = handler(self);
            }
        }

        self.base
            .id(self.id)
            .when(!self.disabled, |this| {
                this.when_some(self.on_click, |this, on_click| {
                    this.on_click(move |event, window, cx| on_click(event, window, cx))
                })
            })
            .children(self.children)
    }
}

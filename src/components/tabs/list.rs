use crate::{Selectable, components::tabs::TabsTrigger, primitives::h_flex};
use gpui::{prelude::FluentBuilder, *};
use smallvec::SmallVec;
use std::rc::Rc;

#[derive(IntoElement)]
pub struct TabsList {
    base: Div,
    triggers: SmallVec<[TabsTrigger; 1]>,
    pub(super) selected_index: Option<usize>,
    pub(super) on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App)>>,
}

impl TabsList {
    pub(super) fn new() -> Self {
        Self {
            base: h_flex(),
            triggers: SmallVec::new(),
            selected_index: None,
            on_change: None,
        }
    }

    pub fn triggers(mut self, triggers: impl IntoIterator<Item = impl Into<TabsTrigger>>) -> Self {
        self.triggers.extend(triggers.into_iter().map(Into::into));
        self
    }

    pub fn trigger(mut self, trigger: impl Into<TabsTrigger>) -> Self {
        self.triggers.push(trigger.into());
        self
    }
}

impl Styled for TabsList {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for TabsList {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base
            .id("tabs-list")
            .children(self.triggers.into_iter().enumerate().map(|(ix, trigger)| {
                trigger
                    .id(ix)
                    .when_some(self.selected_index, |this, selected_ix| {
                        this.selected(selected_ix == ix)
                    })
                    .when_some(self.on_change.clone(), move |this, on_click| {
                        this.on_click(move |_, window, cx| on_click(&ix, window, cx))
                    })
            }))
    }
}

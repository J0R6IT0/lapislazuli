use crate::components::progress::context::ProgressContext;
use crate::traits::ParentElementWithContext;
use gpui::*;
use smallvec::SmallVec;
use std::rc::Rc;

mod context;
mod fill;
mod track;

pub use fill::*;
pub use track::*;

#[derive(IntoElement)]
pub struct Progress {
    base: Div,
    children: SmallVec<[AnyElement; 2]>,
    state: ProgressContext,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Progress {
    pub fn new() -> Self {
        Self {
            base: div().relative(),

            children: SmallVec::new(),
            state: ProgressContext {
                value: 0.0,
                min_value: 0.0,
                max_value: 100.0,
                value_label: None,
            },
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.state.value = value;
        self
    }

    pub fn min_value(mut self, min_value: f32) -> Self {
        self.state.min_value = min_value;
        self
    }

    pub fn max_value(mut self, max_value: f32) -> Self {
        self.state.max_value = max_value;
        self
    }

    pub fn value_label<F>(mut self, label_fn: F) -> Self
    where
        F: Fn(&ProgressContext) -> String + 'static,
    {
        self.state.value_label = Some(Rc::new(Box::new(label_fn)));
        self
    }
}

impl ParentElement for Progress {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Progress {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElementWithContext<ProgressContext> for Progress {
    fn get_context(&self) -> ProgressContext {
        self.state.clone()
    }
}

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}

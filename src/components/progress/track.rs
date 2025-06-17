use gpui::*;
use smallvec::SmallVec;

#[derive(IntoElement)]
pub struct ProgressTrack {
    base: Div,
    children: SmallVec<[AnyElement; 2]>,
}

impl ProgressTrack {
    pub fn new() -> Self {
        Self {
            base: div().relative(),
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for ProgressTrack {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for ProgressTrack {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ProgressTrack {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}

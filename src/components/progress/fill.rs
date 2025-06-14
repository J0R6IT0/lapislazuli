use gpui::*;

#[derive(IntoElement)]
pub struct ProgressFill {
    base: Div,
}

impl ProgressFill {
    pub(super) fn new() -> Self {
        Self { base: div() }
    }
}

impl Styled for ProgressFill {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ProgressFill {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base
    }
}

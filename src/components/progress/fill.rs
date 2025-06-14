use gpui::*;

#[derive(IntoElement)]
pub struct ProgressFill {
    base: Div,
    pub(super) value: f32,
    pub(super) min_value: f32,
    pub(super) max_value: f32,
}

impl ProgressFill {
    pub(super) fn new(value: f32, min_value: f32, max_value: f32) -> Self {
        Self {
            base: div(),
            value,
            min_value,
            max_value,
        }
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

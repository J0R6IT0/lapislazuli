use crate::components::progress::ProgressFill;
use gpui::{prelude::FluentBuilder, *};

#[derive(IntoElement)]
pub struct ProgressTrack {
    base: Div,
    fill: Option<ProgressFill>,
    pub(super) value: f32,
    pub(super) min_value: f32,
    pub(super) max_value: f32,
}

impl ProgressTrack {
    pub(super) fn new(value: f32, min_value: f32, max_value: f32) -> Self {
        Self {
            base: div(),
            fill: None,
            value,
            min_value,
            max_value,
        }
    }

    pub fn fill<F>(mut self, builder: F) -> Self
    where
        F: Fn(ProgressFill, f32) -> ProgressFill,
    {
        let percentage = if self.max_value > self.min_value {
            ((self.value - self.min_value) / (self.max_value - self.min_value)).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let fill = builder(
            ProgressFill::new(self.value, self.min_value, self.max_value),
            percentage,
        );
        self.fill = Some(fill);
        self
    }
}

impl Styled for ProgressTrack {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ProgressTrack {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base
            .when_some(self.fill, |this, fill| this.child(fill))
    }
}

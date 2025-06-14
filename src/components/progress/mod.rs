use gpui::*;

mod fill;
mod track;

pub use fill::*;
pub use track::*;

#[derive(IntoElement)]
pub struct Progress {
    base: Div,
    value: f32,
    min_value: f32,
    max_value: f32,
    children: Vec<AnyElement>,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Progress {
    pub fn new() -> Self {
        Self {
            base: div(),
            value: 0.0,
            min_value: 0.0,
            max_value: 100.0,
            children: Vec::new(),
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn min_value(mut self, min_value: f32) -> Self {
        self.min_value = min_value;
        self
    }

    pub fn max_value(mut self, max_value: f32) -> Self {
        self.max_value = max_value;
        self
    }

    pub fn track<F>(mut self, builder: F) -> Self
    where
        F: Fn(ProgressTrack, f32) -> ProgressTrack,
    {
        let percentage = if self.max_value > self.min_value {
            ((self.value - self.min_value) / (self.max_value - self.min_value)).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let track = builder(
            ProgressTrack::new(self.value, self.min_value, self.max_value),
            percentage,
        );
        self.children.push(track.into_any_element());
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

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}

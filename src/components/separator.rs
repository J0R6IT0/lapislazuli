use gpui::*;
use smallvec::SmallVec;

#[derive(IntoElement)]
pub struct Separator {
    base: Div,
    vertical: bool,
    children: SmallVec<[AnyElement; 1]>,
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

impl Separator {
    pub fn new() -> Self {
        Self {
            base: div(),
            vertical: false,
            children: SmallVec::new(),
        }
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.vertical = false;
        self
    }

    pub fn when_vertical(self, handler: impl FnOnce(Self) -> Self) -> Self {
        if self.vertical { handler(self) } else { self }
    }

    pub fn when_vertical_else(
        self,
        handler: impl FnOnce(Self) -> Self,
        else_: impl FnOnce(Self) -> Self,
    ) -> Self {
        if self.vertical {
            handler(self)
        } else {
            else_(self)
        }
    }

    pub fn when_horizontal(self, handler: impl FnOnce(Self) -> Self) -> Self {
        if !self.vertical { handler(self) } else { self }
    }

    pub fn when_horizontal_else(
        self,
        handler: impl FnOnce(Self) -> Self,
        else_: impl FnOnce(Self) -> Self,
    ) -> Self {
        if !self.vertical {
            handler(self)
        } else {
            else_(self)
        }
    }
}

impl Styled for Separator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for Separator {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}

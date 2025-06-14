use gpui::*;

#[derive(IntoElement)]
pub struct Separator {
    base: Div,
    vertical: bool,
}

impl Separator {
    pub fn new() -> Self {
        Self {
            base: div(),
            vertical: false,
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

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base
    }
}

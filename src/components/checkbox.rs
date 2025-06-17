use crate::{Disableable, primitives::h_flex_center};
use gpui::{prelude::FluentBuilder, *};

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Checkbox {
    base: Stateful<Div>,
    disabled: bool,
    checked: bool,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    indicator: AnyElement,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: h_flex_center().id(id),
            disabled: false,
            checked: false,
            indicator: div().into_any_element(),
            on_change: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn indicator(mut self, indicator: impl IntoElement) -> Self {
        self.indicator = indicator.into_any_element();
        self
    }

    pub fn when_checked(self, handler: impl FnOnce(Self) -> Self) -> Self {
        if self.checked { handler(self) } else { self }
    }

    pub fn on_change<F>(mut self, on_change: F) -> Self
    where
        F: Fn(&bool, &mut Window, &mut App) + 'static,
    {
        self.on_change = Some(Box::new(on_change));
        self
    }
}

impl StatefulInteractiveElement for Checkbox {}

impl Disableable for Checkbox {
    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for Checkbox {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Checkbox {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base
            .when_some(
                self.on_change.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |_, window, cx| {
                        cx.stop_propagation();
                        let checked = !self.checked;
                        on_click(&checked, window, cx);
                    })
                },
            )
            .when(self.checked, |this| this.child(self.indicator))
    }
}

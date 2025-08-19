use std::rc::Rc;

use crate::{AutoFocusable, Disableable};
use gpui::{
    AnyElement, App, Div, ElementId, FocusHandle, Focusable, InteractiveElement, Interactivity,
    IntoElement, ParentElement, RenderOnce, Stateful, StatefulInteractiveElement, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder,
};

pub fn checkbox(id: impl Into<ElementId>) -> Checkbox {
    let id = id.into();
    Checkbox {
        id: id.clone(),
        base: div().id(id),
        disabled: false,
        checked: None,
        indeterminate: false,
        on_change: None,
        checked_indicator: div().into_any_element(),
        indeterminate_indicator: div().into_any_element(),
        auto_focus: false,
        tab_index: 0,
        tab_stop: true,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChangeEvent {
    pub checked: bool,
}

#[derive(Debug)]
struct CheckboxState {
    checked: bool,
    indeterminate: bool,
    focus_handle: FocusHandle,
}

impl CheckboxState {
    fn new(app: &mut App) -> Self {
        let focus_handle = app.focus_handle();
        Self {
            checked: false,
            indeterminate: false,
            focus_handle,
        }
    }
}

impl Focusable for CheckboxState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    base: Stateful<Div>,
    disabled: bool,
    checked: Option<bool>,
    indeterminate: bool,
    on_change: Option<Rc<dyn Fn(&ChangeEvent, &mut Window, &mut App) + 'static>>,
    checked_indicator: AnyElement,
    indeterminate_indicator: AnyElement,
    auto_focus: bool,
    tab_index: isize,
    tab_stop: bool,
}

impl Checkbox {
    pub fn on_change(
        mut self,
        on_change: impl Fn(&ChangeEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn checked_indicator(mut self, indicator: impl IntoElement) -> Self {
        self.checked_indicator = indicator.into_any_element();
        self
    }

    pub fn indeterminate_indicator(mut self, indicator: impl IntoElement) -> Self {
        self.indeterminate_indicator = indicator.into_any_element();
        self
    }
}

impl AutoFocusable for Checkbox {
    fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }
}

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

impl StatefulInteractiveElement for Checkbox {}

impl RenderOnce for Checkbox {
    fn render(self, window: &mut Window, app: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id, app, |_, app| CheckboxState::new(app));

        state.update(app, |state, _| {
            if let Some(checked) = self.checked {
                state.checked = checked;
            };
            state.indeterminate = self.indeterminate;
        });

        let state_read = state.read(app);

        let indeterminate = state_read.indeterminate;
        let checked = state_read.checked;

        let mut focus_handle = state_read.focus_handle(app);
        if focus_handle.tab_stop != self.tab_stop {
            focus_handle = focus_handle.tab_stop(self.tab_stop);
        }
        if focus_handle.tab_index != self.tab_index {
            focus_handle = focus_handle.tab_index(self.tab_index);
        }

        let mut checkbox = self.base;

        if indeterminate {
            checkbox = checkbox.child(self.indeterminate_indicator);
        } else if checked {
            checkbox = checkbox.child(self.checked_indicator);
        }

        checkbox.when(!self.disabled, |this| {
            this.track_focus(&focus_handle)
                .map(|this| {
                    let state = state.clone();
                    this.on_key_up(move |event, _, app| {
                        if event.keystroke.key == "space" {
                            state.update(app, |state, cx| {
                                state.checked = !state.checked;
                                cx.notify();
                            });
                        }
                    })
                })
                .on_click(move |_, _, app| {
                    state.update(app, |state, cx| {
                        state.checked = !state.checked;
                        cx.notify();
                    });
                })
                .when_some(self.on_change, |this, on_change| {
                    this.map(|this| {
                        let on_change = on_change.clone();
                        this.on_key_up(move |event, window, app| {
                            if event.keystroke.key == "space" {
                                (on_change)(&ChangeEvent { checked: !checked }, window, app);
                            }
                        })
                    })
                    .on_click(move |_, window, app| {
                        (on_change)(&ChangeEvent { checked: !checked }, window, app)
                    })
                })
        })
    }
}

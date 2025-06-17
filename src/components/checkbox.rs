use crate::{Disableable, primitives::h_flex_center};
use gpui::{prelude::FluentBuilder, *};

/// A checkbox component that allows users to toggle between checked and unchecked states.
///
/// The checkbox provides a boolean input control that can be styled and customized.
/// It supports disabled states, custom indicators, and change callbacks.
///
/// # Examples
///
/// Basic usage:
/// ```rust
/// Checkbox::new("my-checkbox")
///     .checked(true)
///     .on_change(|checked, _window, _cx| {
///         println!("Checkbox is now: {}", checked);
///     })
/// ```
///
/// With custom styling and indicator:
/// ```rust
/// Checkbox::new("styled-checkbox")
///     .checked(false)
///     .indicator(span("✓").text_color(rgb(0xffffff)))
///     .bg(rgb(0x3b82f6))
///     .when_checked(|this| this.bg(rgb(0x1d4ed8)))
///     .rounded_md()
///     .size(rems(1.5))
/// ```
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
    /// Creates a new checkbox with the specified ID.
    ///
    /// The checkbox starts in an unchecked, enabled state with a default empty indicator.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for the checkbox element
    ///
    /// # Examples
    ///
    /// ```rust
    /// let checkbox = Checkbox::new("user-agreement");
    /// ```
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: h_flex_center().id(id),
            disabled: false,
            checked: false,
            indicator: div().into_any_element(),
            on_change: None,
        }
    }

    /// Sets the checked state of the checkbox.
    ///
    /// # Arguments
    ///
    /// * `checked` - Whether the checkbox should be checked (`true`) or unchecked (`false`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let checkbox = Checkbox::new("my-checkbox").checked(true);
    /// ```
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Sets a custom indicator element that appears when the checkbox is checked.
    ///
    /// The indicator can be any element, such as text, icons, or custom graphics.
    /// It will only be visible when the checkbox is in the checked state.
    ///
    /// # Arguments
    ///
    /// * `indicator` - An element to display when the checkbox is checked
    ///
    /// # Examples
    ///
    /// ```rust
    /// let checkbox = Checkbox::new("my-checkbox")
    ///     .indicator(span("✓").text_color(rgb(0xffffff)));
    /// ```
    pub fn indicator(mut self, indicator: impl IntoElement) -> Self {
        self.indicator = indicator.into_any_element();
        self
    }

    /// Conditionally applies styling or modifications when the checkbox is checked.
    ///
    /// This method allows you to apply different styles or properties based on the
    /// checked state. The handler function is only called if the checkbox is currently checked.
    ///
    /// # Arguments
    ///
    /// * `handler` - A function that takes and returns the checkbox, applying modifications
    ///
    /// # Examples
    ///
    /// ```rust
    /// let checkbox = Checkbox::new("my-checkbox")
    ///     .checked(true)
    ///     .when_checked(|this| this.bg(rgb(0x3b82f6)).border_color(rgb(0x1d4ed8)));
    /// ```
    pub fn when_checked(self, handler: impl FnOnce(Self) -> Self) -> Self {
        if self.checked { handler(self) } else { self }
    }

    /// Sets a callback function that is called when the checkbox state changes.
    ///
    /// The callback receives the new checked state as a boolean value. This callback
    /// will not be triggered if the checkbox is disabled.
    ///
    /// # Arguments
    ///
    /// * `on_change` - A function that handles the checkbox state change
    ///
    /// # Examples
    ///
    /// ```rust
    /// let checkbox = Checkbox::new("my-checkbox")
    ///     .on_change(|checked, _window, _cx| {
    ///         if *checked {
    ///             println!("Checkbox was checked!");
    ///         } else {
    ///             println!("Checkbox was unchecked!");
    ///         }
    ///     });
    /// ```
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

use crate::{Disableable, primitives::h_flex};
use gpui::{prelude::FluentBuilder, *};

/// A toggle switch component that allows users to switch between on/off states.
///
/// The switch provides a boolean input control similar to a checkbox but with a different
/// visual appearance, typically showing a sliding thumb that moves between positions.
/// It supports disabled states, custom thumb styling, and change callbacks.
///
/// # Examples
///
/// Basic usage:
/// ```rust
/// Switch::new("my-switch")
///     .checked(true)
///     .on_change(|checked, _window, _cx| {
///         println!("Switch is now: {}", checked);
///     })
/// ```
///
/// With custom styling and thumb:
/// ```rust
/// Switch::new("styled-switch")
///     .checked(false)
///     .bg(rgb(0xe5e7eb))
///     .when_checked(|this| this.bg(rgb(0x3b82f6)))
///     .thumb(|thumb| thumb.bg(rgb(0xffffff)).rounded_full().size(px(20)))
///     .rounded_full()
///     .p(px(2))
///     .with_animation(("checkbox", self.disabled as u32), Animation::new(Duration::from_millis(100)), move |this, delta| {
///         this.thumb(|thumb| {
///             if disabled {
///                 thumb.left(px(delta * LEFT_OFFSET))
///             } else {
///                 thumb.left(px(LEFT_OFFSET - delta * LEFT_OFFSET))
///             }
///         })
///     })
/// ```

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Switch {
    base: Stateful<Div>,
    disabled: bool,
    checked: bool,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    thumb: SwitchThumb,
    when_checked_handler: Option<Box<dyn FnOnce(Self) -> Self>>,
}

impl Switch {
    /// Creates a new switch with the specified ID.
    ///
    /// The switch starts in an unchecked, enabled state with a default thumb.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for the switch element
    ///
    /// # Examples
    ///
    /// ```rust
    /// let switch = Switch::new("user-notifications");
    /// ```
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: h_flex().id(id),
            disabled: false,
            checked: false,
            on_change: None,
            thumb: SwitchThumb::new(),
            when_checked_handler: None,
        }
    }

    /// Sets the checked state of the switch.
    ///
    /// # Arguments
    ///
    /// * `checked` - Whether the switch should be on (`true`) or off (`false`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let switch = Switch::new("my-switch").checked(true);
    /// ```
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Customizes the switch thumb with the provided handler function.
    ///
    /// The thumb is the movable part of the switch that slides between positions.
    /// This method allows you to style the thumb independently of the switch track.
    ///
    /// # Arguments
    ///
    /// * `handler` - A function that takes and returns a SwitchThumb, applying modifications
    ///
    /// # Examples
    ///
    /// ```rust
    /// let switch = Switch::new("my-switch")
    ///     .thumb(|thumb| thumb.bg(rgb(0xffffff)).rounded_full().size(px(18)));
    /// ```
    pub fn thumb(mut self, handler: impl FnOnce(SwitchThumb) -> SwitchThumb) -> Self {
        self.thumb = handler(self.thumb);
        self
    }

    /// Conditionally applies styling or modifications when the switch is checked.
    ///
    /// This method allows you to apply different styles or properties based on the
    /// checked state.
    ///
    /// # Arguments
    ///
    /// * `handler` - A function that takes and returns the switch, applying modifications
    ///
    /// # Examples
    ///
    /// ```rust
    /// let switch = Switch::new("my-switch")
    ///     .checked(true)
    ///     .when_checked(|this| this.bg(rgb(0x10b981)).border_color(rgb(0x059669)));
    /// ```
    pub fn when_checked(mut self, handler: impl FnOnce(Self) -> Self + 'static) -> Self {
        self.when_checked_handler = Some(Box::new(handler));
        self
    }

    /// Sets a callback function that is called when the switch state changes.
    ///
    /// The callback receives the new checked state as a boolean value. This callback
    /// will not be triggered if the switch is disabled.
    ///
    /// # Arguments
    ///
    /// * `on_change` - A function that handles the switch state change
    ///
    /// # Examples
    ///
    /// ```rust
    /// let switch = Switch::new("my-switch")
    ///     .on_change(|checked, _window, _cx| {
    ///         if *checked {
    ///             println!("Switch was turned on!");
    ///         } else {
    ///             println!("Switch was turned off!");
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

impl StatefulInteractiveElement for Switch {}

impl Disableable for Switch {
    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for Switch {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Switch {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Switch {
    fn render(mut self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        if self.checked {
            if let Some(handler) = self.when_checked_handler.take() {
                self = handler(self);
            }
        }

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
            .child(self.thumb)
    }
}

#[derive(IntoElement)]
pub struct SwitchThumb {
    base: Div,
}

impl SwitchThumb {
    fn new() -> Self {
        Self { base: div() }
    }
}

impl Styled for SwitchThumb {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for SwitchThumb {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl RenderOnce for SwitchThumb {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base
    }
}

use crate::Disableable;
use gpui::{prelude::FluentBuilder, *};
use smallvec::SmallVec;

/// A clickable button component that can contain child elements and handle user interactions.
///
/// The button provides a flexible interactive element that can be styled, disabled, and
/// configured with custom click handlers. It supports event propagation control and can
/// contain any number of child elements.
///
/// # Examples
///
/// Basic usage:
/// ```rust
/// Button::new("my-button")
///     .child(span("Click me"))
///     .on_click(|_event, _window, _cx| {
///         println!("Button clicked!");
///     })
/// ```
///
/// With styling and disabled state:
/// ```rust
/// Button::new("styled-button")
///     .child(span("Submit"))
///     .bg(rgb(0x3b82f6))
///     .hover(|this| this.bg(rgb(0x2563eb)))
///     .disabled(true)
///     .px(rems(1.5))
///     .py(rems(0.75))
///     .rounded_md()
/// ```
#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Button {
    base: Stateful<Div>,
    disabled: bool,
    children: SmallVec<[AnyElement; 2]>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    stop_propagation: bool,
}

impl Button {
    /// Creates a new button with the specified ID.
    ///
    /// The button starts in an enabled state with no children and no click handler.
    /// Event propagation is enabled by default.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for the button element
    ///
    /// # Examples
    ///
    /// ```rust
    /// let button = Button::new("submit-button");
    /// ```
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().id(id),
            disabled: false,
            children: SmallVec::new(),
            on_click: None,
            stop_propagation: true,
        }
    }

    /// Controls whether the button stops event propagation when clicked.
    ///
    /// When set to `true` (default), the button will prevent the click event from
    /// bubbling up to parent elements. When `false`, the event will continue to propagate.
    ///
    /// # Arguments
    ///
    /// * `stop` - Whether to stop event propagation (`true`) or allow it (`false`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let button = Button::new("my-button")
    ///     .stop_propagation(false); // Allow event to bubble up
    /// ```
    pub fn stop_propagation(mut self, stop: bool) -> Self {
        self.stop_propagation = stop;
        self
    }

    /// Sets a callback function that is called when the button is clicked.
    ///
    /// The callback receives the click event and provides access to the window and app context.
    /// This callback will not be triggered if the button is disabled.
    ///
    /// # Arguments
    ///
    /// * `on_click` - A function that handles the button click event
    ///
    /// # Examples
    ///
    /// ```rust
    /// let button = Button::new("my-button")
    ///     .on_click(|event, _window, _cx| {
    ///         println!("Button clicked at position: {:?}", event.position);
    ///     });
    /// ```
    pub fn on_click<F>(mut self, on_click: F) -> Self
    where
        F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Box::new(on_click));
        self
    }
}

impl Disableable for Button {
    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    let stop_propagation = self.stop_propagation;
                    this.on_mouse_down(MouseButton::Left, move |_, window, app| {
                        window.prevent_default();
                        if stop_propagation {
                            app.stop_propagation();
                        }
                    })
                    .on_click(on_click)
                },
            )
            .children(self.children)
    }
}

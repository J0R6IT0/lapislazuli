use gpui::*;

/// A visual separator component that creates horizontal or vertical dividing lines.
///
/// The separator provides a simple way to visually divide content sections with
/// customizable orientation and styling. It defaults to horizontal orientation
/// and can be styled with colors, thickness, and spacing.
///
/// # Examples
///
/// Basic horizontal separator:
/// ```rust
/// Separator::new()
///     .bg(rgb(0xe2e8f0))
///     .h(px(1))
///     .w_full()
/// ```
///
/// Vertical separator with conditional styling:
/// ```rust
/// Separator::new()
///     .vertical()
///     .when_vertical(|this| this.w(px(1)).h_full())
///     .bg(rgb(0xd1d5db))
/// ```
///
/// Responsive separator with different styles:
/// ```rust
/// Separator::new()
///     .when_horizontal_else(
///         |this| this.h(px(2)).w_full().bg(rgb(0x3b82f6)),
///         |this| this.w(px(2)).h_full().bg(rgb(0xef4444))
///     )
/// ```
#[derive(IntoElement)]
pub struct Separator {
    base: Div,
    vertical: bool,
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

impl Separator {
    /// Creates a new horizontal separator.
    ///
    /// The separator starts in horizontal orientation. Use styling methods
    /// to set dimensions, colors, and other visual properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let separator = Separator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            base: div(),
            vertical: false,
        }
    }

    /// Sets the separator to vertical orientation.
    ///
    /// Vertical separators are typically used to divide content horizontally,
    /// creating columns or side-by-side sections.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let separator = Separator::new()
    ///     .vertical()
    ///     .w(px(1))
    ///     .h_full();
    /// ```
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// Sets the separator to horizontal orientation.
    ///
    /// Horizontal separators are typically used to divide content vertically,
    /// creating rows or stacked sections. This is the default orientation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let separator = Separator::new()
    ///     .horizontal()
    ///     .h(px(1))
    ///     .w_full();
    /// ```
    pub fn horizontal(mut self) -> Self {
        self.vertical = false;
        self
    }

    /// Conditionally applies styling when the separator is in vertical orientation.
    ///
    /// This method allows you to apply specific styles only when the separator
    /// is vertical. The handler function is only called if the separator is vertical.
    ///
    /// # Arguments
    ///
    /// * `handler` - A function that takes and returns the separator, applying modifications
    ///
    /// # Examples
    ///
    /// ```rust
    /// let separator = Separator::new()
    ///     .vertical()
    ///     .when_vertical(|this| this.w(px(2)).h_full().bg(rgb(0xe2e8f0)));
    /// ```
    pub fn when_vertical(self, handler: impl FnOnce(Self) -> Self) -> Self {
        if self.vertical { handler(self) } else { self }
    }

    /// Conditionally applies styling based on vertical orientation with an alternative.
    ///
    /// This method applies one set of styles when the separator is vertical and
    /// a different set when it's horizontal. Useful for responsive design patterns.
    ///
    /// # Arguments
    ///
    /// * `handler` - Function called when the separator is vertical
    /// * `else_` - Function called when the separator is horizontal
    ///
    /// # Examples
    ///
    /// ```rust
    /// let separator = Separator::new()
    ///     .when_vertical_else(
    ///         |this| this.w(px(1)).h_full(),     // Vertical: thin and tall
    ///         |this| this.h(px(1)).w_full()      // Horizontal: thin and wide
    ///     );
    /// ```
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

    /// Conditionally applies styling when the separator is in horizontal orientation.
    ///
    /// This method allows you to apply specific styles only when the separator
    /// is horizontal. The handler function is only called if the separator is horizontal.
    ///
    /// # Arguments
    ///
    /// * `handler` - A function that takes and returns the separator, applying modifications
    ///
    /// # Examples
    ///
    /// ```rust
    /// let separator = Separator::new()
    ///     .horizontal()
    ///     .when_horizontal(|this| this.h(px(2)).w_full().bg(rgb(0xd1d5db)));
    /// ```
    pub fn when_horizontal(self, handler: impl FnOnce(Self) -> Self) -> Self {
        if !self.vertical { handler(self) } else { self }
    }

    /// Conditionally applies styling based on horizontal orientation with an alternative.
    ///
    /// This method applies one set of styles when the separator is horizontal and
    /// a different set when it's vertical. Useful for responsive design patterns.
    ///
    /// # Arguments
    ///
    /// * `handler` - Function called when the separator is horizontal
    /// * `else_` - Function called when the separator is vertical
    ///
    /// # Examples
    ///
    /// ```rust
    /// let separator = Separator::new()
    ///     .when_horizontal_else(
    ///         |this| this.h(px(1)).w_full(),     // Horizontal: thin and wide
    ///         |this| this.w(px(1)).h_full()      // Vertical: thin and tall
    ///     );
    /// ```
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

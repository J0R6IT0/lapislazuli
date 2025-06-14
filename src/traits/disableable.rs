/// An element that can be disabled to prevent user interaction.
pub trait Disableable: Sized {
    /// Returns whether the element is currently disabled.
    fn is_disabled(&self) -> bool;

    /// Sets the disabled state of the element.
    fn disabled(self, disabled: bool) -> Self;

    /// Conditionally modify the element if it is disabled.
    fn when_disabled(self, handler: impl FnOnce(Self) -> Self) -> Self {
        if self.is_disabled() {
            handler(self)
        } else {
            self
        }
    }
}

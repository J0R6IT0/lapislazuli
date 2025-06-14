/// An element that can be disabled to prevent user interaction.
pub trait Disableable {
    /// Returns whether the element is currently disabled.
    fn is_disabled(&self) -> bool;

    /// Sets the disabled state of the element.
    fn disabled(self, disabled: bool) -> Self;
}

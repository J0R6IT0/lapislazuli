use gpui::*;

/// A trait for parent elements that can provide context to their children.
///
/// This trait allows components to pass contextual information (like state, configuration, or computed values)
/// to child elements through a closure-based API.
pub trait ParentElementWithContext<Context>: ParentElement + Sized
where
    Context: Clone,
{
    /// Returns the current context that will be provided to child elements.
    fn get_context(&self) -> Context;

    /// Adds a child element that receives the parent's context.
    ///
    /// The provided closure receives a copy of the context and should return
    /// an element that implements `IntoElement`.
    ///
    /// # Example
    ///
    /// ```rust
    /// progress
    ///     .child_with_context(|context| {
    ///         span(format!("Progress: {}", context.percentage()))
    ///     })
    /// ```
    fn child_with_context<F, E>(self, f: F) -> Self
    where
        F: FnOnce(Context) -> E,
        E: IntoElement,
    {
        let context = self.get_context();
        let element = f(context);
        self.child(element)
    }

    /// Adds multiple child elements that receive the parent's context.
    fn children_with_context<F, I, E>(self, f: F) -> Self
    where
        F: FnOnce(Context) -> I,
        I: IntoIterator<Item = E>,
        E: IntoElement,
    {
        let context = self.get_context();
        let elements = f(context);
        self.children(elements)
    }
}

use gpui::ElementId;

pub trait Selectable: Sized {
    fn element_id(&self) -> &ElementId;

    fn selected(self, selected: bool) -> Self;

    fn is_selected(&self) -> bool;
}

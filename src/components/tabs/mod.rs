use gpui::*;
use std::rc::Rc;

mod list;
mod trigger;

pub use list::*;
pub use trigger::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectTab(usize);

impl_internal_actions!(tab_bar, [SelectTab]);

#[derive(IntoElement)]
pub struct Tabs {
    base: Stateful<Div>,
    list: TabsList,
    on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
    value: Option<usize>,
}

impl Tabs {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().id(id),
            list: TabsList::new(),
            on_change: None,
            value: None,
        }
    }

    pub fn list(mut self, handler: impl FnOnce(TabsList) -> TabsList) -> Self {
        self.list = handler(self.list);
        self.list.selected_index = self.value;
        self
    }

    pub fn value(mut self, value: usize) -> Self {
        self.value = Some(value);
        self
    }

    pub fn on_change(
        mut self,
        on_change: impl Fn(&usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self.list.on_change = self.on_change.clone();
        self
    }
}

impl Styled for Tabs {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Tabs {
    fn render(self, _window: &mut Window, _app: &mut App) -> impl IntoElement {
        self.base
            .on_action({
                let on_click = self.on_change;
                move |action: &SelectTab, window, app| {
                    if let Some(on_click) = on_click.clone() {
                        on_click(&action.0, window, app);
                    }
                }
            })
            .child(self.list)
    }
}

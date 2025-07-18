use gpui::SharedString;

pub struct InputEvent {
    pub value: SharedString,
}

pub struct ChangeEvent {
    pub value: SharedString,
}

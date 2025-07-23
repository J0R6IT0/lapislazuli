use super::CONTEXT;
use gpui::{Action, App, KeyBinding, actions};

/// Initialize text field key bindings and actions
pub fn init(app: &mut App) {
    app.bind_keys([
        key_binding("left", Left),
        key_binding("right", Right),
        key_binding("home", Home),
        key_binding("end", End),
        key_binding("shift-left", SelectLeft),
        key_binding("shift-right", SelectRight),
        key_binding("backspace", Backspace),
        key_binding("delete", Delete),
        key_binding("enter", Enter),
    ]);

    #[cfg(target_os = "macos")]
    macos_bindings(app);
    #[cfg(not(target_os = "macos"))]
    windows_linux_bindings(app);
}

#[cfg(not(target_os = "macos"))]
fn windows_linux_bindings(app: &mut App) {
    app.bind_keys([
        key_binding("ctrl-left", WordLeft),
        key_binding("ctrl-right", WordRight),
        key_binding("ctrl-a", SelectAll),
        key_binding("ctrl-shift-left", SelectWordLeft),
        key_binding("ctrl-shift-right", SelectWordRight),
        key_binding("shift-home", SelectToHome),
        key_binding("shift-end", SelectToEnd),
        key_binding("ctrl-backspace", DeleteWordLeft),
        key_binding("ctrl-delete", DeleteWordRight),
        key_binding("ctrl-c", Copy),
        key_binding("ctrl-insert", Copy),
        key_binding("ctrl-v", Paste),
        key_binding("shift-insert", Paste),
        key_binding("ctrl-x", Cut),
        key_binding("shift-delete", Cut),
        key_binding("ctrl-z", Undo),
        key_binding("ctrl-y", Redo),
        key_binding("ctrl-shift-z", Redo),
    ]);
}

#[cfg(target_os = "macos")]
fn macos_bindings(app: &mut App) {
    app.bind_keys([
        key_binding("ctrl-b", Left),
        key_binding("ctrl-f", Right),
        key_binding("alt-left", WordLeft),
        key_binding("alt-right", WordRight),
        key_binding("ctrl-a", Home),
        key_binding("cmd-left", Home),
        key_binding("ctrl-e", End),
        key_binding("cmd-right", End),
        key_binding("cmd-a", SelectAll),
        key_binding("alt-shift-left", SelectWordLeft),
        key_binding("alt-shift-right", SelectWordRight),
        key_binding("cmd-shift-left", SelectToHome),
        key_binding("cmd-shift-right", SelectToEnd),
        key_binding("alt-backspace", DeleteWordLeft),
        key_binding("alt-delete", DeleteWordRight),
        key_binding("cmd-backspace", DeleteToBeginning),
        key_binding("cmd-delete", DeleteToEnd),
        key_binding("cmd-c", Copy),
        key_binding("cmd-v", Paste),
        key_binding("cmd-x", Cut),
        key_binding("ctrl-cmd-space", ShowCharacterPalette),
        key_binding("cmd-z", Undo),
        key_binding("cmd-shift-z", Redo),
    ]);
}

fn key_binding(keystrokes: &str, action: impl Action) -> KeyBinding {
    KeyBinding::new(keystrokes, action, Some(CONTEXT))
}

actions!(
    lp_text_field,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        ShowCharacterPalette,
        Copy,
        Paste,
        Cut,
        DeleteWordLeft,
        DeleteWordRight,
        DeleteToBeginning,
        DeleteToEnd,
        WordLeft,
        WordRight,
        SelectWordLeft,
        SelectWordRight,
        SelectToHome,
        SelectToEnd,
        Undo,
        Redo,
        Enter,
    ]
);

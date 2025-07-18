use gpui::{App, KeyBinding, actions};

use crate::components::text_field::CONTEXT;

/// Initialize text field key bindings and actions
pub fn init(app: &mut App) {
    app.bind_keys([
        KeyBinding::new("left", Left, Some(CONTEXT)),
        KeyBinding::new("right", Right, Some(CONTEXT)),
        KeyBinding::new("home", Home, Some(CONTEXT)),
        KeyBinding::new("end", End, Some(CONTEXT)),
        KeyBinding::new("alt-left", WordLeft, Some(CONTEXT)),
        KeyBinding::new("alt-right", WordRight, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-a", Home, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-left", Home, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-e", End, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-right", End, Some(CONTEXT)),
        KeyBinding::new("shift-left", SelectLeft, Some(CONTEXT)),
        KeyBinding::new("shift-right", SelectRight, Some(CONTEXT)),
        KeyBinding::new("cmd-a", SelectAll, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some(CONTEXT)),
        KeyBinding::new("alt-shift-left", SelectWordLeft, Some(CONTEXT)),
        KeyBinding::new("alt-shift-right", SelectWordRight, Some(CONTEXT)),
        KeyBinding::new("shift-home", SelectToHome, Some(CONTEXT)),
        KeyBinding::new("shift-end", SelectToEnd, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-left", SelectToHome, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-right", SelectToEnd, Some(CONTEXT)),
        KeyBinding::new("backspace", Backspace, Some(CONTEXT)),
        KeyBinding::new("delete", Delete, Some(CONTEXT)),
        KeyBinding::new("alt-backspace", DeleteWordLeft, Some(CONTEXT)),
        KeyBinding::new("alt-delete", DeleteWordRight, Some(CONTEXT)),
        KeyBinding::new("cmd-backspace", DeleteToBeginning, Some(CONTEXT)),
        KeyBinding::new("cmd-delete", DeleteToEnd, Some(CONTEXT)),
        KeyBinding::new("cmd-c", Copy, Some(CONTEXT)),
        KeyBinding::new("cmd-v", Paste, Some(CONTEXT)),
        KeyBinding::new("cmd-x", Cut, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some(CONTEXT)),
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-z", Undo, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-y", Redo, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-shift-z", Redo, Some(CONTEXT)),
        KeyBinding::new("cmd-z", Undo, Some(CONTEXT)),
        KeyBinding::new("cmd-shift-z", Redo, Some(CONTEXT)),
        KeyBinding::new("enter", Enter, Some(CONTEXT)),
    ]);
}

actions!(
    input,
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

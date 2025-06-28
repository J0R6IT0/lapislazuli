use std::ops::Range;

use gpui::*;

#[derive(Clone, Debug)]
pub enum Operation {
    Insert {
        range: Range<usize>,
        text: SharedString,
    },
    Delete {
        range: Range<usize>,
        deleted_text: SharedString,
    },
    Replace {
        range: Range<usize>,
        old_text: SharedString,
        new_text: SharedString,
    },
}

impl Operation {
    pub fn invert(&self) -> Operation {
        match self {
            Operation::Insert { range, text } => Operation::Delete {
                range: range.start..range.start,
                deleted_text: text.clone(),
            },
            Operation::Delete {
                range,
                deleted_text,
            } => Operation::Insert {
                range: range.clone(),
                text: deleted_text.clone(),
            },
            Operation::Replace {
                range,
                old_text,
                new_text: _,
            } => Operation::Replace {
                range: range.clone(),
                old_text: old_text.clone(),
                new_text: old_text.clone(),
            },
        }
    }
}

pub struct History {
    pub undo_stack: Vec<SharedString>,
    pub redo_stack: Vec<SharedString>,
    pub current: SharedString,
}

impl History {
    pub fn new(initial: SharedString) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current: initial,
        }
    }

    pub fn push(&mut self, text: SharedString) {
        if self.current == text {
            return;
        }
        self.undo_stack.push(self.current.clone());
        self.current = text;
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<SharedString> {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.current.clone());
            self.current = prev.clone();
            Some(prev)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<SharedString> {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.current.clone());
            self.current = next.clone();
            Some(next)
        } else {
            None
        }
    }
}

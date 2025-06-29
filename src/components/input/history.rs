use gpui::*;
use std::ops::Range;

#[derive(Clone, Debug)]
pub enum HistoryChange {
    Insert {
        text: SharedString,
        range: Range<usize>,
    },
    Delete {
        text: SharedString,
        range: Range<usize>,
    },
    Replace {
        old_text: SharedString,
        new_text: SharedString,
        range: Range<usize>,
    },
}

impl HistoryChange {
    pub fn inverse(&self) -> HistoryChange {
        match self {
            HistoryChange::Insert { range, text } => HistoryChange::Delete {
                text: SharedString::new(""),
                range: range.start..range.start + text.len(),
            },
            HistoryChange::Delete { text, range } => HistoryChange::Insert {
                text: text.clone(),
                range: range.start..range.start,
            },
            HistoryChange::Replace {
                old_text,
                new_text,
                range,
            } => HistoryChange::Replace {
                old_text: new_text.clone(),
                new_text: old_text.clone(),
                range: range.start..range.start + new_text.len(),
            },
        }
    }
}

pub struct History {
    undo_stack: Vec<HistoryChange>,
    redo_stack: Vec<HistoryChange>,
    max_size: usize,
}

impl History {
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::with_capacity(max_size),
            redo_stack: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, text: HistoryChange) {
        self.redo_stack.clear();
        self.undo_stack.push(text);
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self) -> Option<HistoryChange> {
        if let Some(change) = self.undo_stack.pop() {
            let inverse = change.inverse();
            self.redo_stack.push(change);
            Some(inverse)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<HistoryChange> {
        if let Some(change) = self.redo_stack.pop() {
            self.undo_stack.push(change.clone());
            Some(change)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

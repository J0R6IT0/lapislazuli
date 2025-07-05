use gpui::SharedString;
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub enum Change {
    Insert {
        range: Range<usize>,
        text: SharedString,
    },
    Delete {
        range: Range<usize>,
        text: SharedString,
    },
    Replace {
        range: Range<usize>,
        old_text: SharedString,
        new_text: SharedString,
    },
}

impl Change {
    fn inverse(self) -> Change {
        match self {
            Change::Insert { range, text } => Change::Delete {
                range: range.start..range.start + text.grapheme_indices(true).count(),
                text: SharedString::new(""),
            },
            Change::Delete { range, text } => Change::Insert {
                range: range.start..range.start,
                text: text,
            },
            Change::Replace {
                range,
                old_text,
                new_text,
            } => Change::Replace {
                range: range.start..range.start + new_text.grapheme_indices(true).count(),
                old_text: new_text,
                new_text: old_text,
            },
        }
    }

    pub fn text(&self) -> SharedString {
        match self {
            Change::Insert { text, .. } => text.clone(),
            Change::Delete { text, .. } => text.clone(),
            Change::Replace { new_text, .. } => new_text.clone(),
        }
    }

    pub fn range(&self) -> Range<usize> {
        match self {
            Change::Insert { range, .. } => range.clone(),
            Change::Delete { range, .. } => range.clone(),
            Change::Replace { range, .. } => range.clone(),
        }
    }
}

#[derive(Clone)]
pub struct HistoryEntry {
    pub change: Change,
}

pub struct History {
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
    max_size: usize,
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    pub fn new() -> Self {
        Self::with_max_size(100)
    }

    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    pub fn push(&mut self, change: Change) {
        self.undo_stack.push(HistoryEntry { change });
        self.redo_stack.clear();

        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self) -> Option<Change> {
        if let Some(entry) = self.undo_stack.pop() {
            self.redo_stack.push(entry.clone());
            let inverse_change = entry.change.inverse();
            Some(inverse_change)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<Change> {
        if let Some(entry) = self.redo_stack.pop() {
            self.undo_stack.push(entry.clone());
            Some(entry.change)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

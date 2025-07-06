use gpui::SharedString;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
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
        marked: bool,
    },
}

impl Change {
    fn inverse(self) -> Change {
        match self {
            Change::Insert { range, text } => Change::Delete {
                range: range.start..range.start + text.len(),
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
                marked,
            } => Change::Replace {
                range: range.start..range.start + new_text.len(),
                old_text: new_text,
                new_text: old_text,
                marked,
            },
        }
    }

    pub fn text(&self) -> SharedString {
        match self {
            Change::Insert { text, .. } => text.clone(),
            Change::Delete { .. } => SharedString::new(""),
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

    pub fn selection_range(&self) -> Range<usize> {
        match self {
            Change::Insert { range, text } => range.start..range.start + text.len(),
            Change::Delete { range, .. } => range.start..range.start,
            Change::Replace {
                range, new_text, ..
            } => range.start..range.start + new_text.len(),
        }
    }

    fn merge_with(self, other: &Change) -> Option<Change> {
        match (self, other) {
            (
                Change::Insert {
                    range: r1,
                    text: t1,
                },
                Change::Insert {
                    range: r2,
                    text: t2,
                },
            ) if r1.start + t1.len() == r2.start => Some(Change::Insert {
                range: r1.start..r1.start,
                text: SharedString::from(format!("{}{}", t1, t2)),
            }),
            (
                Change::Delete {
                    range: r1,
                    text: t1,
                },
                Change::Insert {
                    range: r2,
                    text: t2,
                },
            ) if r1.start == r2.start => Some(Change::Replace {
                range: r1.clone(),
                old_text: t1,
                new_text: t2.clone(),
                marked: false,
            }),
            (
                Change::Delete {
                    range: r1,
                    text: t1,
                },
                Change::Delete {
                    range: r2,
                    text: t2,
                },
            ) if r1.start == r2.end => Some(Change::Delete {
                range: r2.start..r1.end,
                text: SharedString::from(format!("{}{}", t2, t1)),
            }),
            (
                Change::Delete {
                    range: r1,
                    text: t1,
                },
                Change::Delete {
                    range: r2,
                    text: t2,
                },
            ) if r1.start == r2.start => Some(Change::Delete {
                range: r1.start..(r1.end.max(r2.end)),
                text: SharedString::from(format!("{}{}", t1, t2)),
            }),
            (
                Change::Replace {
                    range: r1,
                    new_text: t1,
                    old_text,
                    ..
                },
                Change::Insert { text: t2, .. },
            ) => Some(Change::Replace {
                range: r1,
                new_text: SharedString::from(format!("{}{}", t1, t2)),
                old_text,
                marked: false,
            }),
            (
                Change::Insert {
                    text: t1,
                    range: r1,
                },
                Change::Replace {
                    new_text,
                    old_text,
                    marked,
                    ..
                },
            ) if *marked && t1.ends_with(old_text.as_ref()) => Some(Change::Insert {
                range: r1,
                text: SharedString::from(format!(
                    "{}{}",
                    &t1[..t1.len() - old_text.len()],
                    new_text
                )),
            }),
            (
                Change::Replace {
                    range: r1,
                    new_text: nt1,
                    old_text: ot1,
                    ..
                },
                Change::Replace {
                    new_text: nt2,
                    old_text: ot2,
                    marked,
                    ..
                },
            ) if *marked && nt1.ends_with(ot2.as_ref()) => Some(Change::Replace {
                range: r1,
                old_text: ot1,
                new_text: SharedString::from(format!("{}{}", &nt1[..nt1.len() - ot2.len()], nt2)),
                marked: false,
            }),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HistoryEntry {
    pub change: Change,
}

pub struct History {
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
    max_size: usize,
    can_merge: bool,
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
            can_merge: true,
        }
    }

    pub fn push(&mut self, change: Change) {
        self.redo_stack.clear();

        if self.can_merge
            && let Some(last_entry) = self.undo_stack.last_mut()
            && let Some(merged_change) = last_entry.change.clone().merge_with(&change)
        {
            last_entry.change = merged_change;
            return;
        }

        self.undo_stack.push(HistoryEntry { change });
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
        self.can_merge = true;
    }

    pub fn undo(&mut self) -> Option<Change> {
        self.prevent_merge();
        if let Some(entry) = self.undo_stack.pop() {
            self.redo_stack.push(entry.clone());
            let inverse_change = entry.change.inverse();
            Some(inverse_change)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<Change> {
        self.prevent_merge();
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

    pub fn prevent_merge(&mut self) {
        self.can_merge = false;
    }
}

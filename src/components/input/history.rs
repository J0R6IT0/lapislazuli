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

#[derive(Clone, Debug, PartialEq)]
pub enum ChangeType {
    Insert,
    Delete,
    Replace,
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

    pub fn change_type(&self) -> ChangeType {
        match self {
            HistoryChange::Insert { .. } => ChangeType::Insert,
            HistoryChange::Delete { .. } => ChangeType::Delete,
            HistoryChange::Replace { .. } => ChangeType::Replace,
        }
    }

    pub fn can_merge_with(&self, other: &HistoryChange) -> bool {
        if self.change_type() != other.change_type() {
            return false;
        }

        match (self, other) {
            (
                HistoryChange::Insert {
                    range: range1,
                    text: text1,
                },
                HistoryChange::Insert {
                    range: range2,
                    text: text2,
                },
            ) => {
                range1.end + 1 == range2.start
            }
            (
                HistoryChange::Delete { range: range1, .. },
                HistoryChange::Delete { range: range2, .. },
            ) => range2.end == range1.start || range1.end == range2.start,
            _ => false,
        }
    }

    pub fn merge_with(&self, other: &HistoryChange) -> Option<HistoryChange> {
        match (self, other) {
            (
                HistoryChange::Insert {
                    range: range1,
                    text: text1,
                },
                HistoryChange::Insert {
                    range: range2,
                    text: text2,
                },
            ) if range1.end + 1 == range2.start => {
                let mut merged_text = text1.to_string();
                merged_text.push_str(&text2);
                Some(HistoryChange::Insert {
                    text: SharedString::from(merged_text),
                    range: range1.start..range2.end,
                })
            }
            (
                HistoryChange::Delete {
                    range: range1,
                    text: text1,
                },
                HistoryChange::Delete {
                    range: range2,
                    text: text2,
                },
            ) => {
                if range2.end == range1.start {
                    let mut merged_text = text2.to_string();
                    merged_text.push_str(&text1);
                    Some(HistoryChange::Delete {
                        text: SharedString::from(merged_text),
                        range: range2.start..range1.end,
                    })
                } else if range1.end == range2.start {
                    let mut merged_text = text1.to_string();
                    merged_text.push_str(&text2);
                    Some(HistoryChange::Delete {
                        text: SharedString::from(merged_text),
                        range: range1.start..range2.end,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
struct HistoryEntry {
    change: HistoryChange,
}

pub struct History {
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
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

    pub fn push(&mut self, change: HistoryChange) {
         self.redo_stack.clear();

         let entry = HistoryEntry {
             change: change.clone(),
         };

         if let Some(last_entry) = self.undo_stack.last_mut() {
             if last_entry.change.can_merge_with(&change) {
                 if let Some(merged_change) = last_entry.change.merge_with(&change) {
                     last_entry.change = merged_change;
                     return;
                 }
             }
         }

         self.undo_stack.push(entry);

         if self.undo_stack.len() > self.max_size {
             self.undo_stack.remove(0);
         }
    }

    pub fn undo(&mut self) -> Option<HistoryChange> {
        if let Some(entry) = self.undo_stack.pop() {
            let inverse = entry.change.inverse();
            self.redo_stack.push(entry);
            Some(inverse)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<HistoryChange> {
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

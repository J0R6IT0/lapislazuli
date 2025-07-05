#[cfg(test)]
mod history {
    use std::ops::Range;

    use crate::components::input::history::{Change, History};

    fn insert_text(history: &mut History, text: &str) {
        for (i, ch) in text.char_indices() {
            history.push(Change::Insert {
                text: ch.to_string().into(),
                range: i..i,
            });
        }
    }

    fn cut_text(history: &mut History, text: &str, range: Range<usize>) {
        history.prevent_merge();
        history.push(Change::Delete {
            text: text.to_string().into(),
            range,
        });
    }

    fn paste_text(history: &mut History, text: &str, range: Range<usize>) {
        history.prevent_merge();
        history.push(Change::Insert {
            text: text.to_string().into(),
            range,
        });
    }

    #[test]
    fn simple_insertions() {
        let mut history = History::new();
        insert_text(&mut history, "Hello World!");

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Delete {
                text: "".into(),
                range: 0..12
            }
        );
        assert_eq!(undo.selection_range(), 0..0);

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Insert {
                text: "Hello World!".into(),
                range: 0..0
            }
        );
    }

    #[test]
    fn paste_over_selection() {
        let mut history = History::new();
        insert_text(&mut history, "abcdef");

        history.prevent_merge();
        history.push(Change::Replace {
            range: 2..4,
            old_text: "cd".into(),
            new_text: "X".into(),
        });
        history.push(Change::Insert {
            text: "Y".into(),
            range: 3..3,
        });

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Replace {
                range: 2..4,
                old_text: "XY".into(),
                new_text: "cd".into(),
            }
        );
        assert_eq!(undo.selection_range(), 2..4);

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Delete {
                text: "".into(),
                range: 0..6
            }
        );
        assert_eq!(undo.selection_range(), 0..0);

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Insert {
                text: "abcdef".into(),
                range: 0..0
            }
        );

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Replace {
                range: 2..4,
                old_text: "cd".into(),
                new_text: "XY".into(),
            }
        );
    }

    #[test]
    fn cut_and_paste() {
        let mut history = History::new();
        insert_text(&mut history, "quick brown fox");
        cut_text(&mut history, "brown ", 6..12);
        paste_text(&mut history, "brown ", 0..0);

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Delete {
                text: "".into(),
                range: 0..6
            }
        );
        assert_eq!(undo.selection_range(), 0..0);

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Insert {
                text: "brown ".into(),
                range: 6..6
            }
        );
        assert_eq!(undo.selection_range(), 6..12);

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Delete {
                text: "".into(),
                range: 0..15
            }
        );
        assert_eq!(undo.selection_range(), 0..0);

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Insert {
                text: "quick brown fox".into(),
                range: 0..0
            }
        );

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Delete {
                text: "brown ".into(),
                range: 6..12
            }
        );

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Insert {
                text: "brown ".into(),
                range: 0..0
            }
        );
    }

    #[test]
    fn replace_same_text() {
        let mut history = History::new();
        insert_text(&mut history, "quick brown fox");

        history.push(Change::Replace {
            range: 6..11,
            old_text: "brown".into(),
            new_text: "brown".into(),
        });

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Replace {
                range: 6..11,
                old_text: "brown".into(),
                new_text: "brown".into(),
            }
        );
        assert_eq!(undo.selection_range(), 6..11);

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Delete {
                text: "".into(),
                range: 0..15
            }
        );
        assert_eq!(undo.selection_range(), 0..0);

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Insert {
                text: "quick brown fox".into(),
                range: 0..0
            }
        );
    }
}

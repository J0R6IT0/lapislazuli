#[cfg(test)]
mod history {
    use crate::components::input::history::{Change, History};
    use std::ops::Range;

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

        history.push(Change::Replace {
            range: 2..4,
            old_text: "cd".into(),
            new_text: "X".into(),
            marked: false,
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
                marked: false,
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
                marked: false,
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
            marked: false,
        });

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Replace {
                range: 6..11,
                old_text: "brown".into(),
                new_text: "brown".into(),
                marked: false,
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

    #[test]
    fn undo_redo_mixed() {
        let mut history = History::new();
        insert_text(&mut history, "Hello World!");
        cut_text(&mut history, "Hello", 0..5);

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Insert {
                text: "Hello".into(),
                range: 0..0
            }
        );
        assert_eq!(undo.selection_range(), 0..5);

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Delete {
                range: 0..5,
                text: "Hello".into(),
            }
        );

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Insert {
                text: "Hello".into(),
                range: 0..0
            }
        );
        assert_eq!(undo.selection_range(), 0..5);
    }

    #[test]
    fn undo_clear() {
        let mut history = History::new();
        insert_text(&mut history, "tree");

        history.push(Change::Delete {
            text: "r".into(),
            range: 1..2,
        });

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Insert {
                text: "r".into(),
                range: 1..1
            }
        );
        assert_eq!(undo.selection_range(), 1..2);

        history.push(Change::Insert {
            text: "s".into(),
            range: 4..4,
        });

        assert!(history.redo().is_none())
    }

    #[test]
    fn write_delete_type() {
        let mut history = History::new();
        insert_text(&mut history, "world");

        history.push(Change::Delete {
            text: "ld".into(),
            range: 3..5,
        });

        history.push(Change::Insert {
            range: 3..3,
            text: "l".into(),
        });
        history.push(Change::Insert {
            range: 4..4,
            text: "d".into(),
        });
        history.push(Change::Insert {
            range: 5..5,
            text: "w".into(),
        });
        history.push(Change::Insert {
            range: 6..6,
            text: "i".into(),
        });
        history.push(Change::Insert {
            range: 7..7,
            text: "d".into(),
        });
        history.push(Change::Insert {
            range: 8..8,
            text: "e".into(),
        });

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Replace {
                old_text: "ldwide".into(),
                new_text: "ld".into(),
                range: 3..9,
                marked: false,
            }
        );
        assert_eq!(undo.selection_range(), 3..5);

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Delete {
                text: "".into(),
                range: 0..5
            }
        );
        assert_eq!(undo.selection_range(), 0..0);

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Insert {
                text: "world".into(),
                range: 0..0
            }
        );

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Replace {
                old_text: "ld".into(),
                new_text: "ldwide".into(),
                range: 3..5,
                marked: false,
            }
        );
    }

    #[test]
    fn select_all_replace() {
        let mut history = History::new();
        insert_text(&mut history, "important note");

        history.push(Change::Replace {
            range: 0..14,
            old_text: "important note".into(),
            new_text: "R".into(),
            marked: false,
        });

        history.push(Change::Insert {
            range: 1..1,
            text: "E".into(),
        });
        history.push(Change::Insert {
            range: 2..2,
            text: "M".into(),
        });
        history.push(Change::Insert {
            range: 3..3,
            text: "O".into(),
        });
        history.push(Change::Insert {
            range: 4..4,
            text: "V".into(),
        });
        history.push(Change::Insert {
            range: 5..5,
            text: "E".into(),
        });
        history.push(Change::Insert {
            range: 6..6,
            text: "D".into(),
        });

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Replace {
                old_text: "REMOVED".into(),
                new_text: "important note".into(),
                range: 0..7,
                marked: false,
            }
        );
        assert_eq!(undo.selection_range(), 0..14);

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Delete {
                text: "".into(),
                range: 0..14
            }
        );
        assert_eq!(undo.selection_range(), 0..0);

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Insert {
                text: "important note".into(),
                range: 0..0
            }
        );

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Replace {
                old_text: "important note".into(),
                new_text: "REMOVED".into(),
                range: 0..14,
                marked: false,
            }
        );
    }

    #[test]
    fn emojis() {
        let mut history = History::new();
        insert_text(&mut history, "hello 👋 world");

        history.push(Change::Replace {
            range: 6..10,
            old_text: "👋".into(),
            new_text: "🌍".into(),
            marked: false,
        });

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Replace {
                range: 6..10,
                old_text: "🌍".into(),
                new_text: "👋".into(),
                marked: false,
            }
        );
        assert_eq!(undo.selection_range(), 6..10);

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Delete {
                text: "".into(),
                range: 0..16
            }
        );
        assert_eq!(undo.selection_range(), 0..0);

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Insert {
                text: "hello 👋 world".into(),
                range: 0..0
            }
        );

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Replace {
                range: 6..10,
                old_text: "👋".into(),
                new_text: "🌍".into(),
                marked: false,
            }
        );
    }

    #[test]
    fn simple_marked() {
        let mut history = History::new();

        history.push(Change::Insert {
            range: 0..0,
            text: "´".into(),
        });

        history.push(Change::Replace {
            range: 0..2,
            old_text: "´".into(),
            new_text: "á".into(),
            marked: true,
        });

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Delete {
                text: "".into(),
                range: 0..2
            }
        );
        assert_eq!(undo.selection_range(), 0..0);

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Insert {
                text: "á".into(),
                range: 0..0
            }
        );
    }

    #[test]
    fn marked_sequence() {
        let mut history = History::new();
        insert_text(&mut history, "hello w´");

        history.push(Change::Replace {
            range: 7..9,
            old_text: "´".into(),
            new_text: "ó".into(),
            marked: true,
        });
        history.push(Change::Insert {
            range: 9..9,
            text: "rld".into(),
        });

        let undo = history.undo().unwrap();
        assert_eq!(
            undo,
            Change::Delete {
                range: 0..12,
                text: "".into()
            }
        );
        assert_eq!(undo.selection_range(), 0..0);

        let redo = history.redo().unwrap();
        assert_eq!(
            redo,
            Change::Insert {
                text: "hello wórld".into(),
                range: 0..0
            }
        );
    }
}

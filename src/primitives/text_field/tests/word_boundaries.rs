#[cfg(test)]
mod word_boundaries {
    use crate::components::text_field::text_ops::TextOps;

    fn test_boundaries(text: &str, cursor: usize, expected_prev: usize, expected_next: usize) {
        let prev = TextOps::previous_word_boundary(text, cursor);
        let next = TextOps::next_word_boundary(text, cursor);
        assert_eq!(
            prev, expected_prev,
            "prev_word_boundary failed for text='{text}', cursor={cursor}"
        );
        assert_eq!(
            next, expected_next,
            "next_word_boundary failed for text='{text}', cursor={cursor}"
        );
    }

    #[test]
    fn simple_words() {
        test_boundaries("hello world", 6, 0, 11);
        test_boundaries("hello world", 5, 0, 11);
        test_boundaries("hello world", 0, 0, 5);
    }

    #[test]
    fn multiple_spaces() {
        test_boundaries("hello  world", 6, 0, 12);
        test_boundaries("hello  world", 5, 0, 12);
        test_boundaries("hello  world", 0, 0, 5);
        test_boundaries("  hello world  ", 7, 2, 13);
        test_boundaries("  hello world  ", 6, 2, 7);
        test_boundaries("  hello world  ", 0, 0, 7);
        test_boundaries("   ", 0, 0, 3);
    }

    #[test]
    fn punctuation() {
        test_boundaries("hello, world!", 6, 5, 12);
        test_boundaries("hello, world!", 5, 0, 6);
        test_boundaries("hello, world!", 0, 0, 5);
        test_boundaries("hello... world!", 6, 5, 8);
        test_boundaries("hello@world.com", 0, 0, 5);
        test_boundaries("hello@world.com", 5, 0, 6);
        test_boundaries("hello@world.com", 6, 5, 11);
        test_boundaries("hello-world_test", 0, 0, 5);
        test_boundaries("hello-world_test", 5, 0, 6);
        test_boundaries("hello-world_test", 6, 5, 16);
    }

    #[test]
    fn numbers() {
        test_boundaries("123 456", 3, 0, 7);
        test_boundaries("123 456", 2, 0, 3);
        test_boundaries("123 456", 0, 0, 3);
        test_boundaries("123.456", 3, 0, 7);
        test_boundaries("123.456", 2, 0, 7);
        test_boundaries("123.456", 0, 0, 7);
        test_boundaries("1.23e10", 5, 0, 7);
    }

    #[test]
    fn emojis() {
        test_boundaries("hello 👋 world", 6, 0, 10);
        test_boundaries("hello 👋 world", 5, 0, 10);
        test_boundaries("hello 👋 world", 0, 0, 5);
        test_boundaries("👋 hello world", 0, 0, 4);
        test_boundaries("👋 hello world", 4, 0, 10);
        test_boundaries("👋 hello world", 7, 5, 10);
    }

    #[test]
    fn mixed() {
        test_boundaries("file_name_v2-final.txt", 0, 0, 12);
        test_boundaries("file_name_v2-final.txt", 12, 0, 13);
        test_boundaries("file_name_v2-final.txt", 13, 12, 18);
        test_boundaries("file_name_v2-final.txt", 18, 13, 19);
        test_boundaries("file_name_v2-final.txt", 19, 18, 22);
        test_boundaries("the quick-brown_fox42 jumps!", 0, 0, 3);
        test_boundaries("the quick-brown_fox42 jumps!", 3, 0, 9);
        test_boundaries("the quick-brown_fox42 jumps!", 9, 4, 10);
        test_boundaries("the quick-brown_fox42 jumps!", 10, 9, 21);
        test_boundaries("the quick-brown_fox42 jumps!", 21, 10, 27);
        test_boundaries("the quick-brown_fox42 jumps!", 27, 22, 28);
    }
}

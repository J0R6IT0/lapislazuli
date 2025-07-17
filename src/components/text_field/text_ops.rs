//! Text operations module for cursor movement and text manipulation
//!
//! This module provides utilities for working with text boundaries, cursor positioning,
//! and text manipulation operations like word boundaries and grapheme clusters.

use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

/// Character type for word boundary detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharType {
    Whitespace,
    Word,
    Punctuation,
}

pub struct TextOps;

impl TextOps {
    /// Get the previous grapheme boundary from the given offset
    pub fn previous_boundary(text: &str, offset: usize) -> usize {
        let graphemes = text.grapheme_indices(true);
        let mut prev_offset = 0;
        for (i, _) in graphemes {
            if i >= offset {
                break;
            }
            prev_offset = i;
        }
        prev_offset
    }

    /// Get the next grapheme boundary from the given offset
    pub fn next_boundary(text: &str, offset: usize) -> usize {
        text.grapheme_indices(true)
            .find(|(i, _)| *i > offset)
            .map(|(i, _)| i)
            .unwrap_or(text.len())
    }

    /// Get the previous word boundary from the given offset
    pub fn previous_word_boundary(text: &str, offset: usize) -> usize {
        if offset == 0 {
            return 0;
        }

        let mut iter = text.char_indices().rev().peekable();
        let mut found_non_whitespace = false;
        let mut last_char_type = None;
        let mut prev_ch = None;

        while let Some((i, ch)) = iter.next() {
            if i >= offset {
                prev_ch = Some(ch);
                continue;
            }

            let next_ch = iter.peek().map(|&(_, c)| c);
            let char_type = Self::char_type(ch, next_ch, prev_ch);

            if !found_non_whitespace && char_type != CharType::Whitespace {
                found_non_whitespace = true;
                last_char_type = Some(char_type);
                prev_ch = Some(ch);
                continue;
            }

            if found_non_whitespace {
                if let Some(last_type) = last_char_type {
                    if char_type != last_type || char_type == CharType::Whitespace {
                        return Self::next_boundary(text, i);
                    }
                }
            }

            last_char_type = Some(char_type);
            prev_ch = Some(ch);
        }

        0
    }

    /// Get the next word boundary from the given offset
    pub fn next_word_boundary(text: &str, offset: usize) -> usize {
        if offset >= text.len() {
            return text.len();
        }

        let mut iter = text.char_indices().peekable();
        let mut found_non_whitespace = false;
        let mut last_char_type = None;
        let mut prev_ch = None;

        while let Some((i, ch)) = iter.next() {
            if i < offset {
                prev_ch = Some(ch);
                continue;
            }

            let next_ch = iter.peek().map(|&(_, c)| c);
            let char_type = Self::char_type(ch, next_ch, prev_ch);

            if !found_non_whitespace && char_type != CharType::Whitespace {
                found_non_whitespace = true;
                last_char_type = Some(char_type);
                prev_ch = Some(ch);
                continue;
            }

            if found_non_whitespace {
                if let Some(last_type) = last_char_type {
                    if char_type != last_type || char_type == CharType::Whitespace {
                        return i;
                    }
                }
            }

            last_char_type = Some(char_type);
            prev_ch = Some(ch);
        }

        text.len()
    }

    /// Determine the character type for word boundary detection
    fn char_type(ch: char, next: Option<char>, prev: Option<char>) -> CharType {
        if ch.is_whitespace() {
            CharType::Whitespace
        } else if (ch == '.')
            && prev.map_or(false, |c| c.is_ascii_digit())
            && next.map_or(false, |c| c.is_ascii_digit())
        {
            CharType::Word
        } else if ch.is_alphanumeric() || ch == '_' {
            CharType::Word
        } else {
            CharType::Punctuation
        }
    }

    /// Convert a grapheme offset to a byte offset
    pub fn grapheme_offset_to_byte_offset(text: &str, grapheme_offset: usize) -> usize {
        text.grapheme_indices(true)
            .nth(grapheme_offset)
            .map(|(i, _)| i)
            .unwrap_or(text.len())
    }

    /// Convert offset to UTF-16 code units
    pub fn offset_to_utf16(text: &str, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut byte_offset = 0;

        for ch in text.chars() {
            if byte_offset >= offset {
                break;
            }
            utf16_offset += ch.len_utf16();
            byte_offset += ch.len_utf8();
        }

        utf16_offset
    }

    /// Convert UTF-16 offset to byte offset
    pub fn offset_from_utf16(text: &str, utf16_offset: usize) -> usize {
        let mut current_utf16_offset = 0;
        let mut byte_offset = 0;

        for ch in text.chars() {
            if current_utf16_offset >= utf16_offset {
                break;
            }
            current_utf16_offset += ch.len_utf16();
            byte_offset += ch.len_utf8();
        }

        byte_offset
    }

    /// Convert a byte range to UTF-16 range
    pub fn range_to_utf16(text: &str, range: &Range<usize>) -> Range<usize> {
        Self::offset_to_utf16(text, range.start)..Self::offset_to_utf16(text, range.end)
    }

    /// Convert a UTF-16 range to byte range
    pub fn range_from_utf16(text: &str, range: &Range<usize>) -> Range<usize> {
        Self::offset_from_utf16(text, range.start)..Self::offset_from_utf16(text, range.end)
    }
}

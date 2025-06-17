use crate::components::input::element::{CURSOR_WIDTH, TextElement};
use gpui::*;
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

/// Context identifier for input key bindings
pub(super) const CONTEXT: &str = "input";

/// Initialize input key bindings and actions
pub fn init(cx: &mut App) {
    cx.bind_keys([
        // Basic cursor movement
        KeyBinding::new("left", Left, Some(CONTEXT)),
        KeyBinding::new("right", Right, Some(CONTEXT)),
        KeyBinding::new("home", Home, Some(CONTEXT)),
        KeyBinding::new("end", End, Some(CONTEXT)),
        // Word movement
        KeyBinding::new("alt-left", WordLeft, Some(CONTEXT)),
        KeyBinding::new("alt-right", WordRight, Some(CONTEXT)),
        // macOS cursor movement alternatives
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-a", Home, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-left", Home, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-e", End, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-right", End, Some(CONTEXT)),
        // Text selection
        KeyBinding::new("shift-left", SelectLeft, Some(CONTEXT)),
        KeyBinding::new("shift-right", SelectRight, Some(CONTEXT)),
        KeyBinding::new("cmd-a", SelectAll, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some(CONTEXT)),
        // Word selection
        KeyBinding::new("alt-shift-left", SelectWordLeft, Some(CONTEXT)),
        KeyBinding::new("alt-shift-right", SelectWordRight, Some(CONTEXT)),
        // Line selection
        KeyBinding::new("shift-home", SelectToHome, Some(CONTEXT)),
        KeyBinding::new("shift-end", SelectToEnd, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-left", SelectToHome, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-right", SelectToEnd, Some(CONTEXT)),
        // Basic deletion
        KeyBinding::new("backspace", Backspace, Some(CONTEXT)),
        KeyBinding::new("delete", Delete, Some(CONTEXT)),
        // Word deletion
        KeyBinding::new("alt-backspace", DeleteWordLeft, Some(CONTEXT)),
        KeyBinding::new("alt-delete", DeleteWordRight, Some(CONTEXT)),
        // Line deletion
        KeyBinding::new("cmd-backspace", DeleteToBeginning, Some(CONTEXT)),
        KeyBinding::new("cmd-delete", DeleteToEnd, Some(CONTEXT)),
        // Clipboard operations
        KeyBinding::new("cmd-c", Copy, Some(CONTEXT)),
        KeyBinding::new("cmd-v", Paste, Some(CONTEXT)),
        KeyBinding::new("cmd-x", Cut, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some(CONTEXT)),
        // Special features
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, Some(CONTEXT)),
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
        Clear,
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
    ]
);

/// State management for text input components
///
/// Handles text editing, cursor positioning, selection, and scrolling
/// for single-line text input fields.
pub struct InputState {
    pub(super) focus_handle: FocusHandle,
    pub(super) value: SharedString,
    pub(super) placeholder: SharedString,
    pub(super) placeholder_color: Hsla,
    pub(super) selected_range: Range<usize>,
    pub(super) selection_reversed: bool,
    pub(super) marked_range: Option<Range<usize>>,
    pub(super) last_layout: Option<ShapedLine>,
    pub(super) last_bounds: Option<Bounds<Pixels>>,
    pub(super) selecting: bool,
    pub(super) scroll_handle: ScrollHandle,
    pub(super) should_auto_scroll: bool,
}

impl InputState {
    // ============================================================================
    // Constructor and Builder Methods
    // ============================================================================

    /// Create a new InputState with default values
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            value: SharedString::new(""),
            placeholder: SharedString::new(""),
            placeholder_color: hsla(0., 0., 0.5, 0.5),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            selecting: false,
            scroll_handle: ScrollHandle::new(),
            should_auto_scroll: false,
        }
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the placeholder text color
    pub fn placeholder_color(mut self, color: impl Into<Hsla>) -> Self {
        self.placeholder_color = color.into();
        self
    }

    /// Set the initial value
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    // ============================================================================
    // Cursor Movement Actions
    // ============================================================================

    /// Move cursor left by one grapheme cluster
    pub(super) fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx);
        }
    }

    /// Move cursor right by one grapheme cluster
    pub(super) fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx);
        }
    }

    /// Move cursor left by one word
    pub(super) fn word_left(&mut self, _: &WordLeft, _: &mut Window, cx: &mut Context<Self>) {
        let new_offset = self.previous_word_boundary(self.cursor_offset());
        self.move_to(new_offset, cx);
    }

    /// Move cursor right by one word
    pub(super) fn word_right(&mut self, _: &WordRight, _: &mut Window, cx: &mut Context<Self>) {
        let new_offset = self.next_word_boundary(self.cursor_offset());
        self.move_to(new_offset, cx);
    }

    /// Move cursor to the beginning of the input
    pub(super) fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    /// Move cursor to the end of the input
    pub(super) fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.value.len(), cx);
    }

    /// Move cursor to a specific offset
    pub(super) fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = offset.clamp(0, self.value.len());
        self.selected_range = offset..offset;
        self.should_auto_scroll = true;
        cx.notify();
    }

    // ============================================================================
    // Text Selection Actions
    // ============================================================================

    /// Extend selection left by one grapheme cluster
    pub(super) fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    /// Extend selection right by one grapheme cluster
    pub(super) fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    /// Extend selection left by one word
    pub(super) fn select_word_left(
        &mut self,
        _: &SelectWordLeft,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_offset = self.previous_word_boundary(self.cursor_offset());
        self.select_to(new_offset, cx);
    }

    /// Extend selection right by one word
    pub(super) fn select_word_right(
        &mut self,
        _: &SelectWordRight,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_offset = self.next_word_boundary(self.cursor_offset());
        self.select_to(new_offset, cx);
    }

    /// Select from cursor to beginning of input
    pub(super) fn select_to_home(
        &mut self,
        _: &SelectToHome,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(0, cx);
    }

    /// Select from cursor to end of input
    pub(super) fn select_to_end(
        &mut self,
        _: &SelectToEnd,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(self.value.len(), cx);
    }

    /// Select all text in the input
    pub(super) fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.value.len(), cx);
    }

    /// Extend selection to a specific offset
    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset;
        } else {
            self.selected_range.end = offset;
        }

        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }

        self.should_auto_scroll = true;
        cx.notify();
    }

    /// Select the word at the given offset
    fn select_word(&mut self, offset: usize, cx: &mut Context<Self>) {
        let start = self.previous_word_boundary(offset);
        let end = self.next_word_boundary(offset);
        self.selected_range = start..end;
        self.selection_reversed = false;
        cx.notify();
    }

    // ============================================================================
    // Text Editing Actions
    // ============================================================================

    /// Delete character before cursor
    pub(super) fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    /// Delete character after cursor
    pub(super) fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    /// Paste text from clipboard
    pub(super) fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            // Replace newlines with spaces for single-line input
            self.replace_text_in_range(None, &text.replace('\n', " "), window, cx);
        }
    }

    /// Copy selected text to clipboard
    pub(super) fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            let selected_text = self.value[self.selected_range.clone()].to_string();
            cx.write_to_clipboard(ClipboardItem::new_string(selected_text));
        }
    }

    /// Cut selected text to clipboard
    pub(super) fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            let selected_text = self.value[self.selected_range.clone()].to_string();
            cx.write_to_clipboard(ClipboardItem::new_string(selected_text));
            self.replace_text_in_range(None, "", window, cx);
        }
    }

    /// Clear all text and reset state
    pub(super) fn clear(&mut self, _: &Clear, _: &mut Window, cx: &mut Context<Self>) {
        self.value = "".into();
        self.selected_range = 0..0;
        self.selection_reversed = false;
        self.marked_range = None;
        self.last_layout = None;
        self.last_bounds = None;
        self.selecting = false;
        self.should_auto_scroll = true;
        cx.notify();
    }

    /// Delete word to the left of cursor
    pub(super) fn delete_word_left(
        &mut self,
        _: &DeleteWordLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_range.is_empty() {
            let cursor_pos = self.cursor_offset();
            let word_start = self.previous_word_boundary(cursor_pos);
            self.selected_range = word_start..cursor_pos;
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    /// Delete word to the right of cursor
    pub(super) fn delete_word_right(
        &mut self,
        _: &DeleteWordRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_range.is_empty() {
            let cursor_pos = self.cursor_offset();
            let word_end = self.next_word_boundary(cursor_pos);
            self.selected_range = cursor_pos..word_end;
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    /// Delete from cursor to beginning of input
    pub(super) fn delete_to_beginning(
        &mut self,
        _: &DeleteToBeginning,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_range.is_empty() {
            let cursor_pos = self.cursor_offset();
            self.selected_range = 0..cursor_pos;
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    /// Delete from cursor to end of input
    pub(super) fn delete_to_end(
        &mut self,
        _: &DeleteToEnd,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_range.is_empty() {
            let cursor_pos = self.cursor_offset();
            self.selected_range = cursor_pos..self.value.len();
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    // ============================================================================
    // Mouse Event Handlers
    // ============================================================================

    /// Handle mouse down events for cursor positioning and text selection
    pub(super) fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selecting = true;

        // Handle multi-click selection
        if event.click_count > 1 {
            if event.click_count % 2 == 0 {
                // Double-click: select word
                self.select_word(self.index_for_mouse_position(event.position), cx);
            } else {
                // Triple-click: select all
                self.select_all(&SelectAll, window, cx);
            }
            return;
        }

        // Single click: position cursor or extend selection
        let mouse_offset = self.index_for_mouse_position(event.position);
        if event.modifiers.shift {
            self.select_to(mouse_offset, cx);
        } else {
            self.move_to(mouse_offset, cx);
        }
    }

    /// Handle mouse up events
    pub(super) fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.selecting = false;
    }

    /// Handle mouse move events for drag selection
    pub(super) fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    /// Clear input on left click (utility method)
    pub(super) fn left_click_clear(
        &mut self,
        _: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.clear(&Clear, window, cx);
    }

    // ============================================================================
    // System Integration
    // ============================================================================

    /// Show character palette
    pub(super) fn show_character_palette(
        &mut self,
        _: &ShowCharacterPalette,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        window.show_character_palette();
    }

    // ============================================================================
    // Scrolling Methods
    // ============================================================================

    /// Handle scroll wheel events
    pub(super) fn on_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let delta = event.delta.pixel_delta(window.line_height());
        let current_offset = self.scroll_handle.offset();
        let new_offset = current_offset + delta;
        self.update_scroll_offset(Some(new_offset), cx);
    }

    /// Update scroll offset with bounds checking
    fn update_scroll_offset(&mut self, offset: Option<Point<Pixels>>, cx: &mut Context<Self>) {
        let mut offset = offset.unwrap_or(self.scroll_handle.offset());

        // Constrain horizontal scrolling
        if let (Some(layout), Some(bounds)) = (self.last_layout.as_ref(), self.last_bounds.as_ref())
        {
            let text_width = layout.width;
            let visible_width = bounds.size.width;

            offset.x = offset.x.max(px(0.0));

            if text_width > visible_width {
                offset.x = offset.x.min(text_width - visible_width);
            } else {
                offset.x = px(0.0);
            }
        } else {
            offset.x = offset.x.max(px(0.0));
        }

        // Disable vertical scrolling for single-line input
        offset.y = px(0.0);

        self.scroll_handle.set_offset(offset);
        cx.notify();
    }

    /// Automatically scroll to keep cursor visible
    pub(super) fn auto_scroll_to_cursor(
        &mut self,
        layout: &ShapedLine,
        bounds: Bounds<Pixels>,
        _: &mut Context<Self>,
    ) {
        if !self.should_auto_scroll {
            return;
        }

        self.should_auto_scroll = false;

        let cursor_offset = self.cursor_offset();
        let cursor_x = layout.x_for_index(cursor_offset);
        let current_scroll = self.scroll_handle.offset();
        let visible_width = bounds.size.width;
        let visible_left = current_scroll.x;
        let visible_right = current_scroll.x + visible_width;

        let mut new_scroll_x = current_scroll.x;

        if cursor_x < visible_left {
            new_scroll_x = cursor_x.max(px(0.0));
        } else if cursor_x + px(CURSOR_WIDTH) >= visible_right {
            new_scroll_x = cursor_x - visible_width + px(CURSOR_WIDTH);
        }

        if new_scroll_x != current_scroll.x {
            let new_offset = point(new_scroll_x, current_scroll.y);
            self.scroll_handle.set_offset(new_offset);
        }
    }

    // ============================================================================
    // Text Boundary Helpers
    // ============================================================================

    /// Find the previous grapheme boundary
    fn previous_boundary(&self, offset: usize) -> usize {
        self.value
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    /// Find the next grapheme boundary
    fn next_boundary(&self, offset: usize) -> usize {
        self.value
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.value.len())
    }

    /// Find the previous word boundary
    fn previous_word_boundary(&self, offset: usize) -> usize {
        if offset == 0 {
            return 0;
        }

        let chars = self.value.char_indices().rev();
        let mut found_non_whitespace = false;

        for (idx, ch) in chars {
            if idx >= offset {
                continue;
            }

            if ch.is_whitespace() {
                if found_non_whitespace {
                    return idx + ch.len_utf8();
                }
            } else if ch.is_alphanumeric() || ch == '_' {
                found_non_whitespace = true;
            } else {
                if found_non_whitespace {
                    return idx + ch.len_utf8();
                }
                // Treat punctuation as individual words
                if idx + ch.len_utf8() < offset {
                    return idx + ch.len_utf8();
                }
            }
        }

        0
    }

    /// Find the next word boundary
    fn next_word_boundary(&self, offset: usize) -> usize {
        if offset >= self.value.len() {
            return self.value.len();
        }

        let chars = self.value.char_indices();
        let mut found_non_whitespace = false;
        let mut start_char_type = None;

        for (idx, ch) in chars {
            if idx < offset {
                continue;
            }

            if start_char_type.is_none() {
                start_char_type = Some(Self::char_type(ch));
                continue;
            }

            let current_type = Self::char_type(ch);

            match start_char_type.unwrap() {
                'w' => {
                    // Starting from whitespace
                    if current_type != 'w' {
                        found_non_whitespace = true;
                    } else if found_non_whitespace {
                        return idx;
                    }
                }
                'a' => {
                    // Starting from alphanumeric
                    if current_type != 'a' {
                        return idx;
                    }
                }
                'p' => {
                    // Starting from punctuation
                    return idx;
                }
                _ => {}
            }
        }

        self.value.len()
    }

    /// Classify character type for word boundary detection
    fn char_type(ch: char) -> char {
        if ch.is_whitespace() {
            'w' // whitespace
        } else if ch.is_alphanumeric() || ch == '_' {
            'a' // alphanumeric
        } else {
            'p' // punctuation
        }
    }

    // ============================================================================
    // Position and Index Calculation
    // ============================================================================

    /// Get the current cursor offset
    pub(super) fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    /// Calculate text index for mouse position
    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.value.is_empty() {
            return 0;
        }

        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };

        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.value.len();
        }

        let scroll_offset = self.scroll_handle.offset();
        line.closest_index_for_x(position.x - bounds.left() + scroll_offset.x)
    }

    // ============================================================================
    // UTF-16 Conversion Helpers
    // ============================================================================

    /// Convert UTF-8 offset to UTF-16 offset
    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for ch in self.value.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }

        utf16_offset
    }

    /// Convert UTF-16 offset to UTF-8 offset
    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in self.value.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    /// Convert UTF-8 range to UTF-16 range
    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    /// Convert UTF-16 range to UTF-8 range
    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl EntityInputHandler for InputState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.value[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _: &mut Window, _: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        let new_value = format!(
            "{}{}{}",
            &self.value[0..range.start],
            new_text,
            &self.value[range.end..]
        );

        self.value = new_value.into();
        let new_cursor_pos = range.start + new_text.len();
        self.selected_range = new_cursor_pos..new_cursor_pos;
        self.marked_range = None;
        self.should_auto_scroll = true;
        self.update_scroll_offset(None, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        let new_value = format!(
            "{}{}{}",
            &self.value[0..range.start],
            new_text,
            &self.value[range.end..]
        );

        self.value = new_value.into();

        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }

        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| (new_range.start + range.start)..(new_range.end + range.start))
            .unwrap_or_else(|| {
                let new_pos = range.start + new_text.len();
                new_pos..new_pos
            });

        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);

        Some(Bounds::from_corners(
            point(
                bounds.left() + last_layout.x_for_index(range.start),
                bounds.top(),
            ),
            point(
                bounds.left() + last_layout.x_for_index(range.end),
                bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&point)?;
        let last_layout = self.last_layout.as_ref()?;

        assert_eq!(last_layout.text, self.value);
        let utf8_index = last_layout.index_for_x(point.x - line_point.x)?;
        Some(self.offset_to_utf16(utf8_index))
    }
}

impl Focusable for InputState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for InputState {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("text-element")
            .flex_1()
            .flex_grow()
            .overflow_x_hidden()
            .child(TextElement::new(cx.entity().clone()))
    }
}

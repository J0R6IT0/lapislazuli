use crate::components::text_field::{
    cursor::Cursor,
    element::{CURSOR_WIDTH, TextElement},
    history::{Change, History},
    text_ops::TextOps,
    *,
};
use gpui::*;
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

pub struct InputEvent {
    pub value: SharedString,
}

pub struct ChangeEvent {
    pub value: SharedString,
}

/// State management for text field components
///
/// Handles text editing, cursor positioning, selection, and scrolling
/// for single-line text fields.
pub struct TextFieldState {
    pub(super) focus_handle: FocusHandle,
    pub(super) value: SharedString,
    pub(super) emitted_value: SharedString,
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
    pub(super) cursor: Entity<Cursor>,
    pub(super) masked: bool,
    pub(super) mask: SharedString,
    history: History,
    ignore_history: bool,
    _subscriptions: Vec<Subscription>,
}

impl TextFieldState {
    // ============================================================================
    // Constructor and Builder Methods
    // ============================================================================

    /// Create a new [`TextFieldState`] with default values
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let cursor = cx.new(|_| Cursor::new());
        let focus_handle = cx.focus_handle();

        let _subscriptions = vec![
            cx.observe(&cursor, |_, _, cx| cx.notify()),
            cx.observe_window_activation(window, |state, window, cx| {
                if window.is_window_active() {
                    let focus_handle = state.focus_handle.clone();
                    if focus_handle.is_focused(window) {
                        state.cursor.update(cx, |cursor, cx| {
                            cursor.start(cx);
                        });
                    }
                }
            }),
            cx.on_focus(&focus_handle, window, Self::on_focus),
            cx.on_blur(&focus_handle, window, Self::on_blur),
        ];

        Self {
            focus_handle,
            value: SharedString::new(""),
            emitted_value: SharedString::new(""),
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
            masked: false,
            mask: SharedString::new("•"),
            history: History::new(),
            ignore_history: false,
            cursor,
            _subscriptions,
        }
    }

    /// Set the placeholder text
    pub fn set_placeholder(&mut self, placeholder: impl Into<SharedString>) {
        self.placeholder = placeholder.into();
    }

    /// Set the placeholder text color
    pub fn set_placeholder_color(&mut self, color: impl Into<Hsla>) {
        self.placeholder_color = color.into();
    }

    /// Set the value of the text field
    pub fn set_value(&mut self, value: impl Into<SharedString>) {
        self.value = value.into();
        self.emitted_value = self.value.clone();
        self.history.clear();
    }

    /// Whether the text field is masked (e.g., for passwords)
    pub fn is_masked(&self) -> bool {
        self.masked
    }

    /// Mask or unmask the text field (e.g., for passwords)
    pub fn set_masked(&mut self, masked: bool) {
        if self.masked != masked {
            self.masked = masked;
            self.should_auto_scroll = true;
        }
    }

    /// Set the mask string to use when masking is enabled
    ///
    /// Each character in the actual text will be replaced with the entire mask string
    /// when masking is enabled.
    pub fn set_mask(&mut self, mask: impl Into<SharedString>) {
        let mask = mask.into();
        if self.mask != mask {
            self.mask = mask;
            if self.masked {
                self.should_auto_scroll = true;
            }
        }
    }

    fn on_focus(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.cursor.update(cx, |cursor, cx| {
            cursor.start(cx);
        });
    }

    fn on_blur(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.history.prevent_merge();
        self.cursor.update(cx, |cursor, cx| {
            cursor.stop(cx);
        });
        self.on_change(cx);
    }

    fn on_change(&mut self, cx: &mut Context<Self>) {
        if self.value == self.emitted_value {
            return;
        }

        self.emitted_value = self.value.clone();
        cx.emit(ChangeEvent {
            value: self.value.clone(),
        });
    }

    fn pause_cursor_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor.update(cx, |cursor, cx| {
            cursor.pause(cx);
        });
    }

    pub(crate) fn cursor_visible(&self, window: &Window, app: &App) -> bool {
        self.focus_handle.is_focused(window) && self.cursor.read(app).visible()
    }

    // ============================================================================
    // Cursor Movement Actions
    // ============================================================================

    /// Move cursor left by one grapheme cluster
    pub(super) fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(
                TextOps::previous_boundary(&self.value, self.cursor_offset()),
                cx,
            );
        } else {
            self.move_to(self.selected_range.start, cx);
        }
    }

    /// Move cursor right by one grapheme cluster
    pub(super) fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(
                TextOps::next_boundary(&self.value, self.selected_range.end),
                cx,
            );
        } else {
            self.move_to(self.selected_range.end, cx);
        }
    }

    /// Move cursor left by one word
    pub(super) fn word_left(&mut self, _: &WordLeft, _: &mut Window, cx: &mut Context<Self>) {
        let new_offset = TextOps::previous_word_boundary(&self.value, self.cursor_offset());
        self.move_to(new_offset, cx);
    }

    /// Move cursor right by one word
    pub(super) fn word_right(&mut self, _: &WordRight, _: &mut Window, cx: &mut Context<Self>) {
        let new_offset = TextOps::next_word_boundary(&self.value, self.cursor_offset());
        self.move_to(new_offset, cx);
    }

    /// Move cursor to the beginning of the field
    pub(super) fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    /// Move cursor to the end of the field
    pub(super) fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.value.len(), cx);
    }

    /// Move cursor to a specific offset
    pub(super) fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.pause_cursor_blink(cx);
        let offset = offset.clamp(0, self.value.len());
        if offset == self.cursor_offset() {
            return;
        }
        self.selected_range = offset..offset;
        self.should_auto_scroll = true;
        self.history.prevent_merge();
        cx.notify();
    }

    // ============================================================================
    // Text Selection Actions
    // ============================================================================

    /// Extend selection left by one grapheme cluster
    pub(super) fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(
            TextOps::previous_boundary(&self.value, self.cursor_offset()),
            cx,
        );
    }

    /// Extend selection right by one grapheme cluster
    pub(super) fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(
            TextOps::next_boundary(&self.value, self.cursor_offset()),
            cx,
        );
    }

    /// Extend selection left by one word
    pub(super) fn select_word_left(
        &mut self,
        _: &SelectWordLeft,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_offset = TextOps::previous_word_boundary(&self.value, self.cursor_offset());
        self.history.prevent_merge();
        self.select_to(new_offset, cx);
    }

    /// Extend selection right by one word
    pub(super) fn select_word_right(
        &mut self,
        _: &SelectWordRight,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_offset = TextOps::next_word_boundary(&self.value, self.cursor_offset());
        self.history.prevent_merge();
        self.select_to(new_offset, cx);
    }

    /// Select from cursor to beginning of field
    pub(super) fn select_to_home(
        &mut self,
        _: &SelectToHome,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(0, cx);
    }

    /// Select from cursor to end of field
    pub(super) fn select_to_end(
        &mut self,
        _: &SelectToEnd,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(self.value.len(), cx);
    }

    /// Select all text in the field
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
        let start = TextOps::previous_word_boundary(&self.value, offset);
        let end = TextOps::next_word_boundary(&self.value, offset);
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
            self.select_to(
                TextOps::previous_boundary(&self.value, self.cursor_offset()),
                cx,
            );
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    /// Delete character after cursor
    pub(super) fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(
                TextOps::next_boundary(&self.value, self.cursor_offset()),
                cx,
            );
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    /// Paste text from clipboard
    pub(super) fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.history.prevent_merge();
            // Replace newlines with spaces for single-line text fields
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
            self.history.prevent_merge();
            self.replace_text_in_range(None, "", window, cx);
        }
    }

    pub(super) fn undo(&mut self, _: &Undo, window: &mut Window, cx: &mut Context<Self>) {
        self.ignore_history = true;
        if let Some(change) = self.history.undo() {
            self.replace_text_in_range(
                Some(TextOps::range_to_utf16(&self.value, &change.range())),
                &change.text(),
                window,
                cx,
            );
            self.selected_range = change.selection_range();
        }
        self.ignore_history = false;
    }

    pub(super) fn redo(&mut self, _: &Redo, window: &mut Window, cx: &mut Context<Self>) {
        self.ignore_history = true;
        if let Some(change) = self.history.redo() {
            self.replace_text_in_range(
                Some(TextOps::range_to_utf16(&self.value, &change.range())),
                &change.text(),
                window,
                cx,
            );
        }
        self.ignore_history = false;
    }

    fn push_history(&mut self, new_text: &str, range: &Range<usize>) {
        if self.ignore_history {
            return;
        }

        if range.start == 0 && range.end == 0 && new_text.is_empty() {
            return;
        }

        let marked = self.marked_range.is_some();

        if range.start == range.end {
            self.history.push(Change::Insert {
                range: range.clone(),
                text: new_text.to_string().into(),
            });
        } else if new_text.is_empty() {
            self.history.push(Change::Delete {
                range: range.clone(),
                text: self.value[range.start..range.end].to_string().into(),
            })
        } else {
            self.history.push(Change::Replace {
                range: range.clone(),
                new_text: new_text.to_string().into(),
                old_text: self.value[range.start..range.end].to_string().into(),
                marked,
            });
        }
    }

    /// Clear all text and reset state
    pub fn clear(&mut self) {
        self.value = "".into();
        self.selected_range = 0..0;
        self.selection_reversed = false;
        self.marked_range = None;
        self.last_layout = None;
        self.last_bounds = None;
        self.selecting = false;
        self.should_auto_scroll = true;
        self.scroll_handle.set_offset(point(px(0.0), px(0.0)));
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
            let word_start = TextOps::previous_word_boundary(&self.value, cursor_pos);
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
            let word_end = TextOps::next_word_boundary(&self.value, cursor_pos);
            self.selected_range = cursor_pos..word_end;
        }
        self.history.prevent_merge();
        self.replace_text_in_range(None, "", window, cx);
    }

    /// Delete from cursor to beginning of text field
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

    /// Delete from cursor to end of text field
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

    pub(super) fn enter(&mut self, _: &Enter, _: &mut Window, cx: &mut Context<Self>) {
        self.on_change(cx);
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
        cx.stop_propagation();
        let delta = event.delta.pixel_delta(window.line_height());
        let current_offset = self.scroll_handle.offset();
        let new_offset = current_offset - delta;
        self.update_scroll_offset(Some(new_offset), cx);
    }

    /// Update scroll offset with bounds checking
    fn update_scroll_offset(&mut self, offset: Option<Point<Pixels>>, cx: &mut Context<Self>) {
        let mut offset = offset.unwrap_or(self.scroll_handle.offset());

        // Constrain horizontal scrolling
        if let (Some(layout), Some(bounds)) = (self.last_layout.as_ref(), self.last_bounds.as_ref())
        {
            let text_width = layout.width;
            let visible_width = bounds.size.width - px(CURSOR_WIDTH);

            offset.x = offset.x.max(px(0.0));

            if text_width > visible_width {
                offset.x = offset.x.min(text_width - visible_width);
            } else {
                offset.x = px(0.0);
            }
        } else {
            offset.x = offset.x.max(px(0.0));
        }

        // Disable vertical scrolling for single-line text fields
        offset.y = px(0.0);

        self.scroll_handle.set_offset(offset);
        cx.notify();
    }

    /// Automatically scroll to keep cursor visible
    pub(super) fn auto_scroll_to_cursor(&mut self, layout: &ShapedLine, bounds: Bounds<Pixels>) {
        self.should_auto_scroll = false;

        let cursor_offset = self.display_cursor_offset();
        let cursor_x = layout.x_for_index(cursor_offset);
        let current_scroll = self.scroll_handle.offset();
        let visible_width = bounds.size.width - px(CURSOR_WIDTH);
        let text_width = layout.width;
        let visible_left = current_scroll.x;
        let visible_right = current_scroll.x + visible_width;

        let mut new_scroll_x = current_scroll.x;

        if cursor_x < visible_left {
            new_scroll_x = cursor_x.max(px(0.0));
        } else if cursor_x >= visible_right {
            new_scroll_x = cursor_x - visible_width;
        }

        // Ensure no blank space is shown when text fits or becomes shorter
        if text_width <= visible_width {
            // Text fits entirely, show from beginning
            new_scroll_x = px(0.0);
        } else {
            // Text is longer than visible area
            // Ensure we don't scroll past the end, leaving blank space
            let max_scroll = text_width - visible_width;
            new_scroll_x = new_scroll_x.min(max_scroll).max(px(0.0));
        }

        if new_scroll_x != current_scroll.x {
            let new_offset = point(new_scroll_x, current_scroll.y);
            self.scroll_handle.set_offset(new_offset);
        }
    }

    // ============================================================================
    // Position and Index Calculation
    // ============================================================================

    /// Get the current cursor offset
    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    /// Get the current cursor offset in display text coordinates
    pub(super) fn display_cursor_offset(&self) -> usize {
        self.actual_to_display_offset(self.cursor_offset())
    }

    /// Convert actual text range to display text range for masked text fields
    pub(super) fn display_selection_range(&self) -> std::ops::Range<usize> {
        let start = self.actual_to_display_offset(self.selected_range.start);
        let end = self.actual_to_display_offset(self.selected_range.end);
        start..end
    }

    /// Convert actual text offset to display text offset
    fn actual_to_display_offset(&self, actual_offset: usize) -> usize {
        if !self.masked {
            return actual_offset;
        }

        if let Some(marked_range) = &self.marked_range {
            if actual_offset <= marked_range.start {
                // Before marked range: count graphemes and multiply by mask length
                let grapheme_count = self.value[..actual_offset].graphemes(true).count();
                grapheme_count * self.mask.len()
            } else if actual_offset <= marked_range.end {
                // Inside marked range: masked graphemes before + unmarked bytes within
                let before_graphemes = self.value[..marked_range.start].graphemes(true).count();
                before_graphemes * self.mask.len() + (actual_offset - marked_range.start)
            } else {
                // After marked range: before masked + marked bytes + after masked
                let before_graphemes = self.value[..marked_range.start].graphemes(true).count();
                let after_graphemes = self.value[marked_range.end..actual_offset]
                    .graphemes(true)
                    .count();
                before_graphemes * self.mask.len()
                    + (marked_range.end - marked_range.start)
                    + after_graphemes * self.mask.len()
            }
        } else {
            // No marked text: count graphemes and multiply by mask length
            let grapheme_count = self.value[..actual_offset].graphemes(true).count();
            grapheme_count * self.mask.len()
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

        let scroll_offset = self.scroll_handle.offset();
        let display_index = line.closest_index_for_x(position.x - bounds.left() + scroll_offset.x);
        self.display_to_actual_offset(display_index)
    }

    /// Convert display text offset back to actual text offset
    fn display_to_actual_offset(&self, display_offset: usize) -> usize {
        if !self.masked || self.mask.is_empty() {
            return display_offset;
        }

        let mask_len = self.mask.len();

        if let Some(marked_range) = &self.marked_range {
            let before_graphemes = self.value[..marked_range.start].graphemes(true).count();
            let masked_before_end = before_graphemes * mask_len;
            let marked_end = masked_before_end + (marked_range.end - marked_range.start);

            if display_offset <= masked_before_end {
                // In masked text before marked range - find grapheme boundary
                let target_grapheme = display_offset / mask_len;
                TextOps::grapheme_offset_to_byte_offset(
                    &self.value,
                    target_grapheme.min(before_graphemes),
                )
            } else if display_offset <= marked_end {
                // In unmarked marked range
                marked_range.start + (display_offset - masked_before_end)
            } else {
                // In masked text after marked range - find grapheme boundary
                let after_display = display_offset - marked_end;
                let target_after_grapheme = after_display / mask_len;
                let after_graphemes = self.value[marked_range.end..].graphemes(true).count();
                let actual_after_grapheme = target_after_grapheme.min(after_graphemes);

                // Convert grapheme index to byte offset from marked_range.end
                let after_byte_offset = self.value[marked_range.end..]
                    .grapheme_indices(true)
                    .nth(actual_after_grapheme)
                    .map(|(i, _)| i)
                    .unwrap_or(self.value.len() - marked_range.end);

                marked_range.end + after_byte_offset
            }
        } else {
            // No marked text: find grapheme boundary
            let target_grapheme = display_offset / mask_len;
            let total_graphemes = self.value.graphemes(true).count();
            TextOps::grapheme_offset_to_byte_offset(
                &self.value,
                target_grapheme.min(total_graphemes),
            )
        }
    }
}

impl EntityInputHandler for TextFieldState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let range = TextOps::range_from_utf16(&self.value, &range_utf16);
        actual_range.replace(TextOps::range_to_utf16(&self.value, &range));
        Some(self.value[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: TextOps::range_to_utf16(&self.value, &self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| TextOps::range_to_utf16(&self.value, range))
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
        self.pause_cursor_blink(cx);
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| TextOps::range_from_utf16(&self.value, range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.push_history(new_text, &range);

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
        self.last_layout = None;
        self.last_bounds = None;

        cx.emit(InputEvent {
            value: self.value.clone(),
        });
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
            .map(|range_utf16| TextOps::range_from_utf16(&self.value, range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.push_history(new_text, &range);

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
            .map(|range_utf16| TextOps::range_from_utf16(&self.value, range_utf16))
            .map(|new_range| (new_range.start + range.start)..(new_range.end + range.start))
            .unwrap_or_else(|| {
                let new_pos = range.start + new_text.len();
                new_pos..new_pos
            });

        self.should_auto_scroll = true;
        cx.emit(InputEvent {
            value: self.value.clone(),
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
        let range = TextOps::range_from_utf16(&self.value, &range_utf16);

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

        let utf8_index = last_layout.index_for_x(point.x - line_point.x)?;
        Some(TextOps::offset_to_utf16(&self.value, utf8_index))
    }
}

impl Focusable for TextFieldState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<InputEvent> for TextFieldState {}
impl EventEmitter<ChangeEvent> for TextFieldState {}

impl Render for TextFieldState {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("text-element")
            .flex_1()
            .flex_grow()
            .overflow_x_hidden()
            .child(TextElement::new(cx.entity().clone()))
    }
}

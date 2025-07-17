use crate::components::text_field::state::TextFieldState;
use gpui::*;
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

pub const CURSOR_WIDTH: f32 = 1.0;
const MARKED_TEXT_UNDERLINE_THICKNESS: f32 = 1.0;
const SELECTION_COLOR: u32 = 0x3390FF80;

/// A text input element that renders editable text with cursor and selection support.
///
/// This element handles:
/// - Text rendering with proper font styling
/// - Cursor positioning and visibility
/// - Text selection highlighting
/// - Automatic scrolling to keep cursor visible
/// - Placeholder text when empty
/// - Marked text (IME composition) with underlines
pub struct TextElement {
    state: Entity<TextFieldState>,
}

impl TextElement {
    pub fn new(state: Entity<TextFieldState>) -> Self {
        Self { state }
    }
}

pub struct PrepaintState {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl TextElement {
    fn prepare_display_text(&self, state: &TextFieldState, text_color: Hsla) -> (SharedString, Hsla) {
        if state.value.is_empty() {
            return (state.placeholder.clone(), state.placeholder_color);
        }

        if !state.masked {
            return (state.value.clone(), text_color);
        }

        if state.mask.is_empty() {
            return (SharedString::from(""), text_color);
        }

        let committed_grapheme_count = if let Some(marked_range) = &state.marked_range {
            let before_count = state.value[..marked_range.start].graphemes(true).count();
            let after_count = state.value[marked_range.end..].graphemes(true).count();
            before_count + after_count
        } else {
            state.value.graphemes(true).count()
        };

        (
            state.mask.repeat(committed_grapheme_count).into(),
            text_color,
        )
    }

    fn create_text_runs(
        &self,
        display_text: &str,
        base_run: TextRun,
        marked_range: Option<&Range<usize>>,
        is_masked: bool,
    ) -> Vec<TextRun> {
        // For masked text, we've already excluded marked text from display_text,
        // so no need for marked text styling
        if is_masked || marked_range.is_none() {
            return vec![base_run];
        }

        if let Some(marked_range) = marked_range {
            // Ensure marked_range doesn't exceed display_text bounds
            let display_len = display_text.len();
            if marked_range.start >= display_len || marked_range.end > display_len {
                return vec![base_run];
            }

            vec![
                TextRun {
                    len: marked_range.start,
                    ..base_run.clone()
                },
                TextRun {
                    len: marked_range.end - marked_range.start,
                    underline: Some(UnderlineStyle {
                        color: Some(base_run.color),
                        thickness: px(MARKED_TEXT_UNDERLINE_THICKNESS),
                        wavy: false,
                    }),
                    ..base_run.clone()
                },
                TextRun {
                    len: display_len - marked_range.end,
                    ..base_run.clone()
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![base_run]
        }
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        app: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let style = Style {
            size: Size {
                width: relative(1.).into(),
                height: window.line_height().into(),
            },
            ..Style::default()
        };
        (window.request_layout(style, [], app), ())
    }
    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        app: &mut App,
    ) -> Self::PrepaintState {
        let state = self.state.read(app);
        let style = window.text_style();

        let (display_text, text_color) = self.prepare_display_text(&state, style.color);

        let base_run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };

        let runs = self.create_text_runs(
            &display_text,
            base_run,
            state.marked_range.as_ref(),
            state.masked,
        );

        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs);

        if state.should_auto_scroll {
            self.state.update(app, |input, _| {
                input.auto_scroll_to_cursor(&line, bounds);
            });
        }

        let state = self.state.read(app);
        let scroll_offset = state.scroll_handle.offset();
        let cursor_pos = line.x_for_index(state.display_cursor_offset());

        let (selection, cursor) = if state.selected_range.is_empty() {
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(bounds.left() + cursor_pos - scroll_offset.x, bounds.top()),
                        size(px(CURSOR_WIDTH), bounds.bottom() - bounds.top()),
                    ),
                    text_color,
                )),
            )
        } else {
            let selection_range = state.display_selection_range();
            (
                Some(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + line.x_for_index(selection_range.start)
                                - scroll_offset.x,
                            bounds.top(),
                        ),
                        point(
                            bounds.left() + line.x_for_index(selection_range.end) - scroll_offset.x,
                            bounds.bottom(),
                        ),
                    ),
                    rgba(SELECTION_COLOR),
                )),
                None,
            )
        };

        PrepaintState {
            line: Some(line),
            cursor,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        app: &mut App,
    ) {
        let state = self.state.read(app);
        let focus_handle = state.focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.state.clone()),
            app,
        );

        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection);
        }

        let line = prepaint.line.take().unwrap();
        let scroll_offset = state.scroll_handle.offset();
        let text_origin = point(bounds.origin.x - scroll_offset.x, bounds.origin.y);

        line.paint(text_origin, window.line_height(), window, app)
            .unwrap();

        if focus_handle.is_focused(window) && self.state.read(app).cursor_visible(window, app) {
            if let Some(cursor) = prepaint.cursor.take() {
                window.paint_quad(cursor);
            }
        }

        self.state.update(app, |input, _cx| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}

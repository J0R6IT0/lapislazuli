use crate::components::input::state::InputState;
use gpui::*;

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
    input: Entity<InputState>,
}

impl TextElement {
    pub fn new(input: Entity<InputState>) -> Self {
        Self { input }
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
    /// Creates the layout style for the text input
    fn create_layout_style(&self, window: &Window) -> Style {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = (window.line_height()).into();
        style
    }

    /// Paints the text line at the specified origin
    fn paint_text(
        &self,
        line: ShapedLine,
        text_origin: Point<Pixels>,
        window: &mut Window,
        app: &mut App,
    ) {
        line.paint(text_origin, window.line_height(), window, app)
            .unwrap();
    }

    /// Prepares the display text and color based on content and placeholder
    fn prepare_display_text(&self, input: &InputState, text_color: Hsla) -> (SharedString, Hsla) {
        let display_text = input.display_text();
        let color = if input.value.is_empty() {
            input.placeholder_color
        } else {
            text_color
        };
        (display_text, color)
    }

    /// Creates text runs with proper styling including marked text underlines
    fn create_text_runs(
        &self,
        display_text: &str,
        base_run: TextRun,
        marked_range: Option<&std::ops::Range<usize>>,
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

    /// Creates cursor paint quad for rendering
    fn create_cursor_quad(
        &self,
        bounds: Bounds<Pixels>,
        cursor_pos: Pixels,
        scroll_offset: Point<Pixels>,
        text_color: Hsla,
    ) -> PaintQuad {
        fill(
            Bounds::new(
                point(bounds.left() + cursor_pos - scroll_offset.x, bounds.top()),
                size(px(CURSOR_WIDTH), bounds.bottom() - bounds.top()),
            ),
            text_color,
        )
    }

    /// Creates selection paint quad for rendering
    fn create_selection_quad(
        &self,
        bounds: Bounds<Pixels>,
        line: &ShapedLine,
        selected_range: &std::ops::Range<usize>,
        scroll_offset: Point<Pixels>,
    ) -> PaintQuad {
        fill(
            Bounds::from_corners(
                point(
                    bounds.left() + line.x_for_index(selected_range.start) - scroll_offset.x,
                    bounds.top(),
                ),
                point(
                    bounds.left() + line.x_for_index(selected_range.end) - scroll_offset.x,
                    bounds.bottom(),
                ),
            ),
            rgba(SELECTION_COLOR),
        )
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
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        app: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let style = self.create_layout_style(window);
        (window.request_layout(style, [], app), ())
    }
    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        app: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(app);
        let style = window.text_style();

        let (display_text, text_color) = self.prepare_display_text(&input, style.color);

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
            input.marked_range.as_ref(),
            input.is_masked(),
        );

        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs);

        self.input.update(app, |input, _| {
            input.auto_scroll_to_cursor(&line, bounds);
        });

        let input = self.input.read(app);
        let scroll_offset = input.scroll_handle.offset();
        let cursor_pos = line.x_for_index(input.display_cursor_offset());

        let (selection, cursor) = if input.selected_range.is_empty() {
            (
                None,
                Some(self.create_cursor_quad(bounds, cursor_pos, scroll_offset, style.color)),
            )
        } else {
            (
                Some(self.create_selection_quad(
                    bounds,
                    &line,
                    &input.display_selection_range(),
                    scroll_offset,
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
        let focus_handle = self.input.read(app).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            app,
        );

        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection);
        }

        let line = prepaint.line.take().unwrap();
        let scroll_offset = self.input.read(app).scroll_handle.offset();
        let text_origin = point(bounds.origin.x - scroll_offset.x, bounds.origin.y);
        self.paint_text(line.clone(), text_origin, window, app);

        if focus_handle.is_focused(window) && self.input.read(app).cursor_visible(window, app) {
            if let Some(cursor) = prepaint.cursor.take() {
                window.paint_quad(cursor);
            }
        }

        self.input.update(app, |input, _cx| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}

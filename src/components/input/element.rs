use crate::components::input::state::InputState;
use gpui::*;

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
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = (window.line_height()).into();
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
        let content = input.value.clone();
        let style = window.text_style();

        let (display_text, text_color) = if content.is_empty() {
            (input.placeholder.clone(), input.placeholder_color)
        } else {
            (content.clone(), style.color)
        };

        let run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let runs = if let Some(marked_range) = input.marked_range.as_ref() {
            vec![
                TextRun {
                    len: marked_range.start,
                    ..run.clone()
                },
                TextRun {
                    len: marked_range.end - marked_range.start,
                    underline: Some(UnderlineStyle {
                        color: Some(run.color),
                        thickness: px(1.0),
                        wavy: false,
                    }),
                    ..run.clone()
                },
                TextRun {
                    len: display_text.len() - marked_range.end,
                    ..run.clone()
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![run]
        };

        let font_size = style.font_size.to_pixels(window.rem_size());
        let text_color = style.color;
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs);

        // Drop input reference before calling auto_scroll_to_cursor
        let _ = input;

        // Handle auto-scrolling if needed
        self.input.update(app, |input, cx| {
            input.auto_scroll_to_cursor(&line, bounds, cx);
        });

        let scroll_offset = self.input.read(app).scroll_handle.offset();

        // Get input reference again for cursor calculations
        let input = self.input.read(app);
        let cursor_pos = line.x_for_index(input.cursor_offset());

        let (selection, cursor) = if input.selected_range.is_empty() {
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(bounds.left() + cursor_pos - scroll_offset.x, bounds.top()),
                        size(px(1.), bounds.bottom() - bounds.top()),
                    ),
                    text_color,
                )),
            )
        } else {
            (
                Some(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + line.x_for_index(input.selected_range.start)
                                - scroll_offset.x,
                            bounds.top(),
                        ),
                        point(
                            bounds.left() + line.x_for_index(input.selected_range.end)
                                - scroll_offset.x,
                            bounds.bottom(),
                        ),
                    ),
                    rgb(0x3311ff),
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
            window.paint_quad(selection)
        }
        let line = prepaint.line.take().unwrap();
        let scroll_offset = self.input.read(app).scroll_handle.offset();
        let text_origin = point(bounds.origin.x - scroll_offset.x, bounds.origin.y);
        line.paint(text_origin, window.line_height(), window, app)
            .unwrap();

        if focus_handle.is_focused(window) {
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

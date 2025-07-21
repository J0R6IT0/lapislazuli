use gpui::{
    Animation, AnimationExt, App, AppContext, Application, Context, Entity, FocusHandle, Focusable,
    FontWeight, InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, Window, WindowOptions, div, px, relative, rems, rgb, rgba,
};
use lapislazuli::{
    AutoFocusable, Disableable, LapislazuliProvider, ParentElementWithContext,
    components::{
        Checkbox, Switch,
        progress::{Progress, ProgressFill, ProgressTrack},
        tabs::{Tabs, TabsTrigger},
    },
    primitives::{
        a, button, h_flex, span,
        text_field::{ChangeEvent, InputEvent, TextFieldState, text_field},
        v_flex,
    },
};
use std::time::Duration;

struct Showcase {
    progress_value: f32,
    previous_progress_value: f32,
    text_state: Entity<TextFieldState>,
    focus_handle: FocusHandle,
    disabled: bool,
    button_click_count: u32,
    selected_tab_index: usize,
}

impl Focusable for Showcase {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Showcase {
    fn new(window: &mut Window, app: &mut App) -> Entity<Self> {
        let text_state = app.new(|cx| {
            let mut state = TextFieldState::new(window, cx);
            state.set_placeholder("Try typing something here...");
            state.set_placeholder_color(rgb(0x9ca3af));
            state.set_mask("😎");
            state.set_max_length(Some(10));
            state.set_validator(|v| v.contains("apple"));
            state
        });

        app.new(|cx| {
            cx.subscribe(&text_state, |_showcase, _state, event: &InputEvent, _cx| {
                println!("On Input: {}", event.value);
            })
            .detach();

            cx.subscribe(
                &text_state,
                |_showcase, _state, event: &ChangeEvent, _cx| {
                    println!("On Change: {}", event.value);
                },
            )
            .detach();

            Self {
                text_state,
                focus_handle: cx.focus_handle(),
                progress_value: 65.0,
                previous_progress_value: 65.0,
                disabled: false,
                button_click_count: 0,
                selected_tab_index: 0,
            }
        })
    }

    fn increment_progress<T>(&mut self, _event: &T, _window: &mut Window, cx: &mut Context<Self>) {
        self.previous_progress_value = self.progress_value;
        self.progress_value = (self.progress_value + 5.0).min(100.0);
        self.button_click_count += 1;
        cx.notify();
    }

    fn decrement_progress<T>(&mut self, _event: &T, _window: &mut Window, cx: &mut Context<Self>) {
        self.previous_progress_value = self.progress_value;
        self.progress_value = (self.progress_value - 5.0).max(0.0);
        self.button_click_count += 1;
        cx.notify();
    }

    fn reset_progress<T>(&mut self, _event: &T, _window: &mut Window, cx: &mut Context<Self>) {
        self.previous_progress_value = self.progress_value;
        self.progress_value = 0.0;
        self.button_click_count += 1;
        cx.notify();
    }

    fn complete_progress<T>(&mut self, _event: &T, _window: &mut Window, cx: &mut Context<Self>) {
        self.previous_progress_value = self.progress_value;
        self.progress_value = 100.0;
        self.button_click_count += 1;
        cx.notify();
    }

    fn toggle_disabled<T>(&mut self, _event: &T, _window: &mut Window, cx: &mut Context<Self>) {
        self.disabled = !self.disabled;
        cx.notify();
    }

    fn set_disabled(&mut self, disabled: &bool, _window: &mut Window, cx: &mut Context<Self>) {
        self.disabled = *disabled;
        cx.notify();
    }

    fn reset_counter<T>(&mut self, _event: &T, _window: &mut Window, cx: &mut Context<Self>) {
        self.button_click_count = 0;
        cx.notify();
    }
}

impl Render for Showcase {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let progress_color = if self.progress_value >= 100.0 {
            rgb(0x10b981)
        } else if self.progress_value >= 70.0 {
            rgb(0x3b82f6)
        } else if self.progress_value >= 40.0 {
            rgb(0xf59e0b)
        } else {
            rgb(0xef4444)
        };

        let disabled = self.disabled;

        v_flex()
            .id("showcase")
            .overflow_scroll()
            .h_full()
            .relative()
            .font_family(".SystemUIFont")
            .track_focus(&self.focus_handle(cx))
            .bg(rgb(0xf8fafc))
            .min_h_full()
            .p(rems(3.0))
            .gap(rems(2.5))
            .child(
                v_flex()
                    .gap(rems(0.5))
                    .child(
                        span("🌟 lapislazuli Component Showcase")
                            .text_size(rems(2.5))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x1e293b))
                    )
                    .child(
                        span("A headless component library for GPUI")
                            .text_size(rems(1.1))
                            .text_color(rgb(0x64748b))
                    )
            )
            .child(
                v_flex()
                    .gap(rems(1.5))
                    .child(
                        span("Tabs Component")
                            .text_size(rems(1.5))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x1e293b))
                    )
                    .child(
                        Tabs::new("showcase-tabs")
                            .value(self.selected_tab_index)
                            .list(|list| {
                                list
                                    .trigger(
                                    TabsTrigger::new()
                                        .child(span("Overview"))
                                        .px(rems(1.0))
                                        .py(rems(0.5))
                                        .border_b_2()
                                        .border_color(rgba(0x00000000))
                                        .text_color(rgb(0x64748b))
                                        .when_selected(|this| {this.border_color(rgb(0x3b82f6)).text_color(rgb(0x3b82f6))})
                                )
                                .trigger(
                                    TabsTrigger::new()
                                        .child(span("Components"))
                                        .px(rems(1.0))
                                        .py(rems(0.5))
                                        .border_b_2()
                                        .border_color(rgba(0x00000000))
                                        .text_color(rgb(0x64748b))
                                        .when_selected(|this| {this.border_color(rgb(0x3b82f6)).text_color(rgb(0x3b82f6))})
                                )
                                .trigger(
                                    TabsTrigger::new()
                                        .child(span("Settings"))
                                        .px(rems(1.0))
                                        .py(rems(0.5))
                                        .border_b_2()
                                        .border_color(rgba(0x00000000))
                                        .text_color(rgb(0x64748b))
                                        .when_selected(|this| {this.border_color(rgb(0x3b82f6)).text_color(rgb(0x3b82f6))})
                                )
                            })
                            .on_change(cx.listener(|this, index, _window, _cx| {
                                this.selected_tab_index = *index;
                            }))
                    )
                    .child(
                        match self.selected_tab_index {
                            0 => v_flex()
                                .gap(rems(1.0))
                                .child(span("Welcome to the lapislazuli showcase!").text_color(rgb(0x1e293b)))
                                .child(span("This is a headless component library for GPUI.").text_color(rgb(0x64748b)))
                                .into_any_element(),
                            1 => v_flex()
                                .gap(rems(1.0))
                                .child(span("Components Tab").text_color(rgb(0x1e293b)))
                                .child(span("Here you can see all the available components.").text_color(rgb(0x64748b)))
                                .into_any_element(),
                            2 => v_flex()
                                .gap(rems(1.0))
                                .child(span("Settings Tab").text_color(rgb(0x1e293b)))
                                .child(span("Configure your preferences here.").text_color(rgb(0x64748b)))
                                .into_any_element(),
                            _ => v_flex().into_any_element(),
                        }
                    )
            )
            .child(
                h_flex()
                    .gap(rems(1.5))
                    .child(
                        v_flex()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe2e8f0))
                            .rounded_lg()
                            .p(rems(1.5))
                            .gap(rems(0.5))
                            .min_w(rems(8.0))
                            .child(
                                span("Progress")
                                    .text_size(rems(0.9))
                                    .text_color(rgb(0x64748b))
                                    .font_weight(FontWeight::MEDIUM)
                            )
                            .child(
                                span(format!("{:.0}%", self.progress_value))
                                    .text_size(rems(1.8))
                                    .text_color(progress_color)
                                    .font_weight(FontWeight::BOLD)
                            )
                    )
                    .child(
                        v_flex()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe2e8f0))
                            .rounded_lg()
                            .p(rems(1.5))
                            .gap(rems(0.5))
                            .min_w(rems(8.0))
                            .child(
                                span("Button Clicks")
                                    .text_size(rems(0.9))
                                    .text_color(rgb(0x64748b))
                                    .font_weight(FontWeight::MEDIUM)
                            )
                            .child(
                                span(format!("{}", self.button_click_count))
                                    .text_size(rems(1.8))
                                    .text_color(rgb(0x3b82f6))
                                    .font_weight(FontWeight::BOLD)
                            )
                    )
                    .child(
                        v_flex()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe2e8f0))
                            .rounded_lg()
                            .p(rems(1.5))
                            .gap(rems(0.5))
                            .min_w(rems(8.0))
                            .child(
                                span("Status")
                                    .text_size(rems(0.9))
                                    .text_color(rgb(0x64748b))
                                    .font_weight(FontWeight::MEDIUM)
                            )
                            .child(
                                span(if self.disabled { "Disabled" } else { "Active" })
                                    .text_size(rems(1.8))
                                    .text_color(if self.disabled { rgb(0xef4444) } else { rgb(0x10b981) })
                                    .font_weight(FontWeight::BOLD)
                            )
                    )
            )

            .child(
                v_flex()
                    .gap(rems(1.5))
                    .child(
                        span("🔘 Interactive Buttons")
                            .text_size(rems(1.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x1e293b))
                    )
                    .child(
                        v_flex()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe2e8f0))
                            .rounded_lg()
                            .p(rems(2.0))
                            .gap(rems(1.5))
                            .child(
                                span("Progress Controls")
                                    .text_size(rems(1.1))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0x374151))
                            )
                            .child(
                                h_flex()
                                    .gap(rems(1.0))
                                    .flex_wrap()
                                    .child(
                                        button("increment")
                                            .border_2()
                                            .focus(|this| this.border_color(rgb(0x000000)))
                                            .bg(rgb(0x3b82f6))
                                            .hover(|this| this.bg(rgb(0x2563eb)))
                                            .disabled(self.disabled)
                                            .px(rems(1.5))
                                            .py(rems(0.75))
                                            .rounded_md()
                                            .child(span("+ Increase").text_color(rgb(0xffffff)).font_weight(FontWeight::MEDIUM))
                                            .on_click(cx.listener(Self::increment_progress))
                                            .when_disabled(|this| this.bg(rgb(0x9ca3af)).cursor_not_allowed())
                                    )
                                    .child(
                                        button("decrement")
                                            .border_2()
                                            .focus(|this| this.border_color(rgb(0x000000)))
                                            .bg(rgb(0xf59e0b))
                                            .hover(|this| this.bg(rgb(0xd97706)))
                                            .disabled(self.disabled)
                                            .px(rems(1.5))
                                            .py(rems(0.75))
                                            .rounded_md()
                                            .child(span("- Decrease").text_color(rgb(0xffffff)).font_weight(FontWeight::MEDIUM))
                                            .on_click(cx.listener(Self::decrement_progress))
                                            .when_disabled(|this| this.bg(rgb(0x9ca3af)).cursor_not_allowed())
                                    )
                                    .child(
                                        button("reset")
                                            .border_2()
                                            .focus(|this| this.border_color(rgb(0x000000)))
                                            .bg(rgb(0xef4444))
                                            .hover(|this| this.bg(rgb(0xdc2626)))
                                            .disabled(self.disabled)
                                            .px(rems(1.5))
                                            .py(rems(0.75))
                                            .rounded_md()
                                            .child(span("Reset").text_color(rgb(0xffffff)).font_weight(FontWeight::MEDIUM))
                                            .on_click(cx.listener(Self::reset_progress))
                                            .when_disabled(|this| this.bg(rgb(0x9ca3af)).cursor_not_allowed())
                                    )
                                    .child(
                                        button("complete")
                                            .border_2()
                                            .focus(|this| this.border_color(rgb(0x000000)))
                                            .bg(rgb(0x10b981))
                                            .hover(|this| this.bg(rgb(0x059669)))
                                            .disabled(self.disabled)
                                            .px(rems(1.5))
                                            .py(rems(0.75))
                                            .rounded_md()
                                            .child(span("Complete").text_color(rgb(0xffffff)).font_weight(FontWeight::MEDIUM))
                                            .on_click(cx.listener(Self::complete_progress))
                                            .when_disabled(|this| this.bg(rgb(0x9ca3af)).cursor_not_allowed())
                                    )
                            )
                            .child(
                                span("State Controls")
                                    .text_size(rems(1.1))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0x374151))
                            )
                            .child(
                                h_flex()
                                    .gap(rems(1.0))
                                    .items_center()
                                    .child(
                                        h_flex()
                                            .gap(rems(0.5))
                                            .items_center()
                                            .child(Checkbox::new("checkbox")
                                                        .rounded_md()
                                                        .checked(self.disabled)
                                                        .on_change(cx.listener(Self::set_disabled))
                                                        .border_1()
                                                        .border_color(rgb(0xe2e8f0))
                                                        .indicator(span("✓").text_color(rgb(0xffffff)).text_xs())
                                                        .size(rems(1.5))
                                                        .when_checked(|this| this.bg(rgb(0x6366f1)))
                                            )
                                            .child(Switch::new("switch")
                                                .rounded_3xl()
                                                .checked(self.disabled)
                                                .items_center()
                                                .on_change(cx.listener(Self::set_disabled))
                                                .border_1()
                                                .px(px(2.))
                                                .border_color(rgb(0xe2e8f0))
                                                .thumb(|thumb| thumb.rounded_full().size(rems(1.)).bg(rgb(0xacacac)))
                                                .h(px(24.))
                                                .w(px(44.))
                                                .when_checked(|this|
                                                    this.thumb(|thumb|
                                                            thumb.bg(rgb(0xffffff)))
                                                        .bg(rgb(0x10b981)))
                                                .with_animation(("checkbox", self.disabled as u32), Animation::new(Duration::from_millis(100)), move |this, delta| {
                                                    this.thumb(|thumb| {
                                                        if disabled {
                                                            thumb.left(px( delta * 21.))

                                                        } else {
                                                            thumb.left(px(21. - delta * 21.))
                                                        }
                                                    })
                                                })
                                            )
                                            .child(
                                                span("Disable Controls")
                                                    .text_color(rgb(0x374151))
                                                    .text_size(rems(0.95))
                                            )
                                    )
                            )
                            .child(
                                h_flex()
                                    .gap(rems(1.0))
                                    .child(
                                        button("toggle_disabled")
                                            .auto_focus(true)
                                            .border_2()
                                            .focus(|this| this.border_color(rgb(0x000000)))
                                            .bg(rgb(0x6366f1))
                                            .hover(|this| this.bg(rgb(0x5b21b6)))
                                            .px(rems(1.5))
                                            .py(rems(0.75))
                                            .rounded_md()
                                            .child(
                                                span(if self.disabled { "Enable Buttons" } else { "Disable Buttons" })
                                                    .text_color(rgb(0xffffff))
                                                    .font_weight(FontWeight::MEDIUM)
                                            )
                                            .on_click(cx.listener(Self::toggle_disabled))
                                    )
                                    .child(
                                        button("reset_counter")
                                            .border_2()
                                            .focus(|this| this.border_color(rgb(0x000000)))
                                            .bg(rgb(0x64748b))
                                            .hover(|this| this.bg(rgb(0x475569)))
                                            .px(rems(1.5))
                                            .py(rems(0.75))
                                            .rounded_md()
                                            .child(span("Reset Counter").text_color(rgb(0xffffff)).font_weight(FontWeight::MEDIUM))
                                            .on_click(cx.listener(Self::reset_counter))
                                    )
                            )
                    )
            )

            .child(div().bg(rgb(0xe2e8f0)).h(px(1.)).w_full())

            .child(
                v_flex()
                    .gap(rems(1.5))
                    .child(
                        span("📊 Progress Indicators")
                            .text_size(rems(1.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x1e293b))
                    )
                    .child(
                        v_flex()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe2e8f0))
                            .rounded_lg()
                            .p(rems(2.0))
                            .gap(rems(1.5))
                            .child(
                                Progress::new()
                                    .flex_col()
                                    .flex()
                                    .value(self.progress_value)
                                    .bg(rgb(0xffffff))
                                    .w_full()
                                    .gap(rems(1.0))
                                    .value_label(|provider| {
                                        format!(
                                            "{}% complete ({}/100 tasks)",
                                            (provider.percentage() * 100.0).round() as u8,
                                            provider.value() as u8
                                        )
                                    })
                                    .child_with_context(|provider| {
                                        h_flex()
                                            .justify_between()
                                            .items_center()
                                            .child(
                                                span("Task Progress")
                                                    .text_color(rgb(0x374151))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_size(rems(1.1))
                                            )
                                            .child(
                                                span(provider.value_label())
                                                    .text_color(rgb(0x64748b))
                                                    .font_weight(FontWeight::MEDIUM)
                                            )
                                    })
                                    .child_with_context(|provider| {
                                        let previous_percent = provider.percentage_of(self.previous_progress_value);
                                        let percentage = provider.percentage();
                                        ProgressTrack::new()
                                            .bg(rgb(0xf1f5f9))
                                            .border_1()
                                            .border_color(rgb(0xe2e8f0))
                                            .h(rems(1.5))
                                            .w_full()
                                            .rounded_3xl()
                                            .overflow_hidden()
                                            .child(
                                                ProgressFill::new()
                                                    .bg(progress_color)
                                                    .h_full()
                                                    .rounded_3xl()
                                                    .with_animation(("progress", (percentage * 1000.) as u32), Animation::new(Duration::from_millis(200)), move |this, delta| {
                                                        let previous_size = previous_percent;
                                                        let current_size = percentage;
                                                        let interpolated_size = previous_size + (current_size - previous_size) * delta;
                                                        this.w(relative(interpolated_size))
                                                    })
                                            )
                                    })
                            )
                    )
            )

            .child(div().bg(rgb(0xe2e8f0)).h(px(1.)).w_full())

            .child(
                v_flex()
                    .gap(rems(1.5))
                    .child(
                        span("✏️ Text Input Components")
                            .text_size(rems(1.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x1e293b))
                    )
                    .child(
                        v_flex()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe2e8f0))
                            .rounded_lg()
                            .p(rems(2.0))
                            .gap(rems(1.0))
                            .child(
                                span("Interactive text input with search icon")
                                    .text_color(rgb(0x64748b))
                                    .text_size(rems(0.95))
                            )
                            .child(
                                text_field(self.text_state.clone())
                                    .disabled(self.disabled)
                                    .border_color(rgb(0xd1d5db))
                                    .focus(|this| this.border_color(rgb(0x3b82f6)))
                                    .text_color(rgb(0x374151))
                                    .bg(rgb(0xffffff))
                                    .h(px(56.))
                                    .pr(px(16.))
                                    .pl(px(8.))
                                    .border_2()
                                    .max_w(rems(20.))
                                    .rounded_lg()
                                    .gap(px(12.))
                            )
                    )
            )

            .child(div().bg(rgb(0xe2e8f0)).h(px(1.)).w_full().w(px(20.)))

            .child(
                v_flex()
                    .gap(rems(1.5))
                    .child(
                        span("🔗 Links & Resources")
                            .text_size(rems(1.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x1e293b))
                    )
                    .child(
                        v_flex()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe2e8f0))
                            .rounded_lg()
                            .p(rems(2.0))
                            .gap(rems(1.5))
                            .child(
                                h_flex()
                                    .gap(rems(2.0))
                                    .flex_wrap()
                                    .child(
                                        a("https://github.com/J0R6IT0/lapislazuli")
                                            .bg(rgb(0x1f2937))
                                            .hover(|this| this.bg(rgb(0x111827)))
                                            .px(rems(1.5))
                                            .py(rems(0.75))
                                            .rounded_md()
                                            .child("📦 View Source Code")
                                            .text_color(rgb(0xffffff))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_decoration_none()
                                    )
                                    .child(
                                        a("https://github.com/zed-industries/zed/tree/main/crates/gpui")
                                            .bg(rgb(0x059669))
                                            .hover(|this| this.bg(rgb(0x047857)))
                                            .px(rems(1.5))
                                            .py(rems(0.75))
                                            .rounded_md()
                                            .child("🚀 GPUI Framework")
                                            .text_color(rgb(0xffffff))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_decoration_none()
                                    )
                            )
                            .child(
                                v_flex()
                                    .gap(rems(0.5))
                                    .child(
                                        span("About lapislazuli")
                                            .text_color(rgb(0x374151))
                                            .font_weight(FontWeight::MEDIUM)
                                    )
                                    .child(
                                        span("A headless component library for GPUI providing reusable building blocks for modern applications. Features buttons, progress indicators, text inputs, and layout primitives.")
                                            .text_color(rgb(0x64748b))
                                            .text_size(rems(0.9))
                                            .line_height(rems(1.4))
                                    )
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |window, app| {
            let showcase = Showcase::new(window, app);
            LapislazuliProvider::new(showcase, window, app)
        })
        .unwrap();
    });
}

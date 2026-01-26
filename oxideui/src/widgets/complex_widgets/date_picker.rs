use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct DatePicker {
    pub value: Option<String>,
    pub placeholder: String,
    pub format: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub disabled: bool,
    pub open: bool,
    pub on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl DatePicker {
    pub fn new() -> Self {
        Self {
            value: None,
            placeholder: "Select date...".to_string(),
            format: "%Y-%m-%d".to_string(),
            width: None,
            height: None,
            disabled: false,
            open: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = format.into();
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.on_change = Some(Arc::new(callback));
        self
    }

    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for DatePicker {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(40.0);

        let bg_color = if self.disabled {
            theme.muted
        } else {
            theme.input
        };

        let border_color = if self.disabled {
            theme.border.with_alpha(128)
        } else {
            theme.border
        };

        let text_color = if self.disabled {
            theme.muted_foreground
        } else {
            theme.foreground
        };

        let mut render_objects = Vec::new();

        // Date picker box
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Border
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, 1.0),
            border_color,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(width - 1.0, 0.0, 1.0, height),
            border_color,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, height - 1.0, width, 1.0),
            border_color,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, 1.0, height),
            border_color,
        ));

        // Display value or placeholder
        let display_text = if let Some(value) = &self.value {
            value.clone()
        } else {
            self.placeholder.clone()
        };

        let display_color = if self.value.is_none() && !self.disabled {
            theme.muted_foreground
        } else {
            text_color
        };

        render_objects.push(RenderObject::text(
            display_text,
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 14.0,
                color: display_color,
                bold: false,
                italic: false,
            },
            Point::new(12.0, height / 2.0 + 5.0),
        ));

        // Calendar icon
        render_objects.push(RenderObject::text(
            "📅".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 16.0,
                color: theme.muted_foreground,
                bold: false,
                italic: false,
            },
            Point::new(width - 30.0, height / 2.0 + 5.0),
        ));

        // Calendar popup (if open)
        if self.open && !self.disabled {
            let calendar_width = 280.0;
            let calendar_height = 320.0;
            let calendar_x = 0.0;
            let calendar_y = height + 4.0;

            // Calendar background
            render_objects.push(RenderObject::rect(
                Rect::new(calendar_x, calendar_y, calendar_width, calendar_height),
                theme.popover,
            ));

            // Calendar border
            render_objects.push(RenderObject::rect(
                Rect::new(calendar_x, calendar_y, calendar_width, 1.0),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(calendar_x + calendar_width - 1.0, calendar_y, 1.0, calendar_height),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(calendar_x, calendar_y + calendar_height - 1.0, calendar_width, 1.0),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(calendar_x, calendar_y, 1.0, calendar_height),
                theme.border,
            ));

            // Calendar header (month/year)
            render_objects.push(RenderObject::text(
                "March 2024".to_string(), // Hardcoded for example
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 16.0,
                    color: theme.popover_foreground,
                    bold: true,
                    italic: false,
                },
                Point::new(calendar_x + 20.0, calendar_y + 30.0),
            ));

            // Day headers (Sun, Mon, Tue, etc.)
            let day_headers = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
            let cell_size = 36.0;
            let header_start_y = calendar_y + 60.0;

            for (i, day) in day_headers.iter().enumerate() {
                let x = calendar_x + 10.0 + (i as f32 * cell_size);
                render_objects.push(RenderObject::text(
                    day.to_string(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 12.0,
                        color: theme.muted_foreground,
                        bold: true,
                        italic: false,
                    },
                    Point::new(x, header_start_y),
                ));
            }

            // Calendar days (example grid)
            let days_start_y = header_start_y + 25.0;
            for week in 0..6 {
                for day in 0..7 {
                    let day_number = (week * 7 + day + 1).min(31);
                    let x = calendar_x + 10.0 + (day as f32 * cell_size);
                    let y = days_start_y + (week as f32 * cell_size);

                    let is_today = day_number == 15; // Example: today is 15th
                    let day_color = if is_today {
                        theme.primary
                    } else {
                        theme.popover_foreground
                    };

                    render_objects.push(RenderObject::text(
                        day_number.to_string(),
                        TextStyle {
                            font_family: theme.font_sans.clone(),
                            font_size: 14.0,
                            color: day_color,
                            bold: is_today,
                            italic: false,
                        },
                        Point::new(x + 10.0, y + 10.0),
                    ));
                }
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for DatePicker {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn key(&self) -> Option<WidgetKey> {
        self.key.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Widget> {
        Box::new(self.clone())
    }
}
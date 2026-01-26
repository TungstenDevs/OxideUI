use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Sonner {
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub duration_ms: u64,
    pub position: ToastPosition,
    pub visible: bool,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastVariant {
    Default,
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Sonner {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
            duration_ms: 3000,
            position: ToastPosition::BottomRight,
            visible: false,
            on_close: None,
            key: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_close = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Sonner {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        if !self.visible {
            return WidgetNode::None;
        }

        let theme = ctx.theme();
        let screen_width = ctx.constraints.max_width;
        let screen_height = ctx.constraints.max_height;

        let toast_width = 350.0;
        let toast_height = if self.description.is_some() { 100.0 } else { 70.0 };
        let padding = 16.0;

        // Calculate position based on toast position
        let (x, y) = match self.position {
            ToastPosition::TopLeft => (20.0, 20.0),
            ToastPosition::TopCenter => ((screen_width - toast_width) / 2.0, 20.0),
            ToastPosition::TopRight => (screen_width - toast_width - 20.0, 20.0),
            ToastPosition::BottomLeft => (20.0, screen_height - toast_height - 20.0),
            ToastPosition::BottomCenter => ((screen_width - toast_width) / 2.0, screen_height - toast_height - 20.0),
            ToastPosition::BottomRight => (screen_width - toast_width - 20.0, screen_height - toast_height - 20.0),
        };

        let bg_color = match self.variant {
            ToastVariant::Default => theme.background,
            ToastVariant::Success => Color::from_hex(0x10B981),
            ToastVariant::Error => theme.destructive,
            ToastVariant::Warning => Color::from_hex(0xF59E0B),
            ToastVariant::Info => theme.primary,
        };

        let text_color = match self.variant {
            ToastVariant::Default => theme.foreground,
            ToastVariant::Success => Color::WHITE,
            ToastVariant::Error => theme.destructive_foreground,
            ToastVariant::Warning => Color::from_hex(0x78350F),
            ToastVariant::Info => theme.primary_foreground,
        };

        let mut render_objects = Vec::new();

        // Toast background with shadow
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, toast_width, toast_height),
            bg_color,
        ));

        // Toast border
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, toast_width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x + toast_width - 1.0, y, 1.0, toast_height),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x, y + toast_height - 1.0, toast_width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, 1.0, toast_height),
            theme.border,
        ));

        // Icon based on variant
        let icon = match self.variant {
            ToastVariant::Default => "💬",
            ToastVariant::Success => "✅",
            ToastVariant::Error => "❌",
            ToastVariant::Warning => "⚠️",
            ToastVariant::Info => "ℹ️",
        };

        render_objects.push(RenderObject::text(
            icon.to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 20.0,
                color: text_color,
                bold: false,
                italic: false,
            },
            Point::new(x + padding, y + padding + 5.0),
        ));

        // Title
        render_objects.push(RenderObject::text(
            self.title.clone(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 14.0,
                color: text_color,
                bold: true,
                italic: false,
            },
            Point::new(x + padding + 30.0, y + padding + 5.0),
        ));

        // Description
        if let Some(description) = &self.description {
            render_objects.push(RenderObject::text(
                description.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 12.0,
                    color: text_color.with_alpha(180),
                    bold: false,
                    italic: false,
                },
                Point::new(x + padding + 30.0, y + padding + 25.0),
            ));
        }

        // Close button
        let close_button_size = 24.0;
        let close_x = x + toast_width - close_button_size - 8.0;
        let close_y = y + 8.0;

        render_objects.push(RenderObject::rect(
            Rect::new(close_x, close_y, close_button_size, close_button_size),
            text_color.with_alpha(50),
        ));

        render_objects.push(RenderObject::text(
            "×".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 18.0,
                color: text_color,
                bold: true,
                italic: false,
            },
            Point::new(close_x + 4.0, close_y + 4.0),
        ));

        // Progress bar (showing time remaining)
        let progress_width = toast_width - (padding * 2.0);
        render_objects.push(RenderObject::rect(
            Rect::new(x + padding, y + toast_height - 4.0, progress_width, 2.0),
            text_color.with_alpha(100),
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Sonner {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        if !self.visible {
            return EventResult::Unhandled;
        }

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() => {
                let screen_width = 800.0;
                let screen_height = 600.0;
                let toast_width = 350.0;
                let toast_height = if self.description.is_some() { 100.0 } else { 70.0 };

                let (x, y) = match self.position {
                    ToastPosition::TopLeft => (20.0, 20.0),
                    ToastPosition::TopCenter => ((screen_width - toast_width) / 2.0, 20.0),
                    ToastPosition::TopRight => (screen_width - toast_width - 20.0, 20.0),
                    ToastPosition::BottomLeft => (20.0, screen_height - toast_height - 20.0),
                    ToastPosition::BottomCenter => ((screen_width - toast_width) / 2.0, screen_height - toast_height - 20.0),
                    ToastPosition::BottomRight => (screen_width - toast_width - 20.0, screen_height - toast_height - 20.0),
                };

                let close_button_size = 24.0;
                let close_x = x + toast_width - close_button_size - 8.0;
                let close_y = y + 8.0;

                let close_button_rect = Rect::new(close_x, close_y, close_button_size, close_button_size);

                if close_button_rect.contains(position.x, position.y) {
                    if let Some(on_close) = &self.on_close {
                        on_close();
                    }
                    return EventResult::Stopped;
                }

                EventResult::Unhandled
            }
            _ => EventResult::Unhandled,
        }
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
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use std::any::Any;
use std::sync::Arc;

pub struct Dialog {
    pub title: String,
    pub description: Option<String>,
    pub children: Vec<Box<dyn Widget>>,
    pub open: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

impl Dialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            children: Vec::new(),
            open: false,
            width: None,
            height: None,
            on_close: None,
            key: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            description: self.description.clone(),
            children: self
                .children
                .iter()
                .map(|child| child.clone_box())
                .collect(),
            open: self.open,
            width: self.width,
            height: self.height,
            on_close: self.on_close.clone(),
            key: self.key.clone(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Widget>>) -> Self {
        self.children = children;
        self
    }

    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
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

impl StatelessWidget for Dialog {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        if !self.open {
            return WidgetNode::None;
        }

        let theme = &ctx.theme;
        let width = self.width.unwrap_or(400.0);
        let height = self.height.unwrap_or(300.0);

        // Position in center of screen (simplified)
        let screen_width = ctx.constraints.max_width;
        let screen_height = ctx.constraints.max_height;

        let x = (screen_width - width) / 2.0;
        let y = (screen_height - height) / 2.0;

        let mut render_objects = Vec::new();

        // Backdrop (semi-transparent overlay)
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, screen_width, screen_height),
            Color::rgba(0, 0, 0, 100),
        ));

        // Dialog container
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, width, height),
            theme.popover,
        ));

        // Dialog border (all sides)
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x + width - 1.0, y, 1.0, height),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x, y + height - 1.0, width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, 1.0, height),
            theme.border,
        ));

        // Title
        render_objects.push(RenderObject::text(
            self.title.clone(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 18.0,
                color: theme.popover_foreground,
                bold: true,
                italic: false,
            },
            Point::new(x + 16.0, y + 20.0),
        ));

        // Description
        if let Some(description) = &self.description {
            render_objects.push(RenderObject::text(
                description.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 14.0,
                    color: theme.muted_foreground,
                    bold: false,
                    italic: false,
                },
                Point::new(x + 16.0, y + 50.0),
            ));
        }

        // Close button
        let close_button_size = 24.0;
        render_objects.push(RenderObject::rect(
            Rect::new(
                x + width - close_button_size - 8.0,
                y + 8.0,
                close_button_size,
                close_button_size,
            ),
            theme.destructive,
        ));

        render_objects.push(RenderObject::text(
            "×".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 18.0,
                color: theme.destructive_foreground,
                bold: true,
                italic: false,
            },
            Point::new(x + width - close_button_size - 4.0, y + 10.0),
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Dialog {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        if !self.open {
            return EventResult::Unhandled;
        }

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() => {
                // Calculate close button rect using screen dimensions
                // Note: We don't have direct access to ctx here, so we use hardcoded values or pass them through state
                let _width = self.width.unwrap_or(400.0);
                let _height = self.height.unwrap_or(300.0);
                let _close_button_size = 24.0;

                // These would need to be calculated from screen dimensions
                // For now, just check if clicked
                if let Some(on_close) = &self.on_close {
                    on_close();
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

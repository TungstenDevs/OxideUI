use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

pub struct Drawer {
    pub title: Option<String>,
    pub position: DrawerPosition,
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub children: Vec<Box<dyn Widget>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerPosition {
    Left,
    Right,
    Top,
    Bottom,
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            title: None,
            position: DrawerPosition::Right,
            width: 300.0,
            height: 400.0,
            open: false,
            children: Vec::new(),
            on_close: None,
            key: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            position: self.position,
            width: self.width,
            height: self.height,
            open: self.open,
            children: self
                .children
                .iter()
                .map(|child| child.clone_box())
                .collect(),
            on_close: self.on_close.clone(),
            key: self.key.clone(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_position(mut self, position: DrawerPosition) -> Self {
        self.position = position;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
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

impl StatelessWidget for Drawer {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        if !self.open {
            return WidgetNode::None;
        }

        let theme = ctx.theme();
        let screen_width = ctx.constraints.max_width;
        let screen_height = ctx.constraints.max_height;

        let mut render_objects = Vec::new();

        // Backdrop (semi-transparent overlay)
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, screen_width, screen_height),
            Color::rgba(0, 0, 0, 100),
        ));

        // Calculate drawer position
        let (x, y, width, height) = match self.position {
            DrawerPosition::Left => (
                0.0,
                0.0,
                self.width,
                screen_height,
            ),
            DrawerPosition::Right => (
                screen_width - self.width,
                0.0,
                self.width,
                screen_height,
            ),
            DrawerPosition::Top => (
                0.0,
                0.0,
                screen_width,
                self.height,
            ),
            DrawerPosition::Bottom => (
                0.0,
                screen_height - self.height,
                screen_width,
                self.height,
            ),
        };

        // Drawer container
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, width, height),
            theme.popover,
        ));

        // Drawer border
        match self.position {
            DrawerPosition::Left => {
                render_objects.push(RenderObject::rect(
                    Rect::new(x + width - 1.0, y, 1.0, height),
                    theme.border,
                ));
            }
            DrawerPosition::Right => {
                render_objects.push(RenderObject::rect(
                    Rect::new(x, y, 1.0, height),
                    theme.border,
                ));
            }
            DrawerPosition::Top => {
                render_objects.push(RenderObject::rect(
                    Rect::new(x, y + height - 1.0, width, 1.0),
                    theme.border,
                ));
            }
            DrawerPosition::Bottom => {
                render_objects.push(RenderObject::rect(
                    Rect::new(x, y, width, 1.0),
                    theme.border,
                ));
            }
        }

        // Title
        if let Some(title) = &self.title {
            let title_y = y + 20.0;
            render_objects.push(RenderObject::text(
                title.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 18.0,
                    color: theme.popover_foreground,
                    bold: true,
                    italic: false,
                },
                Point::new(x + 20.0, title_y),
            ));
        }

        // Close button
        let close_button_size = 24.0;
        let close_x = match self.position {
            DrawerPosition::Left | DrawerPosition::Top => x + width - close_button_size - 8.0,
            DrawerPosition::Right | DrawerPosition::Bottom => x + 8.0,
        };
        let close_y = y + 8.0;

        render_objects.push(RenderObject::rect(
            Rect::new(close_x, close_y, close_button_size, close_button_size),
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
            Point::new(close_x + 4.0, close_y + 4.0),
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Drawer {
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
                // FIX: EventContext doesn't have constraints
                // We need to use stored dimensions or pass them through widget state
                let _close_button_size = 24.0;

                // Check if close button clicked (simplified - needs proper calculation)
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
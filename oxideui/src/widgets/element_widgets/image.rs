use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::core::render_object::{Color, Rect, RenderObject};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Image {
    pub path: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub fit: ImageFit,
    pub alt_text: String,
    pub tooltip: Option<String>,
    pub on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageFit {
    Fill,
    Contain,
    Cover,
    ScaleDown,
}

impl Image {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            width: None,
            height: None,
            fit: ImageFit::Contain,
            alt_text: String::new(),
            tooltip: None,
            on_click: None,
            key: None,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn with_alt_text(mut self, alt_text: impl Into<String>) -> Self {
        self.alt_text = alt_text.into();
        self
    }

    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn with_on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Image {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let width = self.width.unwrap_or(ctx.constraints.max_width);
        let height = self.height.unwrap_or(ctx.constraints.max_height);

        // For now, draw a placeholder rectangle
        // In a real implementation, we would load and decode the image
        let placeholder_color = Color::from_hex(0xE5E7EB);
        let border_color = Color::from_hex(0xD1D5DB);

        let mut children = Vec::new();

        // Background
        children.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            placeholder_color,
        ));

        // Border
        children.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, 1.0),
            border_color,
        ));
        children.push(RenderObject::rect(
            Rect::new(width - 1.0, 0.0, 1.0, height),
            border_color,
        ));
        children.push(RenderObject::rect(
            Rect::new(0.0, height - 1.0, width, 1.0),
            border_color,
        ));
        children.push(RenderObject::rect(
            Rect::new(0.0, 0.0, 1.0, height),
            border_color,
        ));

        // "Image" text
        let theme = ctx.theme();
        children.push(RenderObject::text(
            "📷 Image".to_string(),
            crate::core::render_object::TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 14.0,
                color: Color::from_hex(0x6B7280),
                bold: false,
                italic: true,
            },
            crate::core::render_object::Point::new(width / 2.0 - 30.0, height / 2.0 + 5.0),
        ));

        WidgetNode::Leaf(RenderObject::group(children))
    }
}

impl Widget for Image {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        match event {
            UiEvent::PointerUp { button: MouseButton::Left, .. } if context.is_at_target() => {
                if let Some(on_click) = &self.on_click {
                    on_click();
                    EventResult::Stopped
                } else {
                    EventResult::Unhandled
                }
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
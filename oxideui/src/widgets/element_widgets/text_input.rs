use crate::core::*;
use crate::core::render_object::{Point, Rect, TextStyle};
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct TextInput {
    pub placeholder: String,
    pub value: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub disabled: bool,
    pub on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl TextInput {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            value: String::new(),
            width: None,
            height: Some(40.0),
            disabled: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.on_change = Some(Arc::new(callback));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_tooltip(mut self, text: impl Into<String>) -> Self {
        self.tooltip = Some(text.into());
        self
    }
}

impl StatelessWidget for TextInput {
    fn build_stateless(&self, _ctx: &BuildContext) -> WidgetNode {
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(40.0);

        let bg_color = if self.disabled {
            Color::from_hex(0xF3F4F6)
        } else {
            Color::WHITE
        };

        let mut render_objects = Vec::new();

        // Background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Border
        let border_color = Color::from_hex(0xE5E7EB);
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, 1.0),
            border_color,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, height - 1.0, width, 1.0),
            border_color,
        ));

        // Text
        let text = if self.value.is_empty() {
            &self.placeholder
        } else {
            &self.value
        };

        let text_color = if self.value.is_empty() {
            Color::from_hex(0x9CA3AF)
        } else {
            Color::from_hex(0x111827)
        };

        render_objects.push(RenderObject::text(
            text.clone(),
            TextStyle {
                font_family: "Inter".to_string(),
                font_size: 14.0,
                color: text_color,
                bold: false,
                italic: false,
            },
            Point::new(12.0, height / 2.0 + 5.0),
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for TextInput {
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
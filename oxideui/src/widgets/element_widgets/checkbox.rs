use crate::core::*;
use crate::core::render_object::{Point, Rect, TextStyle};  // Use OUR Rect
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct Checkbox {
    pub checked: bool,
    pub label: Option<String>,
    pub disabled: bool,
    pub on_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Checkbox {
    pub fn new() -> Self {
        Self {
            checked: false,
            label: None,
            disabled: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(bool) + Send + Sync + 'static,
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

impl StatelessWidget for Checkbox {
    fn build_stateless(&self, _ctx: &BuildContext) -> WidgetNode {
        let size = 20.0;
        let mut render_objects = Vec::new();

        let bg_color = if self.checked {
            Color::from_hex(0xD87943)
        } else {
            Color::WHITE
        };

        // Use OUR Rect type
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, size, size),
            bg_color,
        ));

        // Border
        let border_color = if self.checked {
            Color::from_hex(0xD87943)
        } else {
            Color::from_hex(0xE5E7EB)
        };

        for i in 0..4 {
            let (x, y, w, h) = match i {
                0 => (0.0, 0.0, size, 1.0),
                1 => (size - 1.0, 0.0, 1.0, size),
                2 => (0.0, size - 1.0, size, 1.0),
                _ => (0.0, 0.0, 1.0, size),
            };
            render_objects.push(RenderObject::rect(Rect::new(x, y, w, h), border_color));
        }

        // Checkmark
        if self.checked {
            render_objects.push(RenderObject::rect(
                Rect::new(6.0, 9.0, 8.0, 2.0),
                Color::WHITE,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(6.0, 9.0, 2.0, 6.0),
                Color::WHITE,
            ));
        }

        // Label
        if let Some(label) = &self.label {
            render_objects.push(RenderObject::text(
                label.clone(),
                TextStyle {
                    font_family: "Inter".to_string(),
                    font_size: 14.0,
                    color: Color::from_hex(0x111827),
                    bold: false,
                    italic: false,
                },
                Point::new(size + 8.0, size / 2.0 + 5.0),
            ));
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Checkbox {
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

use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{ Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Slider {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub step: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub disabled: bool,
    pub on_change: Option<Arc<dyn Fn(f32) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Slider {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            value: min,
            step: None,
            width: None,
            height: None,
            disabled: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn with_step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
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

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32) + Send + Sync + 'static,
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

impl StatelessWidget for Slider {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(32.0);

        let track_height = 6.0;
        let thumb_size = 20.0;

        let normalized_value = (self.value - self.min) / (self.max - self.min);
        let thumb_position = normalized_value * (width - thumb_size);

        let track_color = if self.disabled {
            theme.muted
        } else {
            theme.border
        };

        let fill_color = if self.disabled {
            theme.muted.with_alpha(128)
        } else {
            theme.primary
        };

        let thumb_color = if self.disabled {
            theme.muted_foreground
        } else {
            theme.primary
        };

        let mut render_objects = Vec::new();

        // Track background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, (height - track_height) / 2.0, width, track_height),
            track_color,
        ));

        // Filled portion
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, (height - track_height) / 2.0, thumb_position + thumb_size / 2.0, track_height),
            fill_color,
        ));

        // Thumb
        render_objects.push(RenderObject::rect(
            Rect::new(thumb_position, (height - thumb_size) / 2.0, thumb_size, thumb_size),
            thumb_color,
        ));

        // Value label
        if !self.disabled {
            let value_text = format!("{:.1}", self.value);
            render_objects.push(RenderObject::text(
                value_text,
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 12.0,
                    color: theme.foreground,
                    bold: false,
                    italic: false,
                },
                Point::new(thumb_position + thumb_size / 2.0 - 10.0, (height - thumb_size) / 2.0 - 15.0),
            ));
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Slider {
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
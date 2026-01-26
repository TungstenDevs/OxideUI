use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Rect, RenderObject};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct ProgressBar {
    pub value: f32,
    pub max: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub variant: ProgressVariant,
    pub show_value: bool,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProgressVariant {
    Default,
    Striped,
    Animated,
    Circular,
}

impl ProgressBar {
    pub fn new(value: f32, max: f32) -> Self {
        Self {
            value: value.clamp(0.0, max),
            max,
            width: None,
            height: None,
            variant: ProgressVariant::Default,
            show_value: false,
            tooltip: None,
            key: None,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
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

impl StatelessWidget for ProgressBar {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(8.0);

        let progress = (self.value / self.max).clamp(0.0, 1.0);
        let progress_width = width * progress;

        let bg_color = theme.muted;
        let progress_color = if progress < 0.3 {
            theme.destructive
        } else if progress < 0.7 {
            theme.secondary
        } else {
            theme.primary
        };

        let mut render_objects = Vec::new();

        // Background track
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Progress fill
        if self.variant == ProgressVariant::Striped {
            // Striped pattern (simplified)
            let stripe_width = 10.0;
            let mut stripe_x = 0.0;
            while stripe_x < progress_width {
                let stripe_end = (stripe_x + stripe_width).min(progress_width);
                let stripe_color = if (stripe_x / stripe_width) as i32 % 2 == 0 {
                    progress_color
                } else {
                    progress_color.with_alpha(180)
                };

                render_objects.push(RenderObject::rect(
                    Rect::new(stripe_x, 0.0, stripe_end - stripe_x, height),
                    stripe_color,
                ));

                stripe_x += stripe_width;
            }
        } else {
            // Solid fill
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, 0.0, progress_width, height),
                progress_color,
            ));
        }

        // Value text
        if self.show_value {
            let value_text = format!("{:.0}%", progress * 100.0);
            render_objects.push(RenderObject::text(
                value_text,
                crate::core::render_object::TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 12.0,
                    color: theme.foreground,
                    bold: false,
                    italic: false,
                },
                crate::core::render_object::Point::new(width + 8.0, height / 2.0 + 5.0),
            ));
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for ProgressBar {
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
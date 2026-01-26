use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Label {
    pub text: String,
    pub bold: bool,
    pub size: Option<f32>,
    pub color: Option<Color>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            bold: false,
            size: None,
            color: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
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

impl StatelessWidget for Label {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let text_color = self.color.unwrap_or_else(|| {
            Color::from_hex(if theme.is_dark { 0xEEEEEE } else { 0x111111 })
        });

        let font_size = self.size.unwrap_or(14.0);

        WidgetNode::Leaf(RenderObject::text(
            self.text.clone(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size,
                color: text_color,
                bold: self.bold,
                italic: false,
            },
            Point::ZERO,
        ))
    }
}

impl Widget for Label {
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
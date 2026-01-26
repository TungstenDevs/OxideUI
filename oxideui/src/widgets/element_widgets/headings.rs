use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Heading {
    pub text: String,
    pub level: u8,
    pub color: Option<Color>,
    pub align: crate::layout::Alignment,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Heading {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            level: 1,
            color: None,
            align: crate::layout::Alignment::TopLeft,
            tooltip: None,
            key: None,
        }
    }

    pub fn level(mut self, level: u8) -> Self {
        self.level = level.min(6).max(1);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn align(mut self, align: crate::layout::Alignment) -> Self {
        self.align = align;
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

impl StatelessWidget for Heading {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let font_sizes = [48.0, 36.0, 30.0, 24.0, 20.0, 18.0];
        let font_weights = [true, true, true, false, false, false];

        let idx = (self.level - 1) as usize;
        let font_size = font_sizes.get(idx).copied().unwrap_or(16.0);
        let is_bold = font_weights.get(idx).copied().unwrap_or(false);

        let text_color = self.color.unwrap_or_else(|| {
            Color::from_hex(if theme.is_dark { 0xFFFFFF } else { 0x000000 })
        });

        WidgetNode::Leaf(RenderObject::text(
            self.text.clone(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size,
                color: text_color,
                bold: is_bold,
                italic: false,
            },
            Point::ZERO,
        ))
    }
}

impl Widget for Heading {
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

// Convenience constructors
pub fn h1(text: impl Into<String>) -> Heading {
    Heading::new(text).level(1)
}

pub fn h2(text: impl Into<String>) -> Heading {
    Heading::new(text).level(2)
}

pub fn h3(text: impl Into<String>) -> Heading {
    Heading::new(text).level(3)
}

pub fn h4(text: impl Into<String>) -> Heading {
    Heading::new(text).level(4)
}

pub fn h5(text: impl Into<String>) -> Heading {
    Heading::new(text).level(5)
}

pub fn h6(text: impl Into<String>) -> Heading {
    Heading::new(text).level(6)
}
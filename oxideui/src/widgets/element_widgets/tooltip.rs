use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct Tooltip {
    pub text: String,
    pub child: Box<dyn Widget>,
    pub placement: TooltipPlacement,
    pub delay: u32,
    pub max_width: Option<f32>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TooltipPlacement {
    Top,
    Bottom,
    Left,
    Right,
}

impl Tooltip {
    pub fn new(text: impl Into<String>, child: Box<dyn Widget>) -> Self {
        Self {
            text: text.into(),
            child,
            placement: TooltipPlacement::Top,
            delay: 500,
            max_width: Some(200.0),
            key: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            child: self.child.clone_box(),
            placement: self.placement,
            delay: self.delay,
            max_width: self.max_width,
            key: self.key.clone(),
        }
    }

    pub fn with_placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_delay(mut self, delay: u32) -> Self {
        self.delay = delay;
        self
    }

    pub fn with_max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Tooltip {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        // For now, just render the child
        // Tooltip display logic would be handled by the framework
        // based on hover state and delay
        self.child.build(ctx)
    }
}

impl Widget for Tooltip {
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

// Tooltip render object builder (used by framework)
pub fn render_tooltip(
    text: &str,
    position: Point,
    placement: TooltipPlacement,
    theme: &crate::core::Theme,
    max_width: f32,
) -> RenderObject {
    let padding = 8.0;
    let font_size = 12.0;

    // Measure text (simplified)
    let text_width = (text.len() as f32 * 7.5).min(max_width - padding * 2.0);
    let text_height = 20.0;

    let tooltip_width = text_width + padding * 2.0;
    let tooltip_height = text_height + padding * 2.0;

    // Calculate position based on placement
    let (x, y) = match placement {
        TooltipPlacement::Top => (position.x - tooltip_width / 2.0, position.y - tooltip_height - 8.0),
        TooltipPlacement::Bottom => (position.x - tooltip_width / 2.0, position.y + 8.0),
        TooltipPlacement::Left => (position.x - tooltip_width - 8.0, position.y - tooltip_height / 2.0),
        TooltipPlacement::Right => (position.x + 8.0, position.y - tooltip_height / 2.0),
    };

    let mut render_objects = Vec::new();

    // Background with shadow
    render_objects.push(RenderObject::rect(
        Rect::new(x, y, tooltip_width, tooltip_height),
        theme.popover,
    ));

    // Border
    render_objects.push(RenderObject::rect(
        Rect::new(x, y, tooltip_width, 1.0),
        theme.border,
    ));
    render_objects.push(RenderObject::rect(
        Rect::new(x + tooltip_width - 1.0, y, 1.0, tooltip_height),
        theme.border,
    ));
    render_objects.push(RenderObject::rect(
        Rect::new(x, y + tooltip_height - 1.0, tooltip_width, 1.0),
        theme.border,
    ));
    render_objects.push(RenderObject::rect(
        Rect::new(x, y, 1.0, tooltip_height),
        theme.border,
    ));

    // Text
    render_objects.push(RenderObject::text(
        text.to_string(),
        TextStyle {
            font_family: theme.font_sans.clone(),
            font_size,
            color: theme.popover_foreground,
            bold: false,
            italic: false,
        },
        Point::new(x + padding, y + padding + 5.0),
    ));

    RenderObject::group(render_objects)
}
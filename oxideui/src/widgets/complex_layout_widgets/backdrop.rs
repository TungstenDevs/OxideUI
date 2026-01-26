use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Rect, RenderObject};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct Backdrop {
    pub child: Box<dyn Widget>,
    pub blur: f32,
    pub brightness: f32,
    pub opacity: f32,
    pub visible: bool,
    key: Option<WidgetKey>,
}

impl Backdrop {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            child,
            blur: 4.0,
            brightness: 0.8,
            opacity: 0.5,
            visible: true,
            key: None,
        }
    }

    pub fn clone(&self) -> Self{
        Self {
            child: self.child.clone_box(),
            blur: self.blur,
            brightness: self.brightness,
            opacity: self.opacity,
            visible: self.visible,
            key: self.key.clone(),
        }
    }

    pub fn blur(mut self, blur: f32) -> Self {
        self.blur = blur;
        self
    }

    pub fn brightness(mut self, brightness: f32) -> Self {
        self.brightness = brightness;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Backdrop {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        if !self.visible {
            return self.child.build(ctx);
        }

        let width = ctx.constraints.max_width;
        let height = ctx.constraints.max_height;

        // In a real implementation, we would apply blur and brightness filters
        // For now, we'll just draw a semi-transparent overlay

        let mut render_objects = Vec::new();

        // Background overlay
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            Color::rgba(0, 0, 0, (self.opacity * 200.0) as u8),
        ));

        // Child content (should be drawn on top)
        let child_node = self.child.build(ctx);
        if let WidgetNode::Leaf(render_obj) = child_node {
            render_objects.push(render_obj);
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Backdrop {
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
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct ScrollArea {
    pub child: Box<dyn Widget>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub scroll_x: bool,
    pub scroll_y: bool,
    pub scrollbar_size: f32,
    key: Option<WidgetKey>,
}

impl ScrollArea {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            child,
            width: None,
            height: None,
            scroll_x: false,
            scroll_y: true,
            scrollbar_size: 8.0,
            key: None,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            child: self.child.clone_box(),
            width: self.width,
            height: self.height,
            scroll_x: self.scroll_x,
            scroll_y: self.scroll_y,
            scrollbar_size: self.scrollbar_size,
            key: self.key.clone(),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn scroll_x(mut self, scroll_x: bool) -> Self {
        self.scroll_x = scroll_x;
        self
    }

    pub fn scroll_y(mut self, scroll_y: bool) -> Self {
        self.scroll_y = scroll_y;
        self
    }

    pub fn scrollbar_size(mut self, size: f32) -> Self {
        self.scrollbar_size = size;
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for ScrollArea {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let width = self.width.unwrap_or(ctx.constraints.max_width);
        let height = self.height.unwrap_or(ctx.constraints.max_height);

        // Create a clipping area for the child
        // In a real implementation, we would handle scrolling and scrollbars
        let child_constraints = crate::layout::constraints::Constraints::new(
            0.0,
            width,
            0.0,
            height,
        );

        let child_ctx = ctx.child_context(ctx.element_id, child_constraints);
        self.child.build(&child_ctx)
    }
}

impl Widget for ScrollArea {
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
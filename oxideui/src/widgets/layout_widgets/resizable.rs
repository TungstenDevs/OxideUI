use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Rect, RenderObject};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

pub struct Resizable {
    pub child: Box<dyn Widget>,
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,
    pub width: f32,
    pub height: f32,
    pub resizable: ResizableEdges,
    pub on_resize: Option<Arc<dyn Fn(f32, f32) + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResizableEdges {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl ResizableEdges {
    pub fn all() -> Self {
        Self {
            left: true,
            right: true,
            top: true,
            bottom: true,
        }
    }

    pub fn none() -> Self {
        Self {
            left: false,
            right: false,
            top: false,
            bottom: false,
        }
    }

}

impl Resizable {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            child,
            min_width: 50.0,
            min_height: 50.0,
            max_width: 1000.0,
            max_height: 1000.0,
            width: 200.0,
            height: 150.0,
            resizable: ResizableEdges::all(),
            on_resize: None,
            key: None,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_min_size(mut self, min_width: f32, min_height: f32) -> Self {
        self.min_width = min_width;
        self.min_height = min_height;
        self
    }

    pub fn with_max_size(mut self, max_width: f32, max_height: f32) -> Self {
        self.max_width = max_width;
        self.max_height = max_height;
        self
    }

    pub fn resizable(mut self, edges: ResizableEdges) -> Self {
        self.resizable = edges;
        self
    }

    pub fn with_on_resize<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32, f32) + Send + Sync + 'static,
    {
        self.on_resize = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }

    pub fn clone(&self) -> Self {
        Self {
            child: self.child.clone_box(),
            min_width: self.min_width,
            min_height: self.min_height,
            max_width: self.max_width,
            max_height: self.max_height,
            width: self.width,
            height: self.height,
            resizable: self.resizable,
            on_resize: self.on_resize.clone(),
            key: self.key.clone(),
        }
    }
}

impl StatelessWidget for Resizable {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let handle_size = 8.0;

        let mut render_objects = Vec::new();

        // Child content
        let child_constraints = crate::layout::constraints::Constraints::new(
            0.0,
            self.width,
            0.0,
            self.height,
        );

        let child_ctx = ctx.child_context(ctx.element_id, child_constraints);
        let child_node = self.child.build(&child_ctx);

        if let WidgetNode::Leaf(render_obj) = child_node {
            render_objects.push(render_obj);
        }

        // Resize handles
        let handle_color = theme.primary.with_alpha(150);

        // Bottom-right handle (always visible if resizable)
        if self.resizable.right && self.resizable.bottom {
            let handle_x = self.width - handle_size;
            let handle_y = self.height - handle_size;

            render_objects.push(RenderObject::rect(
                Rect::new(handle_x, handle_y, handle_size, handle_size),
                handle_color,
            ));

            // Diagonal lines in handle
            render_objects.push(RenderObject::rect(
                Rect::new(handle_x + 1.0, handle_y + 3.0, handle_size - 2.0, 1.0),
                theme.primary_foreground,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(handle_x + 3.0, handle_y + 1.0, 1.0, handle_size - 2.0),
                theme.primary_foreground,
            ));
        }

        // Right handle
        if self.resizable.right {
            let handle_x = self.width - handle_size;
            let handle_y = (self.height - handle_size) / 2.0;

            render_objects.push(RenderObject::rect(
                Rect::new(handle_x, handle_y, handle_size, handle_size),
                handle_color,
            ));
        }

        // Bottom handle
        if self.resizable.bottom {
            let handle_x = (self.width - handle_size) / 2.0;
            let handle_y = self.height - handle_size;

            render_objects.push(RenderObject::rect(
                Rect::new(handle_x, handle_y, handle_size, handle_size),
                handle_color,
            ));
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Resizable {
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
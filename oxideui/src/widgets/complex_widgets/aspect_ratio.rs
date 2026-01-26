use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct AspectRatio {
    pub ratio: f32,
    pub child: Box<dyn Widget>,
    key: Option<WidgetKey>,
}

impl AspectRatio {
    pub fn new(ratio: f32, child: Box<dyn Widget>) -> Self {
        Self {
            ratio,
            child,
            key: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            ratio: self.ratio,
            child: self.child.clone_box(),
            key: self.key.clone(),
        }
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for AspectRatio {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let width = ctx.constraints.max_width;
        let target_height = width / self.ratio;

        let child_constraints = crate::layout::constraints::Constraints::new(
            0.0,
            width,
            0.0,
            target_height,
        );

        let child_ctx = ctx.child_context(ctx.element_id, child_constraints);
        self.child.build(&child_ctx)
    }
}

impl Widget for AspectRatio {
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
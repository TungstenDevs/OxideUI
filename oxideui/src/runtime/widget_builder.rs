use crate::core::{BuildContext, RenderObject, Theme, Widget, WidgetNode};
use crate::layout::Constraints;
use std::sync::Arc;

pub struct WidgetBuilder {
    theme: Arc<Theme>,
}

impl WidgetBuilder {
    pub fn new(theme: Arc<Theme>) -> Self {
        Self { theme }
    }

    /// Build the complete widget tree into render objects
    pub fn build_widget_tree(&self, root_widget: &Box<dyn Widget>, constraints: Constraints) -> RenderObject {
        println!("🎨 Building widget tree...");

        let element_tree = crate::core::element::new_shared_element_tree();

        let ctx = BuildContext::new(
            crate::core::element::ElementId::new(0),
            element_tree,
            constraints,
            self.theme.clone(),
        );

        let widget_node = root_widget.build(&ctx);

        let widget_type = match &widget_node {
            WidgetNode::Leaf(_) => "Leaf",
            WidgetNode::Container { children } => {
                return RenderObject::Group {
                    children: children.iter().map(|child| {
                        self.build_widget_recursive(child, &ctx)
                    }).collect(),
                };
            }
            WidgetNode::None => "None",
        };

        println!("📦 Root widget type: {}", widget_type);

        match widget_node {
            WidgetNode::Leaf(render_obj) => render_obj,
            WidgetNode::Container { children } => {
                let mut child_objects = Vec::new();
                for child in children {
                    let child_obj = self.build_widget_recursive(&child, &ctx);
                    child_objects.push(child_obj);
                }
                RenderObject::group(child_objects)
            }
            WidgetNode::None => {
                println!("⚠️ None widget node");
                RenderObject::None
            }
        }
    }

    fn build_widget_recursive(&self, widget: &Box<dyn Widget>, parent_ctx: &BuildContext) -> RenderObject {
        let widget_node = widget.build(parent_ctx);

        match widget_node {
            WidgetNode::Leaf(render_obj) => render_obj,
            WidgetNode::Container { children } => {
                let mut child_objects = Vec::new();
                for child in children {
                    let child_obj = self.build_widget_recursive(&child, parent_ctx);
                    child_objects.push(child_obj);
                }
                RenderObject::group(child_objects)
            }
            WidgetNode::None => RenderObject::None,
        }
    }
}
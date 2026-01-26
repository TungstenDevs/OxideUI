use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

pub struct Sidebar {
    pub width: f32,
    pub position: SidebarPosition,
    pub collapsed: bool,
    pub collapsible: bool,
    pub children: Vec<Box<dyn Widget>>,
    pub on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SidebarPosition {
    Left,
    Right,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            width: 240.0,
            position: SidebarPosition::Left,
            collapsed: false,
            collapsible: true,
            children: Vec::new(),
            on_toggle: None,
            key: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            width: self.width,
            position: self.position,
            collapsed: self.collapsed,
            collapsible: self.collapsible,
            children: self
                .children
                .iter()
                .map(|child| child.clone_box())
                .collect(),
            on_toggle: self.on_toggle.as_ref().map(|cb| cb.clone()),
            key: self.key.clone(),
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn position(mut self, position: SidebarPosition) -> Self {
        self.position = position;
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Widget>>) -> Self {
        self.children = children;
        self
    }

    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_on_toggle<F>(mut self, callback: F) -> Self
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        self.on_toggle = Some(Arc::new(callback));
        self
    }



    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Sidebar {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let actual_width = if self.collapsed { 60.0 } else { self.width };

        let mut render_objects = Vec::new();

        // Sidebar background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, actual_width, ctx.constraints.max_height),
            theme.sidebar,
        ));

        // Sidebar border
        let border_side = match self.position {
            SidebarPosition::Left => Rect::new(actual_width - 1.0, 0.0, 1.0, ctx.constraints.max_height),
            SidebarPosition::Right => Rect::new(0.0, 0.0, 1.0, ctx.constraints.max_height),
        };
        render_objects.push(RenderObject::rect(
            border_side,
            theme.sidebar_border,
        ));

        // Toggle button if collapsible
        if self.collapsible {
            let toggle_button_size = 32.0;
            let toggle_x = (actual_width - toggle_button_size) / 2.0;
            let toggle_y = ctx.constraints.max_height - toggle_button_size - 16.0;

            render_objects.push(RenderObject::rect(
                Rect::new(toggle_x, toggle_y, toggle_button_size, toggle_button_size),
                theme.sidebar_accent,
            ));

            let arrow_icon = match (self.position, self.collapsed) {
                (SidebarPosition::Left, false) => "◀",
                (SidebarPosition::Left, true) => "▶",
                (SidebarPosition::Right, false) => "▶",
                (SidebarPosition::Right, true) => "◀",
            };

            render_objects.push(RenderObject::text(
                arrow_icon.to_string(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 16.0,
                    color: theme.sidebar_accent_foreground,
                    bold: true,
                    italic: false,
                },
                Point::new(toggle_x + 8.0, toggle_y + 8.0),
            ));
        }

        // Children (only show if not collapsed)
        if !self.collapsed && !self.children.is_empty() {
            let child_y = 20.0;
            let child_height = ctx.constraints.max_height - child_y - 80.0; // Space for toggle button

            for child in &self.children {
                let child_constraints = crate::layout::constraints::Constraints::new(
                    0.0,
                    actual_width - 20.0, // Padding
                    0.0,
                    child_height,
                );

                let child_ctx = ctx.child_context(ctx.element_id, child_constraints);
                let child_node = child.build(&child_ctx);

                if let WidgetNode::Leaf(render_obj) = child_node {
                    let offset_render_obj = RenderObject::transform(
                        crate::core::render_object::Matrix::translate(10.0, child_y),
                        render_obj,
                    );
                    render_objects.push(offset_render_obj);
                }
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Sidebar {
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
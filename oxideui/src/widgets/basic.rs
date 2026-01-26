use crate::core::context::BuildContext;
use crate::core::context::ThemeProvider;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::layout::constraints::{EdgeInsets};
use std::any::Any;

/// Container widget - flexible box with styling
pub struct Container {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub color: Option<Color>,
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Option<Color>,
    pub child: Option<WidgetChild>,
    pub tooltip: Option<String>,
    pub on_click: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

// Helper to avoid Clone requirement on Box<dyn Widget>
pub enum WidgetChild {
    Single(std::sync::Arc<dyn Widget>),
}

impl Clone for WidgetChild {
    fn clone(&self) -> Self {
        match self {
            WidgetChild::Single(w) => WidgetChild::Single(w.clone()),
        }
    }
}

impl Clone for Container {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            color: self.color,
            padding: self.padding,
            margin: self.margin,
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
            child: self.child.clone(),
            tooltip: self.tooltip.clone(),
            on_click: self.on_click.clone(),
            key: self.key.clone(),
        }
    }
}

impl Container {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            color: None,
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::zero(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: None,
            child: None,
            tooltip: None,
            on_click: None,
            key: None,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = EdgeInsets::all(padding);
        self
    }

    pub fn with_padding_all(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        self.padding = EdgeInsets::only(left, top, right, bottom);
        self
    }

    pub fn with_margin(mut self, margin: f32) -> Self {
        self.margin = EdgeInsets::all(margin);
        self
    }

    pub fn with_border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    pub fn with_border(mut self, width: f32, color: Color) -> Self {
        self.border_width = width;
        self.border_color = Some(color);
        self
    }

    pub fn with_child<W: Widget + 'static>(mut self, child: W) -> Self {
        self.child = Some(WidgetChild::Single(std::sync::Arc::new(child)));
        self
    }

    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn with_on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(std::sync::Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Container {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let bg_color = self.color.unwrap_or(theme.background);
        let border_color = self.border_color.unwrap_or(theme.border);

        let available_width = ctx.constraints.max_width - self.margin.horizontal();
        let available_height = ctx.constraints.max_height - self.margin.vertical();

        let width = self.width.unwrap_or(available_width);
        let height = self.height.unwrap_or(available_height);

        let mut render_objects = Vec::new();

        // Background
        render_objects.push(RenderObject::rect(
            Rect::new(self.padding.left, self.padding.top,
                      width - self.padding.horizontal(),
                      height - self.padding.vertical()),
            bg_color,
        ));

        // Border
        if self.border_width > 0.0 {
            let border_rect = Rect::new(
                self.padding.left - self.border_width/2.0,
                self.padding.top - self.border_width/2.0,
                width - self.padding.horizontal() + self.border_width,
                height - self.padding.vertical() + self.border_width
            );

            render_objects.push(RenderObject::rect(
                border_rect,
                border_color,
            ));
        }

        // Child
        if let Some(child) = &self.child {
            match child {
                WidgetChild::Single(widget) => {
                    let child_constraints = ctx.constraints.deflate(EdgeInsets {
                        left: self.padding.left + self.border_width,
                        top: self.padding.top + self.border_width,
                        right: self.padding.right + self.border_width,
                        bottom: self.padding.bottom + self.border_width,
                    });

                    let child_ctx = ctx.child_context(ctx.element_id, child_constraints);
                    let child_node = widget.build(&child_ctx);

                    if let WidgetNode::Leaf(child_render) = child_node {
                        render_objects.push(child_render);
                    }
                }
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Container {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(
        &self,
        event: &crate::core::event::UiEvent,
        context: &mut crate::core::event::EventContext,
    ) -> crate::core::event::EventResult {
        use crate::core::event::{EventResult, MouseButton, UiEvent};
        match event {
            UiEvent::PointerUp {
                button: MouseButton::Left,
                ..
            } if context.is_at_target() => {
                if let Some(on_click) = &self.on_click {
                    on_click();
                    return EventResult::Stopped;
                }
                EventResult::Unhandled
            }
            _ => EventResult::Unhandled,
        }
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

// Text Widget
pub struct Text {
    pub content: String,
    pub style: Option<TextStyle>,
    pub color: Option<Color>,
    key: Option<WidgetKey>,
}

impl Clone for Text {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            style: self.style.clone(),
            color: self.color,
            key: self.key.clone(),
        }
    }
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: None,
            color: None,
            key: None,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl StatelessWidget for Text {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let text_color = self.color.unwrap_or(theme.foreground);
        let style = self.style.clone().unwrap_or(TextStyle {
            font_family: theme.font_sans.clone(),
            font_size: 14.0,
            color: text_color,
            bold: false,
            italic: false,
        });

        WidgetNode::Leaf(RenderObject::text(
            self.content.clone(),
            style,
            Point::new(0.0, 0.0)
        ))
    }
}

impl Widget for Text {
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

// Column Widget
pub struct Column {
    pub children: Vec<std::sync::Arc<dyn Widget>>,
    pub spacing: f32,
    key: Option<WidgetKey>,
}

impl Clone for Column {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
            spacing: self.spacing,
            key: self.key.clone(),
        }
    }
}

impl Column {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
            key: None,
        }
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Widget>>) -> Self {
        self.children = children
            .into_iter()
            .map(|w| std::sync::Arc::from(w))
            .collect();
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Widget for Column {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        let mut accumulated_height = 0.0;
        let mut child_objects = Vec::new();

        for (i, child) in self.children.iter().enumerate() {
            let child_height = ctx.constraints.max_height - accumulated_height;
            let child_constraints = ctx.constraints.constrain_height(child_height);

            let child_ctx = ctx.child_context(
                crate::core::element::ElementId::new(i as u64 + 1),
                child_constraints
            );

            let child_node = child.build(&child_ctx);

            if let WidgetNode::Leaf(render_obj) = child_node {
                let transformed = RenderObject::transform(
                    crate::core::render_object::Matrix::translate(0.0, accumulated_height),
                    render_obj
                );
                child_objects.push(transformed);

                // Estimate height based on render object bounds
                accumulated_height += 50.0 + self.spacing; // Rough estimate
            }
        }

        WidgetNode::Leaf(RenderObject::group(child_objects))
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

// Row Widget
pub struct Row {
    pub children: Vec<std::sync::Arc<dyn Widget>>,
    pub spacing: f32,
    key: Option<WidgetKey>,
}

impl Clone for Row {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
            spacing: self.spacing,
            key: self.key.clone(),
        }
    }
}

impl Row {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
            key: None,
        }
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Widget>>) -> Self {
        self.children = children
            .into_iter()
            .map(|w| std::sync::Arc::from(w))
            .collect();
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Widget for Row {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        let mut accumulated_width = 0.0;
        let mut child_objects = Vec::new();

        for (i, child) in self.children.iter().enumerate() {
            let child_width = ctx.constraints.max_width - accumulated_width;
            let child_constraints = ctx.constraints.constrain_width(child_width);

            let child_ctx = ctx.child_context(
                crate::core::element::ElementId::new(i as u64 + 1),
                child_constraints
            );

            let child_node = child.build(&child_ctx);

            if let WidgetNode::Leaf(render_obj) = child_node {
                let transformed = RenderObject::transform(
                    crate::core::render_object::Matrix::translate(accumulated_width, 0.0),
                    render_obj
                );
                child_objects.push(transformed);

                // Estimate width based on render object bounds
                accumulated_width += 100.0 + self.spacing; // Rough estimate
            }
        }

        WidgetNode::Leaf(RenderObject::group(child_objects))
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

// Center Widget
pub struct Center {
    pub child: Option<std::sync::Arc<dyn Widget>>,
    key: Option<WidgetKey>,
}

impl Clone for Center {
    fn clone(&self) -> Self {
        Self {
            child: self.child.clone(),
            key: self.key.clone(),
        }
    }
}

impl Center {
    pub fn new() -> Self {
        Self {
            child: None,
            key: None,
        }
    }

    pub fn with_child<W: Widget + 'static>(mut self, child: W) -> Self {
        self.child = Some(std::sync::Arc::new(child));
        self
    }
}

impl Widget for Center {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        let mut child_objects = Vec::new();

        if let Some(child) = &self.child {
            let child_ctx = ctx.child_context(
                crate::core::element::ElementId::new(1),
                ctx.constraints
            );

            let child_node = child.build(&child_ctx);

            if let WidgetNode::Leaf(render_obj) = child_node {
                // Center the child by translating it to the center
                let translate_x = (ctx.constraints.max_width - 100.0) / 2.0; // Rough estimation
                let translate_y = (ctx.constraints.max_height - 50.0) / 2.0; // Rough estimation

                let transformed = RenderObject::transform(
                    crate::core::render_object::Matrix::translate(translate_x, translate_y),
                    render_obj
                );
                child_objects.push(transformed);
            }
        }

        WidgetNode::Leaf(RenderObject::group(child_objects))
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

// Add a test widget that actually renders something visible
pub struct HelloWorld {
    key: Option<WidgetKey>,
}

impl HelloWorld {
    pub fn new() -> Self {
        Self { key: None }
    }
}

impl Widget for HelloWorld {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();

        let mut objects = Vec::new();

        // Background container
        objects.push(RenderObject::rect(
            Rect::new(50.0, 50.0, 300.0, 200.0),
            theme.card,
        ));

        // Border
        objects.push(RenderObject::rect(
            Rect::new(50.0, 50.0, 300.0, 1.0),
            theme.border,
        ));
        objects.push(RenderObject::rect(
            Rect::new(50.0, 249.0, 300.0, 1.0),
            theme.border,
        ));

        // Title text
        objects.push(RenderObject::text(
            "Hello, OxideUI!".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 24.0,
                color: theme.primary,
                bold: true,
                italic: false,
            },
            Point::new(70.0, 80.0),
        ));

        // Message text
        objects.push(RenderObject::text(
            "Widgets are now rendering correctly!".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 16.0,
                color: theme.foreground,
                bold: false,
                italic: false,
            },
            Point::new(70.0, 120.0),
        ));

        WidgetNode::Leaf(RenderObject::group(objects))
    }

    fn key(&self) -> Option<WidgetKey> {
        self.key.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Widget> {
        Box::new(Self { key: self.key.clone() })
    }
}
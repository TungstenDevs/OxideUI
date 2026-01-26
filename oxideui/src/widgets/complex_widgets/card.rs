use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

pub struct Card {
    pub title: Option<String>,
    pub description: Option<String>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub padding: f32,
    pub variant: CardVariant,
    pub children: Vec<Box<dyn Widget>>,
    pub on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardVariant {
    Default,
    Outlined,
    Elevated,
    Filled,
}

impl Card {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            width: None,
            height: None,
            padding: 16.0,
            variant: CardVariant::Default,
            children: Vec::new(),
            on_click: None,
            tooltip: None,
            key: None,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            description: self.description.clone(),
            width: self.width,
            height: self.height,
            padding: self.padding,
            variant: self.variant,
            children: self
                .children
                .iter()
                .map(|child| child.clone_box())
                .collect(),
            on_click: self.on_click.as_ref().map(|cb| cb.clone()),
            tooltip: self.tooltip.clone(),
            key: self.key.clone(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
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

    pub fn with_on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Arc::new(callback));
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

impl StatelessWidget for Card {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(300.0);
        let height = self.height.unwrap_or(200.0);

        let bg_color = match self.variant {
            CardVariant::Default => theme.card,
            CardVariant::Outlined => theme.background,
            CardVariant::Elevated => theme.card,
            CardVariant::Filled => theme.muted,
        };

        let border_color = if self.variant == CardVariant::Outlined {
            theme.border
        } else {
            Color::TRANSPARENT
        };

        let mut render_objects = Vec::new();

        // Card background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Card border (if outlined)
        if self.variant == CardVariant::Outlined {
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, 0.0, width, 1.0),
                border_color,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(width - 1.0, 0.0, 1.0, height),
                border_color,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height - 1.0, width, 1.0),
                border_color,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, 0.0, 1.0, height),
                border_color,
            ));
        }

        let mut current_y = self.padding;

        // Title
        if let Some(title) = &self.title {
            render_objects.push(RenderObject::text(
                title.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 18.0,
                    color: theme.card_foreground,
                    bold: true,
                    italic: false,
                },
                Point::new(self.padding, current_y),
            ));
            current_y += 24.0;
        }

        // Description
        if let Some(description) = &self.description {
            render_objects.push(RenderObject::text(
                description.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 14.0,
                    color: theme.muted_foreground,
                    bold: false,
                    italic: false,
                },
                Point::new(self.padding, current_y),
            ));
            current_y += 20.0;
        }

        // Children
        if !self.children.is_empty() {
            let child_y = current_y;
            let child_height = height - child_y - self.padding;

            for child in &self.children {
                let child_constraints = crate::layout::constraints::Constraints::new(
                    0.0,
                    width - (self.padding * 2.0),
                    0.0,
                    child_height,
                );

                let child_ctx = ctx.child_context(ctx.element_id, child_constraints);
                let child_node = child.build(&child_ctx);

                if let WidgetNode::Leaf(render_obj) = child_node {
                    let offset_render_obj = RenderObject::transform(
                        crate::core::render_object::Matrix::translate(self.padding, child_y),
                        render_obj,
                    );
                    render_objects.push(offset_render_obj);
                }
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Card {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        match event {
            UiEvent::PointerUp { button: MouseButton::Left, .. } if context.is_at_target() => {
                if let Some(on_click) = &self.on_click {
                    on_click();
                    EventResult::Stopped
                } else {
                    EventResult::Unhandled
                }
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
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

pub struct Tabs {
    pub tabs: Vec<String>,
    pub active: usize,
    pub orientation: TabOrientation,
    pub variant: TabVariant,
    pub on_tab_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub children: Vec<Box<dyn Widget>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabVariant {
    Default,
    Underline,
    Pills,
    Cards,
}

impl Tabs {
    pub fn new(tabs: Vec<String>) -> Self {
        Self {
            tabs,
            active: 0,
            orientation: TabOrientation::Horizontal,
            variant: TabVariant::Default,
            on_tab_change: None,
            children: Vec::new(),
            key: None,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            tabs: self.tabs.clone(),
            active: self.active,
            orientation: self.orientation,
            variant: self.variant,
            on_tab_change: self.on_tab_change.as_ref().map(|cb| cb.clone()),
            children: self
                .children
                .iter()
                .map(|child| child.clone_box())
                .collect(),
            key: self.key.clone(),
        }
    }

    pub fn active(mut self, index: usize) -> Self {
        self.active = index;
        self
    }

    pub fn with_orientation(mut self, orientation: TabOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_on_tab_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        self.on_tab_change = Some(Arc::new(callback));
        self
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Widget>>) -> Self {
        self.children = children;
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Tabs {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let tab_height = 40.0;
        let tab_padding = 16.0;

        let mut render_objects = Vec::new();

        // Calculate tab widths based on text length
        let tab_widths: Vec<f32> = self.tabs.iter()
            .map(|tab| (tab.len() as f32 * 8.0) + (tab_padding * 2.0))
            .collect();

        let total_width: f32 = tab_widths.iter().sum();
        let mut current_x = 0.0;

        // Tab headers
        for (i, (tab, &width)) in self.tabs.iter().zip(tab_widths.iter()).enumerate() {
            let is_active = i == self.active;

            let bg_color = match (self.variant, is_active) {
                (TabVariant::Default, true) => theme.primary,
                (TabVariant::Default, false) => Color::TRANSPARENT,
                (TabVariant::Underline, _) => Color::TRANSPARENT,
                (TabVariant::Pills, true) => theme.primary,
                (TabVariant::Pills, false) => theme.muted,
                (TabVariant::Cards, true) => theme.card,
                (TabVariant::Cards, false) => Color::TRANSPARENT,
            };

            let text_color = match (self.variant, is_active) {
                (TabVariant::Default, true) => theme.primary_foreground,
                (TabVariant::Default, false) => theme.foreground,
                (TabVariant::Underline, true) => theme.primary,
                (TabVariant::Underline, false) => theme.muted_foreground,
                (TabVariant::Pills, true) => theme.primary_foreground,
                (TabVariant::Pills, false) => theme.foreground,
                (TabVariant::Cards, true) => theme.card_foreground,
                (TabVariant::Cards, false) => theme.foreground,
            };

            // Tab background
            if bg_color != Color::TRANSPARENT {
                render_objects.push(RenderObject::rect(
                    Rect::new(current_x, 0.0, width, tab_height),
                    bg_color,
                ));
            }

            // Tab text
            render_objects.push(RenderObject::text(
                tab.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 14.0,
                    color: text_color,
                    bold: is_active,
                    italic: false,
                },
                Point::new(current_x + tab_padding, tab_height / 2.0 + 5.0),
            ));

            // Underline for active tab (if variant is Underline)
            if self.variant == TabVariant::Underline && is_active {
                render_objects.push(RenderObject::rect(
                    Rect::new(current_x, tab_height - 2.0, width, 2.0),
                    theme.primary,
                ));
            }

            // Border for Cards variant
            if self.variant == TabVariant::Cards {
                render_objects.push(RenderObject::rect(
                    Rect::new(current_x, 0.0, width, 1.0),
                    theme.border,
                ));
                render_objects.push(RenderObject::rect(
                    Rect::new(current_x + width - 1.0, 0.0, 1.0, tab_height),
                    theme.border,
                ));
                render_objects.push(RenderObject::rect(
                    Rect::new(current_x, tab_height - 1.0, width, 1.0),
                    theme.border,
                ));
                render_objects.push(RenderObject::rect(
                    Rect::new(current_x, 0.0, 1.0, tab_height),
                    theme.border,
                ));
            }

            current_x += width;
        }

        // Active content area (below tabs)
        if let Some(child) = self.children.get(self.active) {
            let content_y = tab_height + 16.0;
            let content_height = ctx.constraints.max_height - content_y;

            // Build child in content area
            let child_constraints = crate::layout::constraints::Constraints::new(
                0.0,
                total_width,
                0.0,
                content_height,
            );

            let child_ctx = ctx.child_context(ctx.element_id, child_constraints);
            let child_node = child.build(&child_ctx);

            if let WidgetNode::Leaf(render_obj) = child_node {
                // Offset child to content area
                let offset_render_obj = RenderObject::transform(
                    crate::core::render_object::Matrix::translate(0.0, content_y),
                    render_obj,
                );
                render_objects.push(offset_render_obj);
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Tabs {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() => {
                let tab_height = 40.0;
                let tab_padding = 16.0;

                // Calculate which tab was clicked
                let mut current_x = 0.0;
                for (i, tab) in self.tabs.iter().enumerate() {
                    let width = (tab.len() as f32 * 8.0) + (tab_padding * 2.0);

                    let tab_rect = Rect::new(current_x, 0.0, width, tab_height);
                    if tab_rect.contains(position.x, position.y) && i != self.active {
                        if let Some(on_change) = &self.on_tab_change {
                            on_change(i);
                        }
                        return EventResult::Stopped;
                    }

                    current_x += width;
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
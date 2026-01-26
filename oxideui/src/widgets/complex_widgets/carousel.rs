use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

pub struct Carousel {
    pub items: Vec<Box<dyn Widget>>,
    pub current_index: usize,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub autoplay: bool,
    pub interval_ms: u64,
    pub show_indicators: bool,
    pub show_navigation: bool,
    pub on_index_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    key: Option<WidgetKey>,
}

impl Carousel {
    pub fn new(items: Vec<Box<dyn Widget>>) -> Self {
        Self {
            items,
            current_index: 0,
            width: None,
            height: None,
            autoplay: false,
            interval_ms: 3000,
            show_indicators: true,
            show_navigation: true,
            on_index_change: None,
            key: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            items: self.items.iter().map(|item| item.clone_box()).collect(),
            current_index: self.current_index,
            width: self.width,
            height: self.height,
            autoplay: self.autoplay,
            interval_ms: self.interval_ms,
            show_indicators: self.show_indicators,
            show_navigation: self.show_navigation,
            on_index_change: self.on_index_change.clone(),
            key: self.key.clone(),
        }
    }

    pub fn current_index(mut self, index: usize) -> Self {
        self.current_index = index;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn autoplay(mut self, autoplay: bool) -> Self {
        self.autoplay = autoplay;
        self
    }

    pub fn with_interval(mut self, interval_ms: u64) -> Self {
        self.interval_ms = interval_ms;
        self
    }

    pub fn show_indicators(mut self, show: bool) -> Self {
        self.show_indicators = show;
        self
    }

    pub fn show_navigation(mut self, show: bool) -> Self {
        self.show_navigation = show;
        self
    }

    pub fn with_on_index_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        self.on_index_change = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Carousel {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(400.0);
        let height = self.height.unwrap_or(300.0);

        let mut render_objects = Vec::new();

        // Carousel container
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            theme.background,
        ));

        // Current item
        if let Some(item) = self.items.get(self.current_index) {
            let child_constraints = crate::layout::constraints::Constraints::new(
                0.0,
                width,
                0.0,
                height,
            );

            let child_ctx = ctx.child_context(ctx.element_id, child_constraints);
            let child_node = item.build(&child_ctx);

            if let WidgetNode::Leaf(render_obj) = child_node {
                render_objects.push(render_obj);
            }
        }

        // Navigation buttons
        if self.show_navigation && self.items.len() > 1 {
            let button_size = 40.0;
            let button_color = theme.primary.with_alpha(200);

            // Previous button
            render_objects.push(RenderObject::rect(
                Rect::new(10.0, height / 2.0 - button_size / 2.0, button_size, button_size),
                button_color,
            ));

            render_objects.push(RenderObject::text(
                "◀".to_string(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 20.0,
                    color: theme.primary_foreground,
                    bold: true,
                    italic: false,
                },
                Point::new(20.0, height / 2.0 + 5.0),
            ));

            // Next button
            render_objects.push(RenderObject::rect(
                Rect::new(width - button_size - 10.0, height / 2.0 - button_size / 2.0, button_size, button_size),
                button_color,
            ));

            render_objects.push(RenderObject::text(
                "▶".to_string(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 20.0,
                    color: theme.primary_foreground,
                    bold: true,
                    italic: false,
                },
                Point::new(width - button_size, height / 2.0 + 5.0),
            ));
        }

        // Indicators
        if self.show_indicators && self.items.len() > 1 {
            let indicator_size = 8.0;
            let indicator_spacing = 12.0;
            let total_width = (self.items.len() as f32 * indicator_size) +
                ((self.items.len() - 1) as f32 * indicator_spacing);
            let start_x = (width - total_width) / 2.0;

            for i in 0..self.items.len() {
                let is_active = i == self.current_index;
                let indicator_color = if is_active {
                    theme.primary
                } else {
                    theme.muted
                };

                let x = start_x + (i as f32 * (indicator_size + indicator_spacing));
                let y = height - 20.0;

                render_objects.push(RenderObject::rect(
                    Rect::new(x, y, indicator_size, indicator_size),
                    indicator_color,
                ));
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Carousel {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() => {
                let width = self.width.unwrap_or(400.0);
                let height = self.height.unwrap_or(300.0);

                // Check navigation buttons
                let button_size = 40.0;

                // Previous button
                let prev_button_rect = Rect::new(10.0, height / 2.0 - button_size / 2.0, button_size, button_size);
                if prev_button_rect.contains(position.x, position.y) && self.current_index > 0 {
                    let new_index = self.current_index - 1;
                    if let Some(on_change) = &self.on_index_change {
                        on_change(new_index);
                    }
                    return EventResult::Stopped;
                }

                // Next button
                let next_button_rect = Rect::new(width - button_size - 10.0, height / 2.0 - button_size / 2.0, button_size, button_size);
                if next_button_rect.contains(position.x, position.y) && self.current_index < self.items.len() - 1 {
                    let new_index = self.current_index + 1;
                    if let Some(on_change) = &self.on_index_change {
                        on_change(new_index);
                    }
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
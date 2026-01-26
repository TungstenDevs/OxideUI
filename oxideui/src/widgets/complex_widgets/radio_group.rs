use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct RadioGroup {
    pub options: Vec<String>,
    pub selected: Option<usize>,
    pub orientation: Orientation,
    pub disabled: bool,
    pub on_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl RadioGroup {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected: None,
            orientation: Orientation::Vertical,
            disabled: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        self.on_change = Some(Arc::new(callback));
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

impl StatelessWidget for RadioGroup {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let radio_size = 20.0;
        let spacing = match self.orientation {
            Orientation::Horizontal => 24.0,
            Orientation::Vertical => 16.0,
        };

        let mut render_objects = Vec::new();
        let mut current_x = 0.0;
        let mut current_y = 0.0;

        for (i, option) in self.options.iter().enumerate() {
            let is_selected = self.selected == Some(i);
            let is_disabled = self.disabled;

            let circle_color = if is_disabled {
                theme.muted
            } else if is_selected {
                theme.primary
            } else {
                theme.border
            };

            let dot_color = if is_disabled {
                theme.muted_foreground
            } else {
                theme.primary_foreground
            };

            let text_color = if is_disabled {
                theme.muted_foreground
            } else {
                theme.foreground
            };

            // Radio circle
            render_objects.push(RenderObject::rect(
                Rect::new(current_x, current_y, radio_size, radio_size),
                circle_color,
            ));

            // Radio dot (if selected)
            if is_selected {
                let dot_size = radio_size / 2.0;
                let dot_offset = (radio_size - dot_size) / 2.0;
                render_objects.push(RenderObject::rect(
                    Rect::new(current_x + dot_offset, current_y + dot_offset, dot_size, dot_size),
                    dot_color,
                ));
            }

            // Option label
            render_objects.push(RenderObject::text(
                option.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 14.0,
                    color: text_color,
                    bold: false,
                    italic: false,
                },
                Point::new(current_x + radio_size + 8.0, current_y + radio_size / 2.0 + 5.0),
            ));

            // Update position for next option
            match self.orientation {
                Orientation::Horizontal => {
                    let option_width = radio_size + 8.0 + (option.len() as f32 * 7.0);
                    current_x += option_width + spacing;
                }
                Orientation::Vertical => {
                    current_y += radio_size + spacing;
                }
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for RadioGroup {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() && !self.disabled => {
                let radio_size = 20.0;
                let mut current_y = 0.0;

                for i in 0..self.options.len() {
                    let radio_rect = Rect::new(0.0, current_y, radio_size, radio_size);
                    if radio_rect.contains(position.x, position.y) {
                        if let Some(on_change) = &self.on_change {
                            on_change(i);
                        }
                        return EventResult::Stopped;
                    }

                    current_y += radio_size + 16.0; // Assuming vertical layout
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
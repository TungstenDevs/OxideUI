use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Switch {
    pub checked: bool,
    pub label: Option<String>,
    pub disabled: bool,
    pub on_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Switch {
    pub fn new() -> Self {
        Self {
            checked: false,
            label: None,
            disabled: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        self.on_change = Some(Arc::new(callback));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

impl StatelessWidget for Switch {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = 44.0;
        let height = 24.0;
        let thumb_size = 16.0;
        let padding = (height - thumb_size) / 2.0;

        let thumb_position = if self.checked {
            width - thumb_size - padding
        } else {
            padding
        };

        let track_color = if self.disabled {
            theme.muted
        } else if self.checked {
            theme.primary
        } else {
            theme.border
        };

        let thumb_color = if self.disabled {
            theme.muted_foreground
        } else {
            theme.background
        };

        let mut render_objects = Vec::new();

        // Track
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            track_color,
        ));

        // Thumb
        render_objects.push(RenderObject::rect(
            Rect::new(thumb_position, padding, thumb_size, thumb_size),
            thumb_color,
        ));

        // Label
        if let Some(label) = &self.label {
            render_objects.push(RenderObject::text(
                label.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 14.0,
                    color: theme.foreground,
                    bold: false,
                    italic: false,
                },
                Point::new(width + 8.0, height / 2.0 + 5.0),
            ));
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Switch {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        match event {
            UiEvent::PointerUp { button: MouseButton::Left, .. } if context.is_at_target() && !self.disabled => {
                // Toggle the switch
                if let Some(on_change) = &self.on_change {
                    on_change(!self.checked);
                }
                EventResult::Stopped
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
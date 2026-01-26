//! Button widget - demonstrates event handling

use std::any::Any;
use std::sync::Arc;

use crate::core::context::BuildContext;
use crate::core::event::{EventContext, EventPhase, EventResult, MouseButton, UiEvent};
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::layout::constraints::Size;

/// Callback type for button clicks
pub type OnClick = Arc<dyn Fn() + Send + Sync>;

/// Button widget with event handling
pub struct Button {
    pub label: String,
    pub on_click: Option<OnClick>,
    pub color: Color,
    pub text_color: Color,
    pub width: Option<f32>,
    pub height: Option<f32>,
    key: Option<WidgetKey>,
}

impl Clone for Button {
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            on_click: self.on_click.clone(),
            color: self.color,
            text_color: self.text_color,
            width: self.width,
            height: self.height,
            key: self.key.clone(),
        }
    }
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            on_click: None,
            color: Color::from_hex(0x2196F3), // Material blue
            text_color: Color::WHITE,
            width: None,
            height: None,
            key: None,
        }
    }

    pub fn with_on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Arc::new(callback));
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Button {
    fn build_stateless(&self, _ctx: &BuildContext) -> WidgetNode {
        let width = self.width.unwrap_or(120.0);
        let height = self.height.unwrap_or(40.0);

        let size = Size::new(width, height);
        let rect = Rect::from_size(size);

        // Create button background
        let background = RenderObject::rect(rect, self.color);

        // Create button text
        let text_style = TextStyle {
            font_family: "sans-serif".to_string(),
            font_size: 14.0,
            color: self.text_color,
            bold: false,
            italic: false,
        };

        // Center text in button (rough approximation)
        let text_x = rect.width / 2.0 - (self.label.len() as f32 * 4.0);
        let text_y = rect.height / 2.0 + 5.0;
        let text = RenderObject::text(
            self.label.clone(),
            text_style,
            Point::new(text_x, text_y),
        );

        // Group background and text
        WidgetNode::Leaf(RenderObject::group(vec![background, text]))
    }
}

impl Widget for Button {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &UiEvent, context: &mut EventContext) -> EventResult {
        match event {
            UiEvent::PointerDown {
                button: MouseButton::Left,
                ..
            } => {
                // Visual feedback on mouse down
                println!("Button '{}' pressed", self.label);
                EventResult::Handled // Continue propagation for hover effects
            }
            UiEvent::PointerUp {
                button: MouseButton::Left,
                ..
            } if context.phase == EventPhase::AtTarget => {
                // Execute callback on release (standard button behavior)
                println!("Button '{}' clicked!", self.label);

                if let Some(on_click) = &self.on_click {
                    on_click();
                }

                EventResult::Stopped // Stop propagation - button consumed the click
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new("Click me");
        assert_eq!(button.label, "Click me");
        assert!(button.on_click.is_none());
    }

    #[test]
    fn test_button_with_callback() {
        let clicked = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let clicked_clone = clicked.clone();

        let button = Button::new("Test").with_on_click(move || {
            clicked_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        });

        // Simulate click
        if let Some(on_click) = &button.on_click {
            on_click();
        }

        assert!(clicked.load(std::sync::atomic::Ordering::SeqCst));
    }
}
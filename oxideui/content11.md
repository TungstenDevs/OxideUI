
## File: `./src/lib.rs`
```rs
//! # OxideUI - Production-Ready UI Framework
//!
//! OxideUI is a modern, declarative UI framework for Rust with production-grade features:
//!
//! ## Core Features
//! - **Complete Widget System**: Stateful and stateless widgets with full lifecycle
//! - **GPU-Accelerated Rendering**: Skia-based with OpenGL/CPU fallbacks
//! - **Advanced Text Rendering**: Font management, shaping, and caching
//! - **Event System**: Gesture recognition, focus management, and accessibility
//! - **State Management**: Fine-grained reactivity with automatic rebuilds
//! - **Layout Engine**: Flexbox, grid, and constraint-based layouts
//! - **Scrolling**: Momentum scrolling, snap points, and virtual lists
//! - **Animation System**: Springs, easing curves, and keyframe animations
//! - **Accessibility**: Screen reader support and ARIA-compliant
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use oxideui::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let app = MyApp::new();
//!
//!     oxideui::new(app)
//!         .with_title("My App")
//!         .with_size(1200, 800)
//!         .run()
//!         .await
//! }
//!
//! struct MyApp;
//!
//! impl Widget for MyApp {
//!     fn build(&self, ctx: &BuildContext) -> WidgetNode {
//!         WidgetNode::Container {
//!             children: vec![
//!                 Box::new(Text::new("Hello, OxideUI!")),
//!             ],
//!         }
//!     }
//!
//!     fn key(&self) -> Option<WidgetKey> { None }
//!     fn as_any(&self) -> &dyn Any { self }
//!     fn clone_box(&self) -> Box<dyn Widget> { Box::new(Self) }
//! }
//! ```
//!
//! ## Production Systems
//!
//! ### Rendering Pipeline
//! ```rust,no_run
//! use oxideui::render::pipeline::RenderPipeline;
//! use oxideui::core::render_object::Rect;
//!
//! let viewport = Rect::new(0.0, 0.0, 800.0, 600.0);
//! let mut pipeline = RenderPipeline::new(viewport);
//!
//! // Mark dirty regions for efficient redraws
//! pipeline.mark_dirty(element_id, dirty_rect);
//!
//! // Build display list with culling
//! pipeline.build_display_list(&root_render_object);
//! ```
//!
//! ### Text Rendering
//! ```rust,no_run
//! use oxideui::render::text::{FontManager, TextLayout};
//!
//! let font_manager = FontManager::new();
//! let layout = TextLayout::new(Arc::new(font_manager));
//!
//! // Multi-line text with wrapping
//! let lines = layout.layout_text(
//!     "Long text...",
//!     &text_style,
//!     Some(max_width)
//! )?;
//! ```
//!
//! ### Event Handling
//! ```rust,no_run
//! use oxideui::core::event_system::{GestureRecognizer, FocusManager};
//!
//! let mut gestures = GestureRecognizer::new();
//! let mut focus = FocusManager::new();
//!
//! // Automatic gesture detection
//! if let Some(gesture) = gestures.handle_pointer_down(id, pos) {
//!     match gesture {
//!         GestureType::Tap => println!("Tapped!"),
//!         GestureType::Pan => println!("Panning!"),
//!         _ => {}
//!     }
//! }
//!
//! // Keyboard navigation
//! focus.focus_next();
//! focus.add_focus_listener(element, |focused| {
//!     println!("Focus changed: {}", focused);
//! });
//! ```
//!
//! ### State Management
//! ```rust,no_run
//! use oxideui::core::state_driven::{ReactiveState, StateTracker};
//!
//! let tracker = Arc::new(StateTracker::new());
//! let state = ReactiveState::new(0, tracker.clone());
//!
//! // Subscribe element to state
//! state.subscribe(element_id);
//!
//! // Update triggers automatic rebuild
//! state.set(42);
//! state.update(|val| *val += 1);
//! ```
//!
//! ### Animations
//! ```rust,no_run
//! use oxideui::animation::{Animation, EasingCurve};
//! use std::time::Duration;
//!
//! let mut anim = Animation::new(0.0, 100.0, Duration::from_millis(300))
//!     .with_curve(EasingCurve::Spring {
//!         damping: 0.7,
//!         stiffness: 200.0,
//!     })
//!     .with_on_complete(|| println!("Done!"));
//!
//! // Update in frame loop
//! anim.update();
//! let current = anim.current_value();
//! ```
//!
//! ### Scrolling
//! ```rust,no_run
//! use oxideui::widgets::scrolling::{ScrollController, ScrollPhysics};
//!
//! let mut scroll = ScrollController::new();
//! scroll.physics = ScrollPhysics::Bouncing;
//!
//! // Handle scroll events
//! scroll.scroll(Vector2::new(0.0, -50.0));
//!
//! // Update momentum in frame loop
//! scroll.update_momentum(dt);
//! ```
//!
//! ## Performance
//!
//! OxideUI is optimized for production use:
//! - **Damage Tracking**: Only redraws changed regions
//! - **Display List Culling**: Skips offscreen widgets
//! - **Text Caching**: Shaped text is cached for reuse
//! - **Virtual Scrolling**: Efficient rendering of large lists
//! - **Lazy Layout**: Layout only when needed
//!
//! ## Accessibility
//!
//! Built-in accessibility support:
//! ```rust,no_run
//! use oxideui::core::event_system::{AccessibilityManager, AccessibilityRole};
//!
//! let mut a11y = AccessibilityManager::new();
//! a11y.set_label(element, "Submit button".to_string());
//! a11y.set_role(element, AccessibilityRole::Button);
//! a11y.announce("Form submitted successfully");
//! ```
pub mod core;
pub mod layout;
pub mod platform;
pub mod render;
pub mod runtime;
pub mod widgets;
pub mod state_management;
pub mod theming;
pub mod animation;
pub mod production;

// Core re-exports
pub use core::{BuildContext, Color, RenderObject, Theme};
pub use core::context::ThemeProvider;
pub use core::{StatefulWidget, StatelessWidget, Widget, WidgetKey, WidgetNode, WidgetState};
pub use core::event::{UiEvent, EventResult, MouseButton, Vector2, Modifiers};
// Layout re-exports
pub use layout::{Alignment, Constraints, EdgeInsets, Size};
// Runtime re-exports
pub use runtime::Runtime;
// Theming re-exports
pub use theming::{ThemeConfig, ThemeColors, load_theme_from_file};
// Widget re-exports
pub use widgets::basic::{Container, Text, Column, Row};
pub use widgets::element_widgets::*;
pub use widgets::layout_widgets::*;
pub use widgets::complex_widgets::*;
// State re-exports
pub use state_management::state::State;
// Production system re-exports (conditionally compiled)
#[cfg(feature = "production")]
pub use production::{ProductionRuntime, ProductionRuntimeBuilder};

// Animation re-exports (conditionally compiled)
#[cfg(any(feature = "skia-opengl", feature = "skia-cpu"))]
pub use animation::Animation;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        Widget, WidgetNode, WidgetKey, BuildContext, Color, Theme,
        Container, Text, Column, Row, Button, Label,
        Constraints, Size, Alignment, EdgeInsets,
        State,
    };
    pub use std::any::Any;
}

/// Create a new OxideUI application with a root widget
pub fn new<W: Widget + 'static>(root_widget: W) -> Runtime {
    Runtime::new(Box::new(root_widget))
}

/// Create a new OxideUI application with custom theme
pub fn new_with_theme<W: Widget + 'static>(root_widget: W, theme_path: &str) -> Runtime {
    let mut runtime = Runtime::new(Box::new(root_widget));
    if let Ok(theme_config) = load_theme_from_file(theme_path) {
        runtime = runtime.with_theme(theme_config);
    }
    runtime
}

/// Create production runtime with builder pattern (conditionally compiled)
#[cfg(feature = "production")]
pub fn production() -> ProductionRuntimeBuilder {
    ProductionRuntimeBuilder::new()
}
```

## File: `./src/widgets/mod.rs`
```rs
pub mod basic;
pub mod complex_layout_widgets;
pub mod complex_widgets;
pub mod element_widgets;
pub mod layout_widgets;
pub(crate) mod scrolling;

pub use basic::Container;
pub use complex_layout_widgets::*;
pub use complex_widgets::*;
pub use element_widgets::*;
pub use layout_widgets::*;
pub use crate::widgets::scrolling::{ScrollController, ScrollPhysics, ClipManager};
```

## File: `./src/widgets/element_widgets/mod.rs`
```rs
pub mod button;
pub mod headings;
pub mod label;
pub mod text_input;
pub mod checkbox;
pub mod image;
pub mod table;
pub mod textarea;
pub mod tooltip;
pub mod video;

pub use button::Button;
pub use headings::{Heading, h1, h2, h3, h4, h5, h6};
pub use label::Label;
pub use text_input::TextInput;
pub use checkbox::Checkbox;
pub use textarea::Textarea;
pub use tooltip::{Tooltip, TooltipPlacement};
pub use image::{Image, ImageFit};
pub use table::{Table, TableColumn, TableRow, ColumnWidth, TableAlign, SortDirection};
pub use video::Video;
```

## File: `./src/widgets/element_widgets/text_input.rs`
```rs
use crate::core::*;
use crate::core::render_object::{Point, Rect, TextStyle};
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct TextInput {
    pub placeholder: String,
    pub value: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub disabled: bool,
    pub on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl TextInput {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            value: String::new(),
            width: None,
            height: Some(40.0),
            disabled: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.on_change = Some(Arc::new(callback));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_tooltip(mut self, text: impl Into<String>) -> Self {
        self.tooltip = Some(text.into());
        self
    }
}

impl StatelessWidget for TextInput {
    fn build_stateless(&self, _ctx: &BuildContext) -> WidgetNode {
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(40.0);

        let bg_color = if self.disabled {
            Color::from_hex(0xF3F4F6)
        } else {
            Color::WHITE
        };

        let mut render_objects = Vec::new();

        // Background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Border
        let border_color = Color::from_hex(0xE5E7EB);
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, 1.0),
            border_color,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, height - 1.0, width, 1.0),
            border_color,
        ));

        // Text
        let text = if self.value.is_empty() {
            &self.placeholder
        } else {
            &self.value
        };

        let text_color = if self.value.is_empty() {
            Color::from_hex(0x9CA3AF)
        } else {
            Color::from_hex(0x111827)
        };

        render_objects.push(RenderObject::text(
            text.clone(),
            TextStyle {
                font_family: "Inter".to_string(),
                font_size: 14.0,
                color: text_color,
                bold: false,
                italic: false,
            },
            Point::new(12.0, height / 2.0 + 5.0),
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for TextInput {
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
```

## File: `./src/widgets/element_widgets/button.rs`
```rs
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
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
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
```

## File: `./src/widgets/element_widgets/headings.rs`
```rs
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Heading {
    pub text: String,
    pub level: u8,
    pub color: Option<Color>,
    pub align: crate::layout::Alignment,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Heading {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            level: 1,
            color: None,
            align: crate::layout::Alignment::TopLeft,
            tooltip: None,
            key: None,
        }
    }

    pub fn level(mut self, level: u8) -> Self {
        self.level = level.min(6).max(1);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn align(mut self, align: crate::layout::Alignment) -> Self {
        self.align = align;
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

impl StatelessWidget for Heading {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let font_sizes = [48.0, 36.0, 30.0, 24.0, 20.0, 18.0];
        let font_weights = [true, true, true, false, false, false];

        let idx = (self.level - 1) as usize;
        let font_size = font_sizes.get(idx).copied().unwrap_or(16.0);
        let is_bold = font_weights.get(idx).copied().unwrap_or(false);

        let text_color = self.color.unwrap_or_else(|| {
            Color::from_hex(if theme.is_dark { 0xFFFFFF } else { 0x000000 })
        });

        WidgetNode::Leaf(RenderObject::text(
            self.text.clone(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size,
                color: text_color,
                bold: is_bold,
                italic: false,
            },
            Point::ZERO,
        ))
    }
}

impl Widget for Heading {
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

// Convenience constructors
pub fn h1(text: impl Into<String>) -> Heading {
    Heading::new(text).level(1)
}

pub fn h2(text: impl Into<String>) -> Heading {
    Heading::new(text).level(2)
}

pub fn h3(text: impl Into<String>) -> Heading {
    Heading::new(text).level(3)
}

pub fn h4(text: impl Into<String>) -> Heading {
    Heading::new(text).level(4)
}

pub fn h5(text: impl Into<String>) -> Heading {
    Heading::new(text).level(5)
}

pub fn h6(text: impl Into<String>) -> Heading {
    Heading::new(text).level(6)
}
```

## File: `./src/widgets/element_widgets/checkbox.rs`
```rs
use crate::core::*;
use crate::core::render_object::{Point, Rect, TextStyle};  // Use OUR Rect
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct Checkbox {
    pub checked: bool,
    pub label: Option<String>,
    pub disabled: bool,
    pub on_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Checkbox {
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

    pub fn with_tooltip(mut self, text: impl Into<String>) -> Self {
        self.tooltip = Some(text.into());
        self
    }
}

impl StatelessWidget for Checkbox {
    fn build_stateless(&self, _ctx: &BuildContext) -> WidgetNode {
        let size = 20.0;
        let mut render_objects = Vec::new();

        let bg_color = if self.checked {
            Color::from_hex(0xD87943)
        } else {
            Color::WHITE
        };

        // Use OUR Rect type
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, size, size),
            bg_color,
        ));

        // Border
        let border_color = if self.checked {
            Color::from_hex(0xD87943)
        } else {
            Color::from_hex(0xE5E7EB)
        };

        for i in 0..4 {
            let (x, y, w, h) = match i {
                0 => (0.0, 0.0, size, 1.0),
                1 => (size - 1.0, 0.0, 1.0, size),
                2 => (0.0, size - 1.0, size, 1.0),
                _ => (0.0, 0.0, 1.0, size),
            };
            render_objects.push(RenderObject::rect(Rect::new(x, y, w, h), border_color));
        }

        // Checkmark
        if self.checked {
            render_objects.push(RenderObject::rect(
                Rect::new(6.0, 9.0, 8.0, 2.0),
                Color::WHITE,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(6.0, 9.0, 2.0, 6.0),
                Color::WHITE,
            ));
        }

        // Label
        if let Some(label) = &self.label {
            render_objects.push(RenderObject::text(
                label.clone(),
                TextStyle {
                    font_family: "Inter".to_string(),
                    font_size: 14.0,
                    color: Color::from_hex(0x111827),
                    bold: false,
                    italic: false,
                },
                Point::new(size + 8.0, size / 2.0 + 5.0),
            ));
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Checkbox {
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
```

## File: `./src/widgets/element_widgets/image.rs`
```rs
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::core::render_object::{Color, Rect, RenderObject};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Image {
    pub path: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub fit: ImageFit,
    pub alt_text: String,
    pub tooltip: Option<String>,
    pub on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageFit {
    Fill,
    Contain,
    Cover,
    ScaleDown,
}

impl Image {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            width: None,
            height: None,
            fit: ImageFit::Contain,
            alt_text: String::new(),
            tooltip: None,
            on_click: None,
            key: None,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn with_alt_text(mut self, alt_text: impl Into<String>) -> Self {
        self.alt_text = alt_text.into();
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
        self.on_click = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Image {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let width = self.width.unwrap_or(ctx.constraints.max_width);
        let height = self.height.unwrap_or(ctx.constraints.max_height);

        // For now, draw a placeholder rectangle
        // In a real implementation, we would load and decode the image
        let placeholder_color = Color::from_hex(0xE5E7EB);
        let border_color = Color::from_hex(0xD1D5DB);

        let mut children = Vec::new();

        // Background
        children.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            placeholder_color,
        ));

        // Border
        children.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, 1.0),
            border_color,
        ));
        children.push(RenderObject::rect(
            Rect::new(width - 1.0, 0.0, 1.0, height),
            border_color,
        ));
        children.push(RenderObject::rect(
            Rect::new(0.0, height - 1.0, width, 1.0),
            border_color,
        ));
        children.push(RenderObject::rect(
            Rect::new(0.0, 0.0, 1.0, height),
            border_color,
        ));

        // "Image" text
        let theme = ctx.theme();
        children.push(RenderObject::text(
            "📷 Image".to_string(),
            crate::core::render_object::TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 14.0,
                color: Color::from_hex(0x6B7280),
                bold: false,
                italic: true,
            },
            crate::core::render_object::Point::new(width / 2.0 - 30.0, height / 2.0 + 5.0),
        ));

        WidgetNode::Leaf(RenderObject::group(children))
    }
}

impl Widget for Image {
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
```

## File: `./src/widgets/element_widgets/table.rs`
```rs
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Table {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub width: Option<f32>,
    pub striped: bool,
    pub hoverable: bool,
    pub bordered: bool,
    pub compact: bool,
    pub sortable: bool,
    pub on_row_click: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub on_sort: Option<Arc<dyn Fn(usize, SortDirection) + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone)]
pub struct TableColumn {
    pub label: String,
    pub width: ColumnWidth,
    pub align: TableAlign,
    pub sortable: bool,
}

#[derive(Clone)]
pub struct TableRow {
    pub cells: Vec<String>,
    pub selectable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColumnWidth {
    Fixed(f32),
    Flex(f32),
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TableAlign {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    None,
}

impl Table {
    pub fn new(columns: Vec<TableColumn>) -> Self {
        Self {
            columns,
            rows: Vec::new(),
            width: None,
            striped: false,
            hoverable: true,
            bordered: true,
            compact: false,
            sortable: false,
            on_row_click: None,
            on_sort: None,
            key: None,
        }
    }

    pub fn with_rows(mut self, rows: Vec<TableRow>) -> Self {
        self.rows = rows;
        self
    }

    pub fn add_row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_on_row_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        self.on_row_click = Some(Arc::new(callback));
        self
    }

    pub fn with_on_sort<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, SortDirection) + Send + Sync + 'static,
    {
        self.on_sort = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }

    fn calculate_column_widths(&self, total_width: f32) -> Vec<f32> {
        let mut widths = Vec::new();
        let mut flex_sum = 0.0;
        let mut fixed_total = 0.0;

        // Calculate fixed and flex totals
        for col in &self.columns {
            match col.width {
                ColumnWidth::Fixed(w) => fixed_total += w,
                ColumnWidth::Flex(flex) => flex_sum += flex,
                ColumnWidth::Auto => flex_sum += 1.0,
            }
        }

        let available_flex = (total_width - fixed_total).max(0.0);

        // Calculate actual widths
        for col in &self.columns {
            let width = match col.width {
                ColumnWidth::Fixed(w) => w,
                ColumnWidth::Flex(flex) => {
                    if flex_sum > 0.0 {
                        (flex / flex_sum) * available_flex
                    } else {
                        available_flex / self.columns.len() as f32
                    }
                }
                ColumnWidth::Auto => {
                    if flex_sum > 0.0 {
                        (1.0 / flex_sum) * available_flex
                    } else {
                        available_flex / self.columns.len() as f32
                    }
                }
            };
            widths.push(width);
        }

        widths
    }
}

impl StatelessWidget for Table {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(ctx.constraints.max_width);
        let row_height = if self.compact { 32.0 } else { 48.0 };
        let header_height = if self.compact { 40.0 } else { 56.0 };

        let column_widths = self.calculate_column_widths(width);
        let mut render_objects = Vec::new();

        // Table background
        let total_height = header_height + (self.rows.len() as f32 * row_height);
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, total_height),
            theme.card,
        ));

        // Table border
        if self.bordered {
            let border_color = theme.border;
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, 0.0, width, 1.0),
                border_color,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(width - 1.0, 0.0, 1.0, total_height),
                border_color,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, total_height - 1.0, width, 1.0),
                border_color,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, 0.0, 1.0, total_height),
                border_color,
            ));
        }

        // Header background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, header_height),
            theme.muted,
        ));

        // Header separator
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, header_height - 1.0, width, 1.0),
            theme.border,
        ));

        // Header cells
        let mut current_x = 8.0;
        for (i, col) in self.columns.iter().enumerate() {
            let col_width = column_widths[i];

            // Column text
            let x_offset = match col.align {
                TableAlign::Left => current_x,
                TableAlign::Center => current_x + (col_width - col.label.len() as f32 * 7.0) / 2.0,
                TableAlign::Right => current_x + col_width - col.label.len() as f32 * 7.0 - 8.0,
            };

            render_objects.push(RenderObject::text(
                col.label.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 14.0,
                    color: theme.foreground,
                    bold: true,
                    italic: false,
                },
                Point::new(x_offset.max(current_x), header_height / 2.0 + 5.0),
            ));

            // Sort indicator if sortable
            if self.sortable && col.sortable {
                render_objects.push(RenderObject::text(
                    "⇅".to_string(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 12.0,
                        color: theme.muted_foreground,
                        bold: false,
                        italic: false,
                    },
                    Point::new(current_x + col_width - 20.0, header_height / 2.0 + 5.0),
                ));
            }

            // Vertical separator
            if self.bordered && i < self.columns.len() - 1 {
                render_objects.push(RenderObject::rect(
                    Rect::new(current_x + col_width, 0.0, 1.0, total_height),
                    theme.border,
                ));
            }

            current_x += col_width;
        }

        // Data rows
        let mut current_y = header_height;
        for (row_idx, row) in self.rows.iter().enumerate() {
            // Striped background
            if self.striped && row_idx % 2 == 1 {
                render_objects.push(RenderObject::rect(
                    Rect::new(0.0, current_y, width, row_height),
                    theme.muted.with_alpha(50),
                ));
            }

            // Row separator
            if self.bordered {
                render_objects.push(RenderObject::rect(
                    Rect::new(0.0, current_y + row_height - 1.0, width, 1.0),
                    theme.border,
                ));
            }

            // Row cells
            current_x = 8.0;
            for (col_idx, cell) in row.cells.iter().enumerate() {
                if col_idx >= self.columns.len() {
                    break;
                }

                let col = &self.columns[col_idx];
                let col_width = column_widths[col_idx];

                let x_offset = match col.align {
                    TableAlign::Left => current_x,
                    TableAlign::Center => current_x + (col_width - cell.len() as f32 * 7.0) / 2.0,
                    TableAlign::Right => current_x + col_width - cell.len() as f32 * 7.0 - 8.0,
                };

                render_objects.push(RenderObject::text(
                    cell.clone(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 13.0,
                        color: theme.foreground,
                        bold: false,
                        italic: false,
                    },
                    Point::new(x_offset.max(current_x), current_y + row_height / 2.0 + 5.0),
                ));

                current_x += col_width;
            }

            current_y += row_height;
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Table {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() => {
                let row_height = if self.compact { 32.0 } else { 48.0 };
                let header_height = if self.compact { 40.0 } else { 56.0 };

                // Check if clicked on header (for sorting)
                if position.y <= header_height && self.sortable {
                    let width = self.width.unwrap_or(800.0);
                    let column_widths = self.calculate_column_widths(width);

                    let mut current_x = 0.0;
                    for (i, col_width) in column_widths.iter().enumerate() {
                        if position.x >= current_x && position.x < current_x + col_width {
                            if self.columns[i].sortable {
                                if let Some(on_sort) = &self.on_sort {
                                    on_sort(i, SortDirection::Ascending);
                                }
                                return EventResult::Stopped;
                            }
                        }
                        current_x += col_width;
                    }
                } else if position.y > header_height {
                    // Check if clicked on row
                    let row_index = ((position.y - header_height) / row_height) as usize;
                    if row_index < self.rows.len() && self.rows[row_index].selectable {
                        if let Some(on_row_click) = &self.on_row_click {
                            on_row_click(row_index);
                            return EventResult::Stopped;
                        }
                    }
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

impl TableColumn {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            width: ColumnWidth::Auto,
            align: TableAlign::Left,
            sortable: false,
        }
    }

    pub fn with_width(mut self, width: ColumnWidth) -> Self {
        self.width = width;
        self
    }

    pub fn align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }
}

impl TableRow {
    pub fn new(cells: Vec<String>) -> Self {
        Self {
            cells,
            selectable: true,
        }
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }
}
```

## File: `./src/widgets/element_widgets/textarea.rs`
```rs
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Textarea {
    pub placeholder: String,
    pub value: String,
    pub rows: u32,
    pub width: Option<f32>,
    pub disabled: bool,
    pub on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Textarea {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            value: String::new(),
            rows: 3,
            width: None,
            disabled: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = rows.max(1);
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
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

impl StatelessWidget for Textarea {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(400.0);
        let height = (self.rows as f32 * 24.0) + 16.0; // Approximate line height

        let bg_color = if self.disabled {
            theme.muted
        } else {
            theme.input
        };

        let border_color = if self.disabled {
            theme.border.with_alpha(128)
        } else {
            theme.border
        };

        let text_color = if self.disabled {
            theme.muted_foreground
        } else {
            theme.foreground
        };

        let mut render_objects = Vec::new();

        // Background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Border (all sides)
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

        // Text content
        let text = if self.value.is_empty() {
            &self.placeholder
        } else {
            &self.value
        };

        let display_color = if self.value.is_empty() && !self.disabled {
            theme.muted_foreground
        } else {
            text_color
        };

        // Split text by lines for display
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().take(self.rows as usize).enumerate() {
            render_objects.push(RenderObject::text(
                line.to_string(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 14.0,
                    color: display_color,
                    bold: false,
                    italic: false,
                },
                Point::new(8.0, 12.0 + (i as f32 * 24.0)),
            ));
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Textarea {
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
```

## File: `./src/widgets/element_widgets/tooltip.rs`
```rs
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct Tooltip {
    pub text: String,
    pub child: Box<dyn Widget>,
    pub placement: TooltipPlacement,
    pub delay: u32,
    pub max_width: Option<f32>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TooltipPlacement {
    Top,
    Bottom,
    Left,
    Right,
}

impl Tooltip {
    pub fn new(text: impl Into<String>, child: Box<dyn Widget>) -> Self {
        Self {
            text: text.into(),
            child,
            placement: TooltipPlacement::Top,
            delay: 500,
            max_width: Some(200.0),
            key: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            child: self.child.clone_box(),
            placement: self.placement,
            delay: self.delay,
            max_width: self.max_width,
            key: self.key.clone(),
        }
    }

    pub fn with_placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_delay(mut self, delay: u32) -> Self {
        self.delay = delay;
        self
    }

    pub fn with_max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Tooltip {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        // For now, just render the child
        // Tooltip display logic would be handled by the framework
        // based on hover state and delay
        self.child.build(ctx)
    }
}

impl Widget for Tooltip {
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

// Tooltip render object builder (used by framework)
pub fn render_tooltip(
    text: &str,
    position: Point,
    placement: TooltipPlacement,
    theme: &crate::core::Theme,
    max_width: f32,
) -> RenderObject {
    let padding = 8.0;
    let font_size = 12.0;

    // Measure text (simplified)
    let text_width = (text.len() as f32 * 7.5).min(max_width - padding * 2.0);
    let text_height = 20.0;

    let tooltip_width = text_width + padding * 2.0;
    let tooltip_height = text_height + padding * 2.0;

    // Calculate position based on placement
    let (x, y) = match placement {
        TooltipPlacement::Top => (position.x - tooltip_width / 2.0, position.y - tooltip_height - 8.0),
        TooltipPlacement::Bottom => (position.x - tooltip_width / 2.0, position.y + 8.0),
        TooltipPlacement::Left => (position.x - tooltip_width - 8.0, position.y - tooltip_height / 2.0),
        TooltipPlacement::Right => (position.x + 8.0, position.y - tooltip_height / 2.0),
    };

    let mut render_objects = Vec::new();

    // Background with shadow
    render_objects.push(RenderObject::rect(
        Rect::new(x, y, tooltip_width, tooltip_height),
        theme.popover,
    ));

    // Border
    render_objects.push(RenderObject::rect(
        Rect::new(x, y, tooltip_width, 1.0),
        theme.border,
    ));
    render_objects.push(RenderObject::rect(
        Rect::new(x + tooltip_width - 1.0, y, 1.0, tooltip_height),
        theme.border,
    ));
    render_objects.push(RenderObject::rect(
        Rect::new(x, y + tooltip_height - 1.0, tooltip_width, 1.0),
        theme.border,
    ));
    render_objects.push(RenderObject::rect(
        Rect::new(x, y, 1.0, tooltip_height),
        theme.border,
    ));

    // Text
    render_objects.push(RenderObject::text(
        text.to_string(),
        TextStyle {
            font_family: theme.font_sans.clone(),
            font_size,
            color: theme.popover_foreground,
            bold: false,
            italic: false,
        },
        Point::new(x + padding, y + padding + 5.0),
    ));

    RenderObject::group(render_objects)
}
```

## File: `./src/widgets/element_widgets/label.rs`
```rs
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Label {
    pub text: String,
    pub bold: bool,
    pub size: Option<f32>,
    pub color: Option<Color>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            bold: false,
            size: None,
            color: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
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

impl StatelessWidget for Label {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let text_color = self.color.unwrap_or_else(|| {
            Color::from_hex(if theme.is_dark { 0xEEEEEE } else { 0x111111 })
        });

        let font_size = self.size.unwrap_or(14.0);

        WidgetNode::Leaf(RenderObject::text(
            self.text.clone(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size,
                color: text_color,
                bold: self.bold,
                italic: false,
            },
            Point::ZERO,
        ))
    }
}

impl Widget for Label {
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
```

## File: `./src/widgets/layout_widgets/mod.rs`
```rs
mod scaffolding;
mod flexbox;
mod grid;
mod resizable;
mod scroll_area;
mod sidebar;

pub use scaffolding::Scaffolding;
pub use flexbox::{Flexbox, FlexDirection, JustifyContent, AlignItems, FlexWrap};
pub use grid::Grid;
pub use resizable::{Resizable, ResizableEdges};
pub use scroll_area::ScrollArea;
pub use sidebar::{Sidebar, SidebarPosition};
```

## File: `./src/widgets/layout_widgets/scaffolding.rs`
```rs
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct Scaffolding {
    pub app_bar: Option<Box<dyn Widget>>,
    pub sidebar: Option<Box<dyn Widget>>,
    pub content: Box<dyn Widget>,
    pub footer: Option<Box<dyn Widget>>,
    pub drawer: Option<Box<dyn Widget>>,
    key: Option<WidgetKey>,
}

impl Scaffolding {
    pub fn new(content: Box<dyn Widget>) -> Self {
        Self {
            app_bar: None,
            sidebar: None,
            content,
            footer: None,
            drawer: None,
            key: None,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            app_bar: self.app_bar.as_ref().map(|w| w.clone_box()),
            sidebar: self.sidebar.as_ref().map(|w| w.clone_box()),
            content: self.content.clone_box(),
            footer: self.footer.as_ref().map(|w| w.clone_box()),
            drawer: self.drawer.as_ref().map(|w| w.clone_box()),
            key: self.key.clone(),
        }
    }

    pub fn with_app_bar(mut self, app_bar: Box<dyn Widget>) -> Self {
        self.app_bar = Some(app_bar);
        self
    }

    pub fn with_sidebar(mut self, sidebar: Box<dyn Widget>) -> Self {
        self.sidebar = Some(sidebar);
        self
    }

    pub fn with_footer(mut self, footer: Box<dyn Widget>) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn with_drawer(mut self, drawer: Box<dyn Widget>) -> Self {
        self.drawer = Some(drawer);
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Scaffolding {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        // This is a layout widget that arranges app bar, sidebar, content, and footer
        // In a real implementation, we would calculate the layout

        let mut children = Vec::new();

        // Add app bar if present
        if let Some(app_bar) = &self.app_bar {
            children.push(app_bar.clone_box());
        }

        // Add sidebar if present
        if let Some(sidebar) = &self.sidebar {
            children.push(sidebar.clone_box());
        }

        // Add content
        children.push(self.content.clone_box());

        // Add footer if present
        if let Some(footer) = &self.footer {
            children.push(footer.clone_box());
        }

        // Add drawer if present (drawn on top)
        if let Some(drawer) = &self.drawer {
            children.push(drawer.clone_box());
        }

        WidgetNode::Container { children }
    }
}

impl Widget for Scaffolding {
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
```

## File: `./src/widgets/layout_widgets/flexbox.rs`
```rs
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct Flexbox {
    pub direction: FlexDirection,
    pub justify: JustifyContent,
    pub align: AlignItems,
    pub wrap: FlexWrap,
    pub gap: f32,
    pub children: Vec<Box<dyn Widget>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Flexbox {
    pub fn new() -> Self {
        Self {
            direction: FlexDirection::Row,
            justify: JustifyContent::FlexStart,
            align: AlignItems::Stretch,
            wrap: FlexWrap::NoWrap,
            gap: 0.0,
            children: Vec::new(),
            key: None,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            direction: self.direction,
            justify: self.justify,
            align: self.align,
            wrap: self.wrap,
            gap: self.gap,
            children: self.children.iter().map(|c| c.clone_box()).collect(),
            key: self.key.clone(),
        }
    }

    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.justify = justify;
        self
    }

    pub fn align(mut self, align: AlignItems) -> Self {
        self.align = align;
        self
    }

    pub fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
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

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Flexbox {
    fn build_stateless(&self, _ctx: &BuildContext) -> WidgetNode {
        // For now, just return the children as a container
        // In a real implementation, we would calculate flexbox layout
        WidgetNode::Container {
            children: self.children.iter().map(|c| c.clone_box()).collect(),
        }
    }
}

impl Widget for Flexbox {
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
```

## File: `./src/widgets/layout_widgets/resizable.rs`
```rs
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Rect, RenderObject};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

pub struct Resizable {
    pub child: Box<dyn Widget>,
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,
    pub width: f32,
    pub height: f32,
    pub resizable: ResizableEdges,
    pub on_resize: Option<Arc<dyn Fn(f32, f32) + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResizableEdges {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl ResizableEdges {
    pub fn all() -> Self {
        Self {
            left: true,
            right: true,
            top: true,
            bottom: true,
        }
    }

    pub fn none() -> Self {
        Self {
            left: false,
            right: false,
            top: false,
            bottom: false,
        }
    }

}

impl Resizable {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            child,
            min_width: 50.0,
            min_height: 50.0,
            max_width: 1000.0,
            max_height: 1000.0,
            width: 200.0,
            height: 150.0,
            resizable: ResizableEdges::all(),
            on_resize: None,
            key: None,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_min_size(mut self, min_width: f32, min_height: f32) -> Self {
        self.min_width = min_width;
        self.min_height = min_height;
        self
    }

    pub fn with_max_size(mut self, max_width: f32, max_height: f32) -> Self {
        self.max_width = max_width;
        self.max_height = max_height;
        self
    }

    pub fn resizable(mut self, edges: ResizableEdges) -> Self {
        self.resizable = edges;
        self
    }

    pub fn with_on_resize<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32, f32) + Send + Sync + 'static,
    {
        self.on_resize = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }

    pub fn clone(&self) -> Self {
        Self {
            child: self.child.clone_box(),
            min_width: self.min_width,
            min_height: self.min_height,
            max_width: self.max_width,
            max_height: self.max_height,
            width: self.width,
            height: self.height,
            resizable: self.resizable,
            on_resize: self.on_resize.clone(),
            key: self.key.clone(),
        }
    }
}

impl StatelessWidget for Resizable {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let handle_size = 8.0;

        let mut render_objects = Vec::new();

        // Child content
        let child_constraints = crate::layout::constraints::Constraints::new(
            0.0,
            self.width,
            0.0,
            self.height,
        );

        let child_ctx = ctx.child_context(ctx.element_id, child_constraints);
        let child_node = self.child.build(&child_ctx);

        if let WidgetNode::Leaf(render_obj) = child_node {
            render_objects.push(render_obj);
        }

        // Resize handles
        let handle_color = theme.primary.with_alpha(150);

        // Bottom-right handle (always visible if resizable)
        if self.resizable.right && self.resizable.bottom {
            let handle_x = self.width - handle_size;
            let handle_y = self.height - handle_size;

            render_objects.push(RenderObject::rect(
                Rect::new(handle_x, handle_y, handle_size, handle_size),
                handle_color,
            ));

            // Diagonal lines in handle
            render_objects.push(RenderObject::rect(
                Rect::new(handle_x + 1.0, handle_y + 3.0, handle_size - 2.0, 1.0),
                theme.primary_foreground,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(handle_x + 3.0, handle_y + 1.0, 1.0, handle_size - 2.0),
                theme.primary_foreground,
            ));
        }

        // Right handle
        if self.resizable.right {
            let handle_x = self.width - handle_size;
            let handle_y = (self.height - handle_size) / 2.0;

            render_objects.push(RenderObject::rect(
                Rect::new(handle_x, handle_y, handle_size, handle_size),
                handle_color,
            ));
        }

        // Bottom handle
        if self.resizable.bottom {
            let handle_x = (self.width - handle_size) / 2.0;
            let handle_y = self.height - handle_size;

            render_objects.push(RenderObject::rect(
                Rect::new(handle_x, handle_y, handle_size, handle_size),
                handle_color,
            ));
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Resizable {
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
```

## File: `./src/widgets/layout_widgets/grid.rs`
```rs
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct Grid {
    pub columns: usize,
    pub rows: usize,
    pub column_gap: f32,
    pub row_gap: f32,
    pub children: Vec<Box<dyn Widget>>,
    key: Option<WidgetKey>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            columns: 1,
            rows: 1,
            column_gap: 0.0,
            row_gap: 0.0,
            children: Vec::new(),
            key: None,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            columns: self.columns,
            rows: self.rows,
            column_gap: self.column_gap,
            row_gap: self.row_gap,
            children: self.children.iter().map(|c| c.clone_box()).collect(),
            key: self.key.clone(),
        }
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(1);
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self.row_gap = gap;
        self
    }

    pub fn column_gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self
    }

    pub fn row_gap(mut self, gap: f32) -> Self {
        self.row_gap = gap;
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

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Grid {
    fn build_stateless(&self, _ctx: &BuildContext) -> WidgetNode {
        // For now, just return the children as a container
        // In a real implementation, we would calculate grid layout
        WidgetNode::Container {
            children: self.children.iter().map(|c| c.clone_box()).collect(),
        }
    }
}

impl Widget for Grid {
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
```

## File: `./src/widgets/basic.rs`
```rs
use crate::core::context::BuildContext;
use crate::core::context::ThemeProvider;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::layout::constraints::{EdgeInsets};
use std::any::Any;
use std::sync::Arc;

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
```

## File: `./src/widgets/layout_widgets/sidebar.rs`
```rs
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
```

## File: `./src/widgets/complex_widgets/mod.rs`
```rs
pub mod dialog;
pub mod aspect_ratio;
pub mod carousel;
pub mod chart;
pub mod combobox;
pub mod date_picker;
pub mod drawer;
pub mod radio_group;
pub mod slider;
pub mod sonner;
pub mod switch;
pub mod tabs;
pub mod card;
pub mod dropdown;
pub mod progress_bar;

pub use slider::Slider;
pub use switch::Switch;
pub use tabs::Tabs;
pub use dialog::Dialog;
pub use radio_group::RadioGroup;
pub use combobox::Combobox;
pub use date_picker::DatePicker;
pub use drawer::Drawer;
pub use aspect_ratio::AspectRatio;
pub use carousel::Carousel;
pub use chart::{Chart, ChartType};
pub use sonner::{Sonner, ToastVariant, ToastPosition};
pub use card::{Card, CardVariant};
pub use dropdown::Dropdown;
pub use progress_bar::{ProgressBar, ProgressVariant};
```

## File: `./src/widgets/element_widgets/video.rs`
```rs
use crate::core::context::{BuildContext, ThemeProvider};
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct Video {
    pub source: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub autoplay: bool,
    pub controls: bool,
    pub loop_playback: bool,
    pub muted: bool,
    pub on_play: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_pause: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_ended: Option<Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

impl Video {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            width: None,
            height: None,
            autoplay: false,
            controls: true,
            loop_playback: false,
            muted: false,
            on_play: None,
            on_pause: None,
            on_ended: None,
            key: None,
        }
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

    pub fn controls(mut self, controls: bool) -> Self {
        self.controls = controls;
        self
    }

    pub fn loop_playback(mut self, loop_playback: bool) -> Self {
        self.loop_playback = loop_playback;
        self
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn with_on_play<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_play = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Video {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(640.0);
        let height = self.height.unwrap_or(360.0);

        let mut render_objects = Vec::new();

        // Video placeholder (actual video rendering would need platform integration)
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            Color::from_hex(0x000000),
        ));

        // Play icon overlay
        render_objects.push(RenderObject::text(
            "▶".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 48.0,
                color: Color::WHITE,
                bold: false,
                italic: false,
            },
            Point::new(width / 2.0 - 24.0, height / 2.0 + 16.0),
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Video {
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
```

## File: `./src/widgets/layout_widgets/scroll_area.rs`
```rs
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct ScrollArea {
    pub child: Box<dyn Widget>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub scroll_x: bool,
    pub scroll_y: bool,
    pub scrollbar_size: f32,
    key: Option<WidgetKey>,
}

impl ScrollArea {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            child,
            width: None,
            height: None,
            scroll_x: false,
            scroll_y: true,
            scrollbar_size: 8.0,
            key: None,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            child: self.child.clone_box(),
            width: self.width,
            height: self.height,
            scroll_x: self.scroll_x,
            scroll_y: self.scroll_y,
            scrollbar_size: self.scrollbar_size,
            key: self.key.clone(),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn scroll_x(mut self, scroll_x: bool) -> Self {
        self.scroll_x = scroll_x;
        self
    }

    pub fn scroll_y(mut self, scroll_y: bool) -> Self {
        self.scroll_y = scroll_y;
        self
    }

    pub fn scrollbar_size(mut self, size: f32) -> Self {
        self.scrollbar_size = size;
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for ScrollArea {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let width = self.width.unwrap_or(ctx.constraints.max_width);
        let height = self.height.unwrap_or(ctx.constraints.max_height);

        // Create a clipping area for the child
        // In a real implementation, we would handle scrolling and scrollbars
        let child_constraints = crate::layout::constraints::Constraints::new(
            0.0,
            width,
            0.0,
            height,
        );

        let child_ctx = ctx.child_context(ctx.element_id, child_constraints);
        self.child.build(&child_ctx)
    }
}

impl Widget for ScrollArea {
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
```

## File: `./src/widgets/complex_widgets/aspect_ratio.rs`
```rs
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
```

## File: `./src/widgets/complex_widgets/combobox.rs`
```rs
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Combobox {
    pub options: Vec<String>,
    pub selected: Option<usize>,
    pub placeholder: String,
    pub searchable: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub disabled: bool,
    pub open: bool,
    pub on_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub on_search: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Combobox {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected: None,
            placeholder: "Select an option...".to_string(),
            searchable: false,
            width: None,
            height: None,
            disabled: false,
            open: false,
            on_change: None,
            on_search: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        self.on_change = Some(Arc::new(callback));
        self
    }

    pub fn with_on_search<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.on_search = Some(Arc::new(callback));
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

impl StatelessWidget for Combobox {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(40.0);
        let item_height = 32.0;

        let bg_color = if self.disabled {
            theme.muted
        } else {
            theme.input
        };

        let border_color = if self.disabled {
            theme.border.with_alpha(128)
        } else {
            theme.border
        };

        let text_color = if self.disabled {
            theme.muted_foreground
        } else {
            theme.foreground
        };

        let mut render_objects = Vec::new();

        // Main combobox box
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Border
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

        // Selected value or placeholder
        let display_text = if let Some(selected) = self.selected {
            &self.options[selected]
        } else {
            &self.placeholder
        };

        let display_color = if self.selected.is_none() && !self.disabled {
            theme.muted_foreground
        } else {
            text_color
        };

        render_objects.push(RenderObject::text(
            display_text.to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 14.0,
                color: display_color,
                bold: false,
                italic: false,
            },
            Point::new(12.0, height / 2.0 + 5.0),
        ));

        // Combobox arrow
        render_objects.push(RenderObject::text(
            "▼".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 12.0,
                color: theme.muted_foreground,
                bold: false,
                italic: false,
            },
            Point::new(width - 24.0, height / 2.0 + 5.0),
        ));

        // Dropdown menu (if open)
        if self.open && !self.disabled {
            let menu_height = ((self.options.len() as f32 + 0.5) * item_height).min(250.0);

            // Menu background
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height, width, menu_height),
                theme.popover,
            ));

            // Menu border
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height, width, 1.0),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(width - 1.0, height, 1.0, menu_height),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height + menu_height - 1.0, width, 1.0),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height, 1.0, menu_height),
                theme.border,
            ));

            // Search input (if searchable)
            let mut current_y = height;
            if self.searchable {
                let search_height = item_height;

                // Search background
                render_objects.push(RenderObject::rect(
                    Rect::new(0.0, current_y, width, search_height),
                    theme.background,
                ));

                // Search placeholder
                render_objects.push(RenderObject::text(
                    "Search...".to_string(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 14.0,
                        color: theme.muted_foreground,
                        bold: false,
                        italic: false,
                    },
                    Point::new(12.0, current_y + search_height / 2.0 + 5.0),
                ));

                current_y += search_height;
            }

            // Menu items
            for (i, option) in self.options.iter().enumerate() {
                let item_y = current_y + (i as f32 * item_height);
                let is_selected = self.selected == Some(i);

                // Item background (hover/selected effect)
                if is_selected {
                    render_objects.push(RenderObject::rect(
                        Rect::new(0.0, item_y, width, item_height),
                        theme.accent,
                    ));
                }

                // Item text
                render_objects.push(RenderObject::text(
                    option.clone(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 14.0,
                        color: if is_selected { theme.accent_foreground } else { theme.popover_foreground },
                        bold: false,
                        italic: false,
                    },
                    Point::new(12.0, item_y + item_height / 2.0 + 5.0),
                ));
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Combobox {
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
```

## File: `./src/widgets/complex_widgets/carousel.rs`
```rs
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
```

## File: `./src/widgets/complex_widgets/calender.rs`
```rs
use crate::core::context::{BuildContext, ThemeProvider};
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct Calendar {
    pub selected_date: Option<String>,
    pub month: u32,
    pub year: i32,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub show_header: bool,
    pub show_navigation: bool,
    pub on_date_select: Option<Arc<dyn Fn(String) + Send + Sync>>,
    key: Option<WidgetKey>,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            selected_date: None,
            month: 1,
            year: 2025,
            width: None,
            height: None,
            show_header: true,
            show_navigation: true,
            on_date_select: None,
            key: None,
        }
    }

    pub fn with_date(mut self, date: impl Into<String>) -> Self {
        self.selected_date = Some(date.into());
        self
    }

    pub fn with_month_year(mut self, month: u32, year: i32) -> Self {
        self.month = month;
        self.year = year;
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

impl StatelessWidget for Calendar {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(300.0);
        let height = self.height.unwrap_or(300.0);

        let mut render_objects = Vec::new();

        // Calendar background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            theme.card,
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Calendar {
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
```

## File: `./src/widgets/complex_layout_widgets/mod.rs`
```rs
mod backdrop;

pub use backdrop::Backdrop;
```

## File: `./src/widgets/complex_widgets/dialog.rs`
```rs
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use std::any::Any;
use std::sync::Arc;

pub struct Dialog {
    pub title: String,
    pub description: Option<String>,
    pub children: Vec<Box<dyn Widget>>,
    pub open: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

impl Dialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            children: Vec::new(),
            open: false,
            width: None,
            height: None,
            on_close: None,
            key: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            description: self.description.clone(),
            children: self
                .children
                .iter()
                .map(|child| child.clone_box())
                .collect(),
            open: self.open,
            width: self.width,
            height: self.height,
            on_close: self.on_close.clone(),
            key: self.key.clone(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
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

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_close = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Dialog {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        if !self.open {
            return WidgetNode::None;
        }

        let theme = &ctx.theme;
        let width = self.width.unwrap_or(400.0);
        let height = self.height.unwrap_or(300.0);

        // Position in center of screen (simplified)
        let screen_width = ctx.constraints.max_width;
        let screen_height = ctx.constraints.max_height;

        let x = (screen_width - width) / 2.0;
        let y = (screen_height - height) / 2.0;

        let mut render_objects = Vec::new();

        // Backdrop (semi-transparent overlay)
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, screen_width, screen_height),
            Color::rgba(0, 0, 0, 100),
        ));

        // Dialog container
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, width, height),
            theme.popover,
        ));

        // Dialog border (all sides)
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x + width - 1.0, y, 1.0, height),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x, y + height - 1.0, width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, 1.0, height),
            theme.border,
        ));

        // Title
        render_objects.push(RenderObject::text(
            self.title.clone(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 18.0,
                color: theme.popover_foreground,
                bold: true,
                italic: false,
            },
            Point::new(x + 16.0, y + 20.0),
        ));

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
                Point::new(x + 16.0, y + 50.0),
            ));
        }

        // Close button
        let close_button_size = 24.0;
        render_objects.push(RenderObject::rect(
            Rect::new(
                x + width - close_button_size - 8.0,
                y + 8.0,
                close_button_size,
                close_button_size,
            ),
            theme.destructive,
        ));

        render_objects.push(RenderObject::text(
            "×".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 18.0,
                color: theme.destructive_foreground,
                bold: true,
                italic: false,
            },
            Point::new(x + width - close_button_size - 4.0, y + 10.0),
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Dialog {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        if !self.open {
            return EventResult::Unhandled;
        }

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() => {
                // Calculate close button rect using screen dimensions
                // Note: We don't have direct access to ctx here, so we use hardcoded values or pass them through state
                let width = self.width.unwrap_or(400.0);
                let height = self.height.unwrap_or(300.0);
                let close_button_size = 24.0;

                // These would need to be calculated from screen dimensions
                // For now, just check if clicked
                if let Some(on_close) = &self.on_close {
                    on_close();
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
```

## File: `./src/widgets/complex_widgets/drawer.rs`
```rs
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

pub struct Drawer {
    pub title: Option<String>,
    pub position: DrawerPosition,
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub children: Vec<Box<dyn Widget>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerPosition {
    Left,
    Right,
    Top,
    Bottom,
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            title: None,
            position: DrawerPosition::Right,
            width: 300.0,
            height: 400.0,
            open: false,
            children: Vec::new(),
            on_close: None,
            key: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            position: self.position,
            width: self.width,
            height: self.height,
            open: self.open,
            children: self
                .children
                .iter()
                .map(|child| child.clone_box())
                .collect(),
            on_close: self.on_close.clone(),
            key: self.key.clone(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_position(mut self, position: DrawerPosition) -> Self {
        self.position = position;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
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

    pub fn with_on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_close = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Drawer {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        if !self.open {
            return WidgetNode::None;
        }

        let theme = ctx.theme();
        let screen_width = ctx.constraints.max_width;
        let screen_height = ctx.constraints.max_height;

        let mut render_objects = Vec::new();

        // Backdrop (semi-transparent overlay)
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, screen_width, screen_height),
            Color::rgba(0, 0, 0, 100),
        ));

        // Calculate drawer position
        let (x, y, width, height) = match self.position {
            DrawerPosition::Left => (
                0.0,
                0.0,
                self.width,
                screen_height,
            ),
            DrawerPosition::Right => (
                screen_width - self.width,
                0.0,
                self.width,
                screen_height,
            ),
            DrawerPosition::Top => (
                0.0,
                0.0,
                screen_width,
                self.height,
            ),
            DrawerPosition::Bottom => (
                0.0,
                screen_height - self.height,
                screen_width,
                self.height,
            ),
        };

        // Drawer container
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, width, height),
            theme.popover,
        ));

        // Drawer border
        match self.position {
            DrawerPosition::Left => {
                render_objects.push(RenderObject::rect(
                    Rect::new(x + width - 1.0, y, 1.0, height),
                    theme.border,
                ));
            }
            DrawerPosition::Right => {
                render_objects.push(RenderObject::rect(
                    Rect::new(x, y, 1.0, height),
                    theme.border,
                ));
            }
            DrawerPosition::Top => {
                render_objects.push(RenderObject::rect(
                    Rect::new(x, y + height - 1.0, width, 1.0),
                    theme.border,
                ));
            }
            DrawerPosition::Bottom => {
                render_objects.push(RenderObject::rect(
                    Rect::new(x, y, width, 1.0),
                    theme.border,
                ));
            }
        }

        // Title
        if let Some(title) = &self.title {
            let title_y = y + 20.0;
            render_objects.push(RenderObject::text(
                title.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 18.0,
                    color: theme.popover_foreground,
                    bold: true,
                    italic: false,
                },
                Point::new(x + 20.0, title_y),
            ));
        }

        // Close button
        let close_button_size = 24.0;
        let close_x = match self.position {
            DrawerPosition::Left | DrawerPosition::Top => x + width - close_button_size - 8.0,
            DrawerPosition::Right | DrawerPosition::Bottom => x + 8.0,
        };
        let close_y = y + 8.0;

        render_objects.push(RenderObject::rect(
            Rect::new(close_x, close_y, close_button_size, close_button_size),
            theme.destructive,
        ));

        render_objects.push(RenderObject::text(
            "×".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 18.0,
                color: theme.destructive_foreground,
                bold: true,
                italic: false,
            },
            Point::new(close_x + 4.0, close_y + 4.0),
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Drawer {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        if !self.open {
            return EventResult::Unhandled;
        }

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() => {
                // FIX: EventContext doesn't have constraints
                // We need to use stored dimensions or pass them through widget state
                let close_button_size = 24.0;

                // Check if close button clicked (simplified - needs proper calculation)
                if let Some(on_close) = &self.on_close {
                    on_close();
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
```

## File: `./src/widgets/complex_widgets/radio_group.rs`
```rs
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
```

## File: `./src/widgets/complex_layout_widgets/backdrop.rs`
```rs
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Rect, RenderObject};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct Backdrop {
    pub child: Box<dyn Widget>,
    pub blur: f32,
    pub brightness: f32,
    pub opacity: f32,
    pub visible: bool,
    key: Option<WidgetKey>,
}

impl Backdrop {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            child,
            blur: 4.0,
            brightness: 0.8,
            opacity: 0.5,
            visible: true,
            key: None,
        }
    }

    pub fn clone(&self) -> Self{
        Self {
            child: self.child.clone_box(),
            blur: self.blur,
            brightness: self.brightness,
            opacity: self.opacity,
            visible: self.visible,
            key: self.key.clone(),
        }
    }

    pub fn blur(mut self, blur: f32) -> Self {
        self.blur = blur;
        self
    }

    pub fn brightness(mut self, brightness: f32) -> Self {
        self.brightness = brightness;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Backdrop {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        if !self.visible {
            return self.child.build(ctx);
        }

        let width = ctx.constraints.max_width;
        let height = ctx.constraints.max_height;

        // In a real implementation, we would apply blur and brightness filters
        // For now, we'll just draw a semi-transparent overlay

        let mut render_objects = Vec::new();

        // Background overlay
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            Color::rgba(0, 0, 0, (self.opacity * 200.0) as u8),
        ));

        // Child content (should be drawn on top)
        let child_node = self.child.build(ctx);
        if let WidgetNode::Leaf(render_obj) = child_node {
            render_objects.push(render_obj);
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Backdrop {
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
```

## File: `./src/widgets/complex_widgets/slider.rs`
```rs
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{ Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Slider {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub step: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub disabled: bool,
    pub on_change: Option<Arc<dyn Fn(f32) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Slider {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            value: min,
            step: None,
            width: None,
            height: None,
            disabled: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn with_step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32) + Send + Sync + 'static,
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

impl StatelessWidget for Slider {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(32.0);

        let track_height = 6.0;
        let thumb_size = 20.0;

        let normalized_value = (self.value - self.min) / (self.max - self.min);
        let thumb_position = normalized_value * (width - thumb_size);

        let track_color = if self.disabled {
            theme.muted
        } else {
            theme.border
        };

        let fill_color = if self.disabled {
            theme.muted.with_alpha(128)
        } else {
            theme.primary
        };

        let thumb_color = if self.disabled {
            theme.muted_foreground
        } else {
            theme.primary
        };

        let mut render_objects = Vec::new();

        // Track background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, (height - track_height) / 2.0, width, track_height),
            track_color,
        ));

        // Filled portion
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, (height - track_height) / 2.0, thumb_position + thumb_size / 2.0, track_height),
            fill_color,
        ));

        // Thumb
        render_objects.push(RenderObject::rect(
            Rect::new(thumb_position, (height - thumb_size) / 2.0, thumb_size, thumb_size),
            thumb_color,
        ));

        // Value label
        if !self.disabled {
            let value_text = format!("{:.1}", self.value);
            render_objects.push(RenderObject::text(
                value_text,
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 12.0,
                    color: theme.foreground,
                    bold: false,
                    italic: false,
                },
                Point::new(thumb_position + thumb_size / 2.0 - 10.0, (height - thumb_size) / 2.0 - 15.0),
            ));
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Slider {
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
```

## File: `./src/widgets/complex_widgets/switch.rs`
```rs
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
```

## File: `./src/widgets/complex_widgets/tabs.rs`
```rs
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
```

## File: `./src/widgets/complex_widgets/date_picker.rs`
```rs
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct DatePicker {
    pub value: Option<String>,
    pub placeholder: String,
    pub format: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub disabled: bool,
    pub open: bool,
    pub on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl DatePicker {
    pub fn new() -> Self {
        Self {
            value: None,
            placeholder: "Select date...".to_string(),
            format: "%Y-%m-%d".to_string(),
            width: None,
            height: None,
            disabled: false,
            open: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = format.into();
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
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

impl StatelessWidget for DatePicker {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(40.0);

        let bg_color = if self.disabled {
            theme.muted
        } else {
            theme.input
        };

        let border_color = if self.disabled {
            theme.border.with_alpha(128)
        } else {
            theme.border
        };

        let text_color = if self.disabled {
            theme.muted_foreground
        } else {
            theme.foreground
        };

        let mut render_objects = Vec::new();

        // Date picker box
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Border
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

        // Display value or placeholder
        let display_text = if let Some(value) = &self.value {
            value.clone()
        } else {
            self.placeholder.clone()
        };

        let display_color = if self.value.is_none() && !self.disabled {
            theme.muted_foreground
        } else {
            text_color
        };

        render_objects.push(RenderObject::text(
            display_text,
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 14.0,
                color: display_color,
                bold: false,
                italic: false,
            },
            Point::new(12.0, height / 2.0 + 5.0),
        ));

        // Calendar icon
        render_objects.push(RenderObject::text(
            "📅".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 16.0,
                color: theme.muted_foreground,
                bold: false,
                italic: false,
            },
            Point::new(width - 30.0, height / 2.0 + 5.0),
        ));

        // Calendar popup (if open)
        if self.open && !self.disabled {
            let calendar_width = 280.0;
            let calendar_height = 320.0;
            let calendar_x = 0.0;
            let calendar_y = height + 4.0;

            // Calendar background
            render_objects.push(RenderObject::rect(
                Rect::new(calendar_x, calendar_y, calendar_width, calendar_height),
                theme.popover,
            ));

            // Calendar border
            render_objects.push(RenderObject::rect(
                Rect::new(calendar_x, calendar_y, calendar_width, 1.0),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(calendar_x + calendar_width - 1.0, calendar_y, 1.0, calendar_height),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(calendar_x, calendar_y + calendar_height - 1.0, calendar_width, 1.0),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(calendar_x, calendar_y, 1.0, calendar_height),
                theme.border,
            ));

            // Calendar header (month/year)
            render_objects.push(RenderObject::text(
                "March 2024".to_string(), // Hardcoded for example
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 16.0,
                    color: theme.popover_foreground,
                    bold: true,
                    italic: false,
                },
                Point::new(calendar_x + 20.0, calendar_y + 30.0),
            ));

            // Day headers (Sun, Mon, Tue, etc.)
            let day_headers = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
            let cell_size = 36.0;
            let header_start_y = calendar_y + 60.0;

            for (i, day) in day_headers.iter().enumerate() {
                let x = calendar_x + 10.0 + (i as f32 * cell_size);
                render_objects.push(RenderObject::text(
                    day.to_string(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 12.0,
                        color: theme.muted_foreground,
                        bold: true,
                        italic: false,
                    },
                    Point::new(x, header_start_y),
                ));
            }

            // Calendar days (example grid)
            let days_start_y = header_start_y + 25.0;
            for week in 0..6 {
                for day in 0..7 {
                    let day_number = (week * 7 + day + 1).min(31);
                    let x = calendar_x + 10.0 + (day as f32 * cell_size);
                    let y = days_start_y + (week as f32 * cell_size);

                    let is_today = day_number == 15; // Example: today is 15th
                    let day_color = if is_today {
                        theme.primary
                    } else {
                        theme.popover_foreground
                    };

                    render_objects.push(RenderObject::text(
                        day_number.to_string(),
                        TextStyle {
                            font_family: theme.font_sans.clone(),
                            font_size: 14.0,
                            color: day_color,
                            bold: is_today,
                            italic: false,
                        },
                        Point::new(x + 10.0, y + 10.0),
                    ));
                }
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for DatePicker {
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
```

## File: `./src/widgets/complex_widgets/dropdown.rs`
```rs
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Dropdown {
    pub options: Vec<String>,
    pub selected: Option<usize>,
    pub placeholder: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub disabled: bool,
    pub open: bool,
    pub on_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Dropdown {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected: None,
            placeholder: "Select an option".to_string(),
            width: None,
            height: None,
            disabled: false,
            open: false,
            on_change: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
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

impl StatelessWidget for Dropdown {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(40.0);
        let item_height = 32.0;

        let bg_color = if self.disabled {
            theme.muted
        } else {
            theme.input
        };

        let border_color = if self.disabled {
            theme.border.with_alpha(128)
        } else {
            theme.border
        };

        let text_color = if self.disabled {
            theme.muted_foreground
        } else {
            theme.foreground
        };

        let mut render_objects = Vec::new();

        // Main dropdown box
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Border
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

        // Selected value or placeholder
        let display_text = if let Some(selected) = self.selected {
            &self.options[selected]
        } else {
            &self.placeholder
        };

        let display_color = if self.selected.is_none() && !self.disabled {
            theme.muted_foreground
        } else {
            text_color
        };

        render_objects.push(RenderObject::text(
            display_text.to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 14.0,
                color: display_color,
                bold: false,
                italic: false,
            },
            Point::new(12.0, height / 2.0 + 5.0),
        ));

        // Dropdown arrow
        render_objects.push(RenderObject::text(
            "▼".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 12.0,
                color: theme.muted_foreground,
                bold: false,
                italic: false,
            },
            Point::new(width - 24.0, height / 2.0 + 5.0),
        ));

        // Dropdown menu (if open)
        if self.open && !self.disabled {
            let menu_height = (self.options.len() as f32 * item_height).min(200.0);

            // Menu background
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height, width, menu_height),
                theme.popover,
            ));

            // Menu border
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height, width, 1.0),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(width - 1.0, height, 1.0, menu_height),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height + menu_height - 1.0, width, 1.0),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height, 1.0, menu_height),
                theme.border,
            ));

            // Menu items
            for (i, option) in self.options.iter().enumerate() {
                let item_y = height + (i as f32 * item_height);
                let is_selected = self.selected == Some(i);

                // Item background (hover/selected effect)
                if is_selected {
                    render_objects.push(RenderObject::rect(
                        Rect::new(0.0, item_y, width, item_height),
                        theme.accent,
                    ));
                }

                // Item text
                render_objects.push(RenderObject::text(
                    option.clone(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 14.0,
                        color: if is_selected { theme.accent_foreground } else { theme.popover_foreground },
                        bold: false,
                        italic: false,
                    },
                    Point::new(12.0, item_y + item_height / 2.0 + 5.0),
                ));
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Dropdown {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        if self.disabled {
            return EventResult::Unhandled;
        }

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() => {
                let width = self.width.unwrap_or(200.0);
                let height = self.height.unwrap_or(40.0);

                // Check if clicked on main dropdown
                let main_rect = Rect::new(0.0, 0.0, width, height);
                if main_rect.contains(position.x, position.y) {
                    // Toggle open state (this would need state management)
                    println!("Dropdown clicked - would toggle open state");
                    return EventResult::Stopped;
                }

                // Check if clicked on menu item
                if self.open {
                    let item_height = 32.0;
                    for (i, _) in self.options.iter().enumerate() {
                        let item_y = height + (i as f32 * item_height);
                        let item_rect = Rect::new(0.0, item_y, width, item_height);

                        if item_rect.contains(position.x, position.y) {
                            if let Some(on_change) = &self.on_change {
                                on_change(i);
                            }
                            return EventResult::Stopped;
                        }
                    }
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
```

## File: `./src/widgets/complex_widgets/card.rs`
```rs
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
```

## File: `./src/widgets/complex_widgets/progress_bar.rs`
```rs
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Rect, RenderObject};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct ProgressBar {
    pub value: f32,
    pub max: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub variant: ProgressVariant,
    pub show_value: bool,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProgressVariant {
    Default,
    Striped,
    Animated,
    Circular,
}

impl ProgressBar {
    pub fn new(value: f32, max: f32) -> Self {
        Self {
            value: value.clamp(0.0, max),
            max,
            width: None,
            height: None,
            variant: ProgressVariant::Default,
            show_value: false,
            tooltip: None,
            key: None,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
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

impl StatelessWidget for ProgressBar {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(8.0);

        let progress = (self.value / self.max).clamp(0.0, 1.0);
        let progress_width = width * progress;

        let bg_color = theme.muted;
        let progress_color = if progress < 0.3 {
            theme.destructive
        } else if progress < 0.7 {
            theme.secondary
        } else {
            theme.primary
        };

        let mut render_objects = Vec::new();

        // Background track
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Progress fill
        if self.variant == ProgressVariant::Striped {
            // Striped pattern (simplified)
            let stripe_width = 10.0;
            let mut stripe_x = 0.0;
            while stripe_x < progress_width {
                let stripe_end = (stripe_x + stripe_width).min(progress_width);
                let stripe_color = if (stripe_x / stripe_width) as i32 % 2 == 0 {
                    progress_color
                } else {
                    progress_color.with_alpha(180)
                };

                render_objects.push(RenderObject::rect(
                    Rect::new(stripe_x, 0.0, stripe_end - stripe_x, height),
                    stripe_color,
                ));

                stripe_x += stripe_width;
            }
        } else {
            // Solid fill
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, 0.0, progress_width, height),
                progress_color,
            ));
        }

        // Value text
        if self.show_value {
            let value_text = format!("{:.0}%", progress * 100.0);
            render_objects.push(RenderObject::text(
                value_text,
                crate::core::render_object::TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 12.0,
                    color: theme.foreground,
                    bold: false,
                    italic: false,
                },
                crate::core::render_object::Point::new(width + 8.0, height / 2.0 + 5.0),
            ));
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for ProgressBar {
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
```

## File: `./src/widgets/scrolling.rs`
```rs
// File: ./oxideui/src/widgets/scrolling.rs
//! Advanced scrolling and clipping with momentum and snap points

use std::time::{Duration, Instant};
use crate::core::render_object::{Point, Rect};
use crate::core::event::Vector2;

/// Scroll physics for natural scrolling behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollPhysics {
    /// Bouncy iOS-style scrolling
    Bouncing,
    /// Clamped Android-style scrolling
    Clamping,
    /// No physics, direct positioning
    Never,
}

/// Scroll controller for programmatic scrolling
pub struct ScrollController {
    pub offset: Vector2,
    pub max_offset: Vector2,
    pub physics: ScrollPhysics,
    velocity: Vector2,
    last_update: Instant,
    is_scrolling: bool,
    momentum_enabled: bool,
}

impl ScrollController {
    pub fn new() -> Self {
        Self {
            offset: Vector2::ZERO,
            max_offset: Vector2::ZERO,
            physics: ScrollPhysics::Bouncing,
            velocity: Vector2::ZERO,
            last_update: Instant::now(),
            is_scrolling: false,
            momentum_enabled: true,
        }
    }

    /// Update scroll position with delta
    pub fn scroll(&mut self, delta: Vector2) {
        let new_offset = Vector2::new(
            self.offset.x + delta.x,
            self.offset.y + delta.y,
        );

        self.offset = self.apply_physics(new_offset);
        self.is_scrolling = true;

        // Update velocity for momentum
        let dt = self.last_update.elapsed().as_secs_f32();
        if dt > 0.0 && self.momentum_enabled {
            self.velocity = Vector2::new(delta.x / dt, delta.y / dt);
        }

        self.last_update = Instant::now();
    }

    /// Apply momentum scrolling
    pub fn update_momentum(&mut self, dt: f32) {
        if !self.momentum_enabled || self.velocity.x.abs() < 0.1 && self.velocity.y.abs() < 0.1 {
            self.velocity = Vector2::ZERO;
            self.is_scrolling = false;
            return;
        }

        // Apply friction
        let friction = 0.95;
        self.velocity.x *= friction;
        self.velocity.y *= friction;

        // Apply velocity
        let delta = Vector2::new(
            self.velocity.x * dt,
            self.velocity.y * dt,
        );

        let new_offset = Vector2::new(
            self.offset.x + delta.x,
            self.offset.y + delta.y,
        );

        self.offset = self.apply_physics(new_offset);
    }

    fn apply_physics(&self, offset: Vector2) -> Vector2 {
        match self.physics {
            ScrollPhysics::Clamping => {
                Vector2::new(
                    offset.x.clamp(0.0, self.max_offset.x),
                    offset.y.clamp(0.0, self.max_offset.y),
                )
            }
            ScrollPhysics::Bouncing => {
                // Allow overscroll with resistance
                let overscroll_resistance = 0.3;

                let x = if offset.x < 0.0 {
                    offset.x * overscroll_resistance
                } else if offset.x > self.max_offset.x {
                    self.max_offset.x + (offset.x - self.max_offset.x) * overscroll_resistance
                } else {
                    offset.x
                };

                let y = if offset.y < 0.0 {
                    offset.y * overscroll_resistance
                } else if offset.y > self.max_offset.y {
                    self.max_offset.y + (offset.y - self.max_offset.y) * overscroll_resistance
                } else {
                    offset.y
                };

                Vector2::new(x, y)
            }
            ScrollPhysics::Never => offset,
        }
    }

    /// Animate to specific position
    pub fn animate_to(&mut self, target: Vector2, duration: Duration) {
        // Would use animation system
        self.offset = target;
    }

    /// Jump to position immediately
    pub fn jump_to(&mut self, position: Vector2) {
        self.offset = self.apply_physics(position);
        self.velocity = Vector2::ZERO;
    }

    /// Set content size to calculate max offset
    pub fn set_content_size(&mut self, content_size: Vector2, viewport_size: Vector2) {
        self.max_offset = Vector2::new(
            (content_size.x - viewport_size.x).max(0.0),
            (content_size.y - viewport_size.y).max(0.0),
        );
    }

    pub fn is_scrolling(&self) -> bool {
        self.is_scrolling
    }

    pub fn stop(&mut self) {
        self.velocity = Vector2::ZERO;
        self.is_scrolling = false;
    }
}

impl Default for ScrollController {
    fn default() -> Self {
        Self::new()
    }
}

/// Snap point for scroll snapping
#[derive(Debug, Clone, Copy)]
pub struct SnapPoint {
    pub offset: f32,
    pub strength: f32, // 0.0 to 1.0
}

/// Scroll snap controller
pub struct ScrollSnapController {
    pub snap_points: Vec<SnapPoint>,
    pub snap_threshold: f32,
    pub axis: SnapAxis,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnapAxis {
    Horizontal,
    Vertical,
    Both,
}

impl ScrollSnapController {
    pub fn new(axis: SnapAxis) -> Self {
        Self {
            snap_points: Vec::new(),
            snap_threshold: 50.0,
            axis,
        }
    }

    /// Find nearest snap point
    pub fn find_snap_point(&self, current_offset: Vector2) -> Option<Vector2> {
        if self.snap_points.is_empty() {
            return None;
        }

        let offset = match self.axis {
            SnapAxis::Horizontal => current_offset.x,
            SnapAxis::Vertical => current_offset.y,
            SnapAxis::Both => current_offset.x, // Simplified
        };

        let mut nearest: Option<&SnapPoint> = None;
        let mut min_distance = f32::INFINITY;

        for snap in &self.snap_points {
            let distance = (snap.offset - offset).abs();
            if distance < min_distance && distance < self.snap_threshold {
                min_distance = distance;
                nearest = Some(snap);
            }
        }

        nearest.map(|snap| match self.axis {
            SnapAxis::Horizontal => Vector2::new(snap.offset, current_offset.y),
            SnapAxis::Vertical => Vector2::new(current_offset.x, snap.offset),
            SnapAxis::Both => Vector2::new(snap.offset, snap.offset),
        })
    }

    pub fn add_snap_point(&mut self, point: SnapPoint) {
        self.snap_points.push(point);
        self.snap_points.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());
    }
}

/// Clipping rectangle manager
pub struct ClipManager {
    clip_stack: Vec<Rect>,
}

impl ClipManager {
    pub fn new() -> Self {
        Self {
            clip_stack: Vec::new(),
        }
    }

    /// Push a clip rect
    pub fn push_clip(&mut self, rect: Rect) {
        if let Some(current) = self.clip_stack.last() {
            // Intersect with current clip
            let intersected = self.intersect_rects(*current, rect);
            self.clip_stack.push(intersected);
        } else {
            self.clip_stack.push(rect);
        }
    }

    /// Pop the current clip
    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    /// Get current clip rect
    pub fn current_clip(&self) -> Option<Rect> {
        self.clip_stack.last().copied()
    }

    /// Check if point is clipped
    pub fn is_clipped(&self, point: Point) -> bool {
        if let Some(clip) = self.current_clip() {
            !clip.contains(point.x, point.y)
        } else {
            false
        }
    }

    /// Check if rect is clipped
    pub fn is_rect_clipped(&self, rect: Rect) -> bool {
        if let Some(clip) = self.current_clip() {
            // Check if completely outside
            rect.x + rect.width < clip.x ||
                rect.x > clip.x + clip.width ||
                rect.y + rect.height < clip.y ||
                rect.y > clip.y + clip.height
        } else {
            false
        }
    }

    fn intersect_rects(&self, a: Rect, b: Rect) -> Rect {
        let x = a.x.max(b.x);
        let y = a.y.max(b.y);
        let width = (a.x + a.width).min(b.x + b.width) - x;
        let height = (a.y + a.height).min(b.y + b.height) - y;

        Rect::new(x, y, width.max(0.0), height.max(0.0))
    }

    pub fn clear(&mut self) {
        self.clip_stack.clear();
    }
}

impl Default for ClipManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Virtual scrolling for large lists
pub struct VirtualScroller {
    pub item_height: f32,
    pub viewport_height: f32,
    pub total_items: usize,
    pub buffer_size: usize,
}

impl VirtualScroller {
    pub fn new(item_height: f32, viewport_height: f32) -> Self {
        Self {
            item_height,
            viewport_height,
            total_items: 0,
            buffer_size: 3,
        }
    }

    /// Calculate which items are visible
    pub fn visible_range(&self, scroll_offset: f32) -> (usize, usize) {
        let start_index = (scroll_offset / self.item_height).floor() as usize;
        let visible_count = (self.viewport_height / self.item_height).ceil() as usize;

        let start = start_index.saturating_sub(self.buffer_size);
        let end = (start_index + visible_count + self.buffer_size).min(self.total_items);

        (start, end)
    }

    /// Get total content height
    pub fn content_height(&self) -> f32 {
        self.item_height * self.total_items as f32
    }

    /// Get item position
    pub fn item_position(&self, index: usize) -> f32 {
        index as f32 * self.item_height
    }

    pub fn set_total_items(&mut self, count: usize) {
        self.total_items = count;
    }
}
```

## File: `./src/runtime/mod.rs`
```rs
mod widget_builder;
use anyhow::{Context, Result};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent, ElementState, MouseButton};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use winit_input_helper::WinitInputHelper;
use crate::core::element::SharedElementTree;
use crate::core::widget::Widget;
use crate::core::{EventDispatcher, Theme};
use crate::layout::Constraints;
use crate::render::{select_backend, BackendType, RenderBackend};
use crate::theming::ThemeConfig;
use widget_builder::WidgetBuilder;
use std::time::{Duration, Instant};
use oneshot;
pub struct Runtime {
    event_loop: Option<EventLoop<()>>,
    root_widget: Option<Box<dyn Widget>>,
    title: String,
    width: u32,
    height: u32,
    theme_config: Option<ThemeConfig>,
}

impl Runtime {
    pub fn new(root_widget: Box<dyn Widget>) -> Self {
        Self {
            event_loop: Some(EventLoop::new().unwrap()),
            root_widget: Some(root_widget),
            title: "OxideUI Application".to_string(),
            width: 800,
            height: 600,
            theme_config: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_theme(mut self, theme: ThemeConfig) -> Self {
        self.theme_config = Some(theme);
        self
    }

    pub async fn run(self) -> Result<()> {
        let event_loop = self.event_loop.context("Event loop was taken")?;
        let root_widget = self.root_widget.context("Root widget was taken")?;
        let (tx, rx) = oneshot::channel::<()>();

        let mut app = OxideApp {
            window: None,
            renderer: None,
            backend_type: select_backend(),
            input: WinitInputHelper::new(),
            event_dispatcher: EventDispatcher::new(),
            element_tree: crate::core::element::new_shared_element_tree(),
            exit_tx: Some(tx),
            root_widget,
            theme_config: self.theme_config,
            title: self.title,
            width: self.width,
            height: self.height,
            theme: Arc::new(Theme::default()),
            last_frame_time: Instant::now(),
            frame_count: 0,
        };

        println!("🎨 OxideUI Framework Starting...");
        println!("📦 Selected renderer: {:?}", app.backend_type);
        println!(
            "🪟 Window: \"{}\" ({}x{})",
            app.title, app.width, app.height
        );

        event_loop
            .run_app(&mut app)
            .context("Failed to run application event loop")?;

        rx.await.context("Event loop shutdown channel failed")?;
        Ok(())
    }
}

struct OxideApp {
    window: Option<Arc<Window>>,
    renderer: Option<Box<dyn RenderBackend>>,
    backend_type: BackendType,
    input: WinitInputHelper,
    event_dispatcher: EventDispatcher,
    element_tree: SharedElementTree,
    exit_tx: Option<oneshot::Sender<()>>,
    root_widget: Box<dyn Widget>,
    theme_config: Option<ThemeConfig>,
    title: String,
    width: u32,
    height: u32,
    theme: Arc<Theme>,
    last_frame_time: Instant,
    frame_count: u64,
}

impl ApplicationHandler for OxideApp {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        self.input.step();
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            println!(
                "🪟 Creating window: \"{}\" ({}x{})",
                self.title, self.width, self.height
            );

            let window_attributes = WindowAttributes::default()
                .with_title(&self.title)
                .with_inner_size(LogicalSize::new(self.width, self.height))
                .with_visible(true)
                .with_resizable(true)
                .with_decorations(true)
                .with_transparent(false);

            match event_loop.create_window(window_attributes) {
                Ok(window) => {
                    println!("✅ Window created successfully");
                    if let Some(config) = &self.theme_config {
                        self.theme = Arc::new(Theme::from_config(config, false));
                        println!("🎨 Theme loaded: {}", config.font_sans);
                    }

                    let window_arc = Arc::new(window);
                    self.window = Some(window_arc.clone());

                    // Create renderer based on backend type
                    let renderer = match self.backend_type {
                        #[cfg(feature = "skia-opengl")]
                        BackendType::SkiaOpenGL => {
                            use crate::render::skia_opengl::SkiaOpenGLRenderer;
                            match SkiaOpenGLRenderer::new(window_arc.clone(), event_loop) {
                                Ok(r) => Ok(Box::new(r) as Box<dyn RenderBackend>),
                                Err(e) => Err(e),
                            }
                        }
                        BackendType::SkiaCPU => {
                            use crate::render::skia_cpu::SkiaCPURenderer;
                            match SkiaCPURenderer::new(window_arc) {
                                Ok(r) => Ok(Box::new(r) as Box<dyn RenderBackend>),
                                Err(e) => Err(e),
                            }
                        }
                        BackendType::Softbuffer => {
                            use crate::render::softbuffer::SoftbufferRenderer;
                            match SoftbufferRenderer::new(window_arc.clone()) {
                                Ok(r) => Ok(Box::new(r) as Box<dyn RenderBackend>),
                                Err(e) => Err(e),
                            }
                        }
                    };

                    match renderer {
                        Ok(renderer) => {
                            println!("✅ Renderer ({}) initialized", renderer.name());
                            self.renderer = Some(renderer);
                            if let Some(window) = &self.window {
                                window.request_redraw();
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to create renderer: {}", e);
                            event_loop.exit();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to create window: {}", e);
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.input.process_window_event(&event) {
            self.process_input_events();
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("🛑 Close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.rebuild_and_render();
            }
            WindowEvent::Resized(size) => {
                println!("📐 Window resized to: {}x{}", size.width, size.height);
                if let Some(renderer) = &mut self.renderer {
                    if let Err(e) = renderer.resize(size.width, size.height) {
                        eprintln!("❌ Resize error: {}", e);
                    }
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state: ElementState::Pressed,
                ..
            } => {
                self.process_mouse_click();
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        self.input.process_device_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.input.end_step();
        if self.input.close_requested() || self.input.destroyed() {
            println!("🛑 Application exit requested");
            event_loop.exit();
            return;
        }

        // Request redraw for animation frames
        if let Some(window) = &self.window {
            if self.input.any_pressed() {
                window.request_redraw();
            }
        }

        // Calculate and display FPS every 60 frames
        self.frame_count += 1;
        if self.frame_count % 60 == 0 {
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_frame_time);
            let fps = 60.0 / elapsed.as_secs_f32();
            println!("📊 FPS: {:.1}", fps);
            self.last_frame_time = now;
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("👋 Application exiting...");
        if let Some(mut renderer) = self.renderer.take() {
            renderer.cleanup();
        }
        if let Some(tx) = self.exit_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl OxideApp {
    fn process_input_events(&mut self) {
        // Process keyboard events
        if let Some(key) = self.input.key_pressed() {
            println!("⌨️ Key pressed: {:?}", key);
        }

        // Process mouse events
        if let Some((x, y)) = self.input.mouse() {
            if self.input.mouse_pressed(0) {
                println!("🖱️ Mouse pressed at: ({}, {})", x, y);
            }
        }
    }

    fn process_mouse_click(&mut self) {
        if let Some((x, y)) = self.input.mouse() {
            println!("🖱️ Click detected at: ({}, {})", x, y);

            // This is where you'd trigger widget interactions
            // For now, just force a rebuild to show we're responding
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }

    fn rebuild_and_render(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            let size = if let Some(window) = &self.window {
                window.inner_size()
            } else {
                return;
            };

            let constraints = Constraints::new(
                0.0, size.width as f32,
                0.0, size.height as f32
            );

            let builder = WidgetBuilder::new(self.theme.clone());
            let root_render_obj = builder.build_widget_tree(&self.root_widget, constraints);

            println!("🎨 Rendering frame with constraints: {:?}", constraints);

            if let Err(e) = renderer.draw_render_object(&root_render_obj, size.width, size.height) {
                eprintln!("❌ Draw error: {}", e);
                return;
            }

            if let Err(e) = renderer.present() {
                eprintln!("❌ Present error: {}", e);
                return;
            }

            if let Some(window) = &self.window {
                window.pre_present_notify();
            }
        }
    }
}
```

## File: `./src/runtime/widget_builder.rs`
```rs
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
```

## File: `./src/widgets/complex_widgets/chart.rs`
```rs
use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Chart {
    pub data: Vec<f32>,
    pub labels: Vec<String>,
    pub chart_type: ChartType,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub show_grid: bool,
    pub show_labels: bool,
    pub colors: Vec<Color>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Area,
}

impl Chart {
    pub fn new(data: Vec<f32>) -> Self {
        Self {
            data,
            labels: Vec::new(),
            chart_type: ChartType::Bar,
            width: None,
            height: None,
            show_grid: true,
            show_labels: true,
            colors: Vec::new(),
            tooltip: None,
            key: None,
        }
    }

    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    pub fn with_chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = chart_type;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn with_colors(mut self, colors: Vec<Color>) -> Self {
        self.colors = colors;
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

impl StatelessWidget for Chart {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(400.0);
        let height = self.height.unwrap_or(300.0);

        let padding = 40.0;
        let chart_width = width - (padding * 2.0);
        let chart_height = height - (padding * 2.0);

        let mut render_objects = Vec::new();

        // Chart background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            theme.card,
        ));

        // Chart border
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(width - 1.0, 0.0, 1.0, height),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, height - 1.0, width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, 1.0, height),
            theme.border,
        ));

        // Grid lines
        if self.show_grid {
            let grid_color = theme.border.with_alpha(50);

            // Vertical grid lines
            for i in 0..=10 {
                let x = padding + (i as f32 * chart_width / 10.0);
                render_objects.push(RenderObject::rect(
                    Rect::new(x, padding, 1.0, chart_height),
                    grid_color,
                ));
            }

            // Horizontal grid lines
            for i in 0..=5 {
                let y = padding + (i as f32 * chart_height / 5.0);
                render_objects.push(RenderObject::rect(
                    Rect::new(padding, y, chart_width, 1.0),
                    grid_color,
                ));
            }
        }

        if !self.data.is_empty() {
            let max_value = self.data.iter().cloned().fold(0.0, f32::max).max(1.0);
            let item_count = self.data.len();
            let item_width = chart_width / item_count as f32;

            let default_colors = vec![
                theme.chart_1,
                theme.chart_2,
                theme.chart_3,
                theme.chart_4,
                theme.chart_5,
            ];
            let colors = if self.colors.is_empty() { &default_colors } else { &self.colors };

            match self.chart_type {
                ChartType::Bar => {
                    // Draw bars
                    for (i, &value) in self.data.iter().enumerate() {
                        let bar_height = (value / max_value) * chart_height;
                        let x = padding + (i as f32 * item_width) + 4.0;
                        let y = padding + chart_height - bar_height;
                        let bar_width = item_width - 8.0;

                        let color_index = i % colors.len();
                        render_objects.push(RenderObject::rect(
                            Rect::new(x, y, bar_width, bar_height),
                            colors[color_index],
                        ));

                        // Value label
                        if self.show_labels && bar_height > 20.0 {
                            render_objects.push(RenderObject::text(
                                format!("{:.1}", value),
                                TextStyle {
                                    font_family: theme.font_sans.clone(),
                                    font_size: 10.0,
                                    color: theme.foreground,
                                    bold: false,
                                    italic: false,
                                },
                                Point::new(x + bar_width / 2.0 - 10.0, y - 15.0),
                            ));
                        }
                    }
                }
                ChartType::Line => {
                    // Draw line chart
                    let points: Vec<Point> = self.data.iter().enumerate().map(|(i, &value)| {
                        let x = padding + (i as f32 * item_width) + (item_width / 2.0);
                        let y = padding + chart_height - ((value / max_value) * chart_height);
                        Point::new(x, y)
                    }).collect();

                    // Draw line
                    for i in 0..points.len() - 1 {
                        let start = points[i];
                        let end = points[i + 1];

                        // Simple line drawing (would need proper line rendering)
                        let line_color = colors[0];
                        // For simplicity, draw a rectangle representing the line
                        let dx = end.x - start.x;
                        let dy = end.y - start.y;
                        let length = (dx * dx + dy * dy).sqrt();
                        let angle = dy.atan2(dx);

                        // Note: This is a simplification. Real line drawing would need proper rendering.
                        render_objects.push(RenderObject::rect(
                            Rect::new(start.x, start.y, length, 2.0),
                            line_color,
                        ));
                    }
                }
                ChartType::Pie => {
                    // Draw pie chart (simplified as donut chart)
                    let center_x = padding + chart_width / 2.0;
                    let center_y = padding + chart_height / 2.0;
                    let radius = chart_height.min(chart_width) / 3.0;

                    let total: f32 = self.data.iter().sum();
                    let mut current_angle = 0.0;

                    for (i, &value) in self.data.iter().enumerate() {
                        let slice_angle = (value / total) * 360.0;
                        let color_index = i % colors.len();

                        // Draw slice (simplified as circle segment)
                        // In a real implementation, we'd draw proper arcs
                        render_objects.push(RenderObject::rect(
                            Rect::new(center_x - radius, center_y - radius, radius * 2.0, radius * 2.0),
                            colors[color_index].with_alpha(150),
                        ));

                        current_angle += slice_angle;
                    }
                }
                ChartType::Area => {
                    // Draw area chart (simplified as filled polygon)
                    let points: Vec<Point> = self.data.iter().enumerate().map(|(i, &value)| {
                        let x = padding + (i as f32 * item_width);
                        let y = padding + chart_height - ((value / max_value) * chart_height);
                        Point::new(x, y)
                    }).collect();

                    // Draw area (simplified as series of rectangles)
                    for i in 0..points.len() - 1 {
                        let start = points[i];
                        let end = points[i + 1];

                        let area_color = colors[0].with_alpha(100);
                        let area_height = chart_height - start.y.min(end.y);

                        render_objects.push(RenderObject::rect(
                            Rect::new(start.x, start.y.min(end.y), end.x - start.x, area_height),
                            area_color,
                        ));
                    }
                }
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Chart {
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
```

## File: `./src/render/mod.rs`
```rs
//! Rendering backend abstractions for OxideUI

pub mod skia_opengl;
pub mod skia_cpu;
pub mod softbuffer;
pub mod rendering_impl;
mod pipeline;
pub mod text;

pub use crate::render::text::{FontManager, TextLayout, TextCache, FontDescriptor, FontWeight, FontStyle};

use anyhow::Result;
use crate::core::RenderObject;

/// Core trait for all rendering backends
pub trait RenderBackend: Send {
    /// Draw a frame (fallback when no render object provided)
    fn draw(&mut self, width: u32, height: u32) -> Result<()>;

    /// Draw a complete render object tree - THIS IS THE REAL RENDERING
    fn draw_render_object(&mut self, render_obj: &RenderObject, width: u32, height: u32) -> Result<()> {
        // Default implementation falls back to basic draw
        self.draw(width, height)
    }

    /// Present the rendered frame to screen
    fn present(&mut self) -> Result<()>;

    /// Resize the rendering surface
    fn resize(&mut self, width: u32, height: u32) -> Result<()>;

    /// Clean up rendering resources
    fn cleanup(&mut self);

    /// Get backend name for debugging
    fn name(&self) -> &str {
        "Unknown"
    }
}

/// Available renderer backends
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackendType {
    /// Skia with OpenGL acceleration
    SkiaOpenGL,
    /// Skia with CPU rasterization
    SkiaCPU,
    /// Pure software rendering
    Softbuffer,
}

/// Select the best available rendering backend
pub fn select_backend() -> BackendType {
    match std::env::var("OXIDEUI_RENDERER") {
        Ok(val) => match val.to_lowercase().as_str() {
            "skia" | "skia-opengl" | "opengl" | "gpu" => {
                println!("[Backend] User requested: Skia OpenGL");
                BackendType::SkiaOpenGL
            }
            "skia-cpu" | "cpu-skia" | "skia-cpu-fallback" => {
                println!("[Backend] User requested: Skia CPU");
                BackendType::SkiaCPU
            }
            "softbuffer" | "cpu" | "software" => {
                println!("[Backend] User requested: Softbuffer");
                BackendType::Softbuffer
            }
            _ => {
                eprintln!("[Backend] Unknown renderer '{}', defaulting to Softbuffer", val);
                BackendType::Softbuffer
            }
        },
        Err(_) => {
            // Auto-select based on available features
            #[cfg(feature = "skia-opengl")]
            {
                println!("[Backend] Auto-selected: Skia OpenGL (GPU accelerated)");
                BackendType::SkiaOpenGL
            }
            #[cfg(all(feature = "skia-cpu", not(feature = "skia-opengl")))]
            {
                println!("[Backend] Auto-selected: Skia CPU");
                BackendType::SkiaCPU
            }
            #[cfg(not(any(feature = "skia-opengl", feature = "skia-cpu")))]
            {
                println!("[Backend] Auto-selected: Softbuffer (fallback)");
                BackendType::Softbuffer
            }
        }
    }
}
```

## File: `./src/render/softbuffer.rs`
```rs
use anyhow::{anyhow, Result};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::window::Window;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use super::RenderBackend;

pub struct SoftbufferRenderer {
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    context: Option<Context<Arc<Window>>>,
    width: u32,
    height: u32,
    window: Arc<Window>,
}

impl SoftbufferRenderer {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        println!("[Softbuffer] Initializing renderer...");

        let context = Context::new(window.clone())
            .map_err(|e| anyhow!("Failed to create softbuffer context: {}", e))?;

        println!("[Softbuffer] Renderer initialized successfully!");

        Ok(Self {
            surface: None,
            context: Some(context),
            width: 0,
            height: 0,
            window,
        })
    }

    fn ensure_surface(&mut self) -> Result<&mut Surface<Arc<Window>, Arc<Window>>> {
        if self.surface.is_none() {
            let size = self.window.inner_size();
            let width = size.width.max(1);
            let height = size.height.max(1);

            let context = self.context.as_ref().unwrap();
            self.surface = Some(Surface::new(context, self.window.clone())
                .map_err(|e| anyhow!("Softbuffer error: {}", e))?);

            self.width = width;
            self.height = height;

            self.surface.as_mut().unwrap().resize(
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            ).map_err(|e| anyhow!("Failed to resize surface: {}", e))?;
        }

        Ok(self.surface.as_mut().unwrap())
    }

    fn render_object_to_buffer(
        buffer: &mut [u32],
        obj: &RenderObject,
        width: u32,
        height: u32,
    ) {
        match obj {
            RenderObject::Rect { rect, paint } => {
                Self::draw_rect_to_buffer(buffer, rect, paint.color, width, height);
            }
            RenderObject::Text { content, style, position } => {
                Self::draw_text_to_buffer(buffer, content, style, position, width, height);
            }
            RenderObject::Group { children } => {
                for child in children {
                    Self::render_object_to_buffer(buffer, child, width, height);
                }
            }
            RenderObject::Transform { child, .. } => {
                Self::render_object_to_buffer(buffer, child, width, height);
            }
            RenderObject::Clip { child, .. } => {
                Self::render_object_to_buffer(buffer, child, width, height);
            }
            _ => {}
        }
    }

    fn draw_rect_to_buffer(
        buffer: &mut [u32],
        rect: &Rect,
        color: Color,
        width: u32,
        height: u32,
    ) {
        let x1 = rect.x.max(0.0).min(width as f32) as u32;
        let y1 = rect.y.max(0.0).min(height as f32) as u32;
        let x2 = ((rect.x + rect.width).max(0.0).min(width as f32)) as u32;
        let y2 = ((rect.y + rect.height).max(0.0).min(height as f32)) as u32;

        let color_u32 = ((color.a as u32) << 24)
            | ((color.r as u32) << 16)
            | ((color.g as u32) << 8)
            | (color.b as u32);

        for y in y1..y2 {
            for x in x1..x2 {
                let idx = (y * width + x) as usize;
                if idx < buffer.len() {
                    buffer[idx] = color_u32;
                }
            }
        }
    }

    fn draw_text_to_buffer(
        buffer: &mut [u32],
        text: &str,
        style: &TextStyle,
        position: &Point,
        width: u32,
        height: u32,
    ) {
        let x = position.x.max(0.0) as u32;
        let y = position.y.max(0.0) as u32;
        let char_width = (style.font_size * 0.6) as u32;
        let char_height = (style.font_size * 1.2) as u32;
        let color_u32 = ((style.color.a as u32) << 24)
            | ((style.color.r as u32) << 16)
            | ((style.color.g as u32) << 8)
            | (style.color.b as u32);

        for (i, ch) in text.chars().enumerate() {
            let char_x = x + (i as u32 * char_width);
            if char_x >= width || y >= height {
                break;
            }

            if ch.is_whitespace() {
                continue;
            }

            for dy in 0..char_height.min(height - y) {
                for dx in 0..(char_width - 2).min(width - char_x) {
                    let px = char_x + dx;
                    let py = y + dy;
                    let idx = (py * width + px) as usize;
                    if idx < buffer.len() {
                        buffer[idx] = color_u32;
                    }
                }
            }
        }
    }
}

impl RenderBackend for SoftbufferRenderer {
    fn draw(&mut self, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        let surface = self.ensure_surface()?;
        let mut buffer = surface.buffer_mut()
            .map_err(|e| anyhow!("Failed to get buffer: {}", e))?;

        // Clear with white background
        for pixel in buffer.iter_mut() {
            *pixel = 0xFFFFFFFF;
        }

        buffer.present()
            .map_err(|e| anyhow!("Failed to present buffer: {}", e))?;

        Ok(())
    }

    fn draw_render_object(&mut self, render_obj: &RenderObject, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        let surface = self.ensure_surface()?;
        let mut buffer = surface.buffer_mut()
            .map_err(|e| anyhow!("Failed to get buffer: {}", e))?;

        // Clear with white background
        for pixel in buffer.iter_mut() {
            *pixel = 0xFFFFFFFF;
        }

        Self::render_object_to_buffer(&mut buffer, render_obj, width, height);

        buffer.present()
            .map_err(|e| anyhow!("Failed to present buffer: {}", e))?;

        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        let width = width.max(1);
        let height = height.max(1);

        self.width = width;
        self.height = height;

        if let Some(surface) = &mut self.surface {
            surface.resize(
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            ).map_err(|e| anyhow!("Failed to resize surface: {}", e))?;
        }

        Ok(())
    }

    fn cleanup(&mut self) {
        println!("[Softbuffer] Cleaning up renderer");
        self.surface = None;
        self.context = None;
    }

    fn name(&self) -> &str {
        "Softbuffer"
    }
}
```

## File: `./src/widgets/complex_widgets/sonner.rs`
```rs
use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Sonner {
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub duration_ms: u64,
    pub position: ToastPosition,
    pub visible: bool,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastVariant {
    Default,
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Sonner {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
            duration_ms: 3000,
            position: ToastPosition::BottomRight,
            visible: false,
            on_close: None,
            key: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_close = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Sonner {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        if !self.visible {
            return WidgetNode::None;
        }

        let theme = ctx.theme();
        let screen_width = ctx.constraints.max_width;
        let screen_height = ctx.constraints.max_height;

        let toast_width = 350.0;
        let toast_height = if self.description.is_some() { 100.0 } else { 70.0 };
        let padding = 16.0;

        // Calculate position based on toast position
        let (x, y) = match self.position {
            ToastPosition::TopLeft => (20.0, 20.0),
            ToastPosition::TopCenter => ((screen_width - toast_width) / 2.0, 20.0),
            ToastPosition::TopRight => (screen_width - toast_width - 20.0, 20.0),
            ToastPosition::BottomLeft => (20.0, screen_height - toast_height - 20.0),
            ToastPosition::BottomCenter => ((screen_width - toast_width) / 2.0, screen_height - toast_height - 20.0),
            ToastPosition::BottomRight => (screen_width - toast_width - 20.0, screen_height - toast_height - 20.0),
        };

        let bg_color = match self.variant {
            ToastVariant::Default => theme.background,
            ToastVariant::Success => Color::from_hex(0x10B981),
            ToastVariant::Error => theme.destructive,
            ToastVariant::Warning => Color::from_hex(0xF59E0B),
            ToastVariant::Info => theme.primary,
        };

        let text_color = match self.variant {
            ToastVariant::Default => theme.foreground,
            ToastVariant::Success => Color::WHITE,
            ToastVariant::Error => theme.destructive_foreground,
            ToastVariant::Warning => Color::from_hex(0x78350F),
            ToastVariant::Info => theme.primary_foreground,
        };

        let mut render_objects = Vec::new();

        // Toast background with shadow
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, toast_width, toast_height),
            bg_color,
        ));

        // Toast border
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, toast_width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x + toast_width - 1.0, y, 1.0, toast_height),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x, y + toast_height - 1.0, toast_width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(x, y, 1.0, toast_height),
            theme.border,
        ));

        // Icon based on variant
        let icon = match self.variant {
            ToastVariant::Default => "💬",
            ToastVariant::Success => "✅",
            ToastVariant::Error => "❌",
            ToastVariant::Warning => "⚠️",
            ToastVariant::Info => "ℹ️",
        };

        render_objects.push(RenderObject::text(
            icon.to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 20.0,
                color: text_color,
                bold: false,
                italic: false,
            },
            Point::new(x + padding, y + padding + 5.0),
        ));

        // Title
        render_objects.push(RenderObject::text(
            self.title.clone(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 14.0,
                color: text_color,
                bold: true,
                italic: false,
            },
            Point::new(x + padding + 30.0, y + padding + 5.0),
        ));

        // Description
        if let Some(description) = &self.description {
            render_objects.push(RenderObject::text(
                description.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 12.0,
                    color: text_color.with_alpha(180),
                    bold: false,
                    italic: false,
                },
                Point::new(x + padding + 30.0, y + padding + 25.0),
            ));
        }

        // Close button
        let close_button_size = 24.0;
        let close_x = x + toast_width - close_button_size - 8.0;
        let close_y = y + 8.0;

        render_objects.push(RenderObject::rect(
            Rect::new(close_x, close_y, close_button_size, close_button_size),
            text_color.with_alpha(50),
        ));

        render_objects.push(RenderObject::text(
            "×".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 18.0,
                color: text_color,
                bold: true,
                italic: false,
            },
            Point::new(close_x + 4.0, close_y + 4.0),
        ));

        // Progress bar (showing time remaining)
        let progress_width = toast_width - (padding * 2.0);
        render_objects.push(RenderObject::rect(
            Rect::new(x + padding, y + toast_height - 4.0, progress_width, 2.0),
            text_color.with_alpha(100),
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Sonner {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        if !self.visible {
            return EventResult::Unhandled;
        }

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() => {
                let screen_width = 800.0;
                let screen_height = 600.0;
                let toast_width = 350.0;
                let toast_height = if self.description.is_some() { 100.0 } else { 70.0 };

                let (x, y) = match self.position {
                    ToastPosition::TopLeft => (20.0, 20.0),
                    ToastPosition::TopCenter => ((screen_width - toast_width) / 2.0, 20.0),
                    ToastPosition::TopRight => (screen_width - toast_width - 20.0, 20.0),
                    ToastPosition::BottomLeft => (20.0, screen_height - toast_height - 20.0),
                    ToastPosition::BottomCenter => ((screen_width - toast_width) / 2.0, screen_height - toast_height - 20.0),
                    ToastPosition::BottomRight => (screen_width - toast_width - 20.0, screen_height - toast_height - 20.0),
                };

                let close_button_size = 24.0;
                let close_x = x + toast_width - close_button_size - 8.0;
                let close_y = y + 8.0;

                let close_button_rect = Rect::new(close_x, close_y, close_button_size, close_button_size);

                if close_button_rect.contains(position.x, position.y) {
                    if let Some(on_close) = &self.on_close {
                        on_close();
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
```

## File: `./src/render/skia_cpu.rs`
```rs
use anyhow::{Result, anyhow};
use skia_safe::{ColorType, ISize, ImageInfo, Surface, AlphaType};
use skia_safe::image::CachingHint;
use softbuffer::Context;
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::window::Window;

use super::RenderBackend;
use crate::core::RenderObject;
use crate::render::rendering_impl::SkiaRenderer;

pub struct SkiaCPURenderer {
    surface: Surface,
    width: u32,
    height: u32,
    softbuffer_surface: softbuffer::Surface<Arc<Window>, Arc<Window>>,
    window: Arc<Window>,
    skia_renderer: SkiaRenderer,
}

unsafe impl Send for SkiaCPURenderer {}

impl SkiaCPURenderer {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        println!("[Skia CPU] Initializing renderer...");

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let info = ImageInfo::new(
            ISize::new(width as i32, height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        let surface = skia_safe::surfaces::raster(&info, None, None)
            .ok_or_else(|| anyhow!("Failed to create CPU surface"))?;

        let context = Context::new(window.clone())
            .map_err(|e| anyhow!("Failed to create softbuffer context: {}", e))?;

        let softbuffer_surface = softbuffer::Surface::new(&context, window.clone())
            .map_err(|e| anyhow!("Failed to create softbuffer surface: {}", e))?;

        println!("[Skia CPU] Renderer initialized successfully!");

        Ok(Self {
            surface,
            width,
            height,
            softbuffer_surface,
            window,
            skia_renderer: SkiaRenderer::new(),
        })
    }
}

impl RenderBackend for SkiaCPURenderer {
    fn draw(&mut self, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        let canvas = self.surface.canvas();

        // Clear with background
        self.skia_renderer.clear(canvas, crate::core::Color::from_hex(0xFFFFFF));

        Ok(())
    }
    

    fn draw_render_object(&mut self, render_obj: &RenderObject, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        let canvas = self.surface.canvas();

        // Clear canvas
        self.skia_renderer.clear(canvas, crate::core::Color::from_hex(0xFFFFFF));

        // Actually render the widget tree!
        self.skia_renderer.render(canvas, render_obj);

        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        // Copy Skia surface pixels to softbuffer
        let image = self.surface.image_snapshot();
        let info = ImageInfo::new(
            (self.width as i32, self.height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        let row_bytes = (self.width * 4) as usize;
        let mut pixel_data = vec![0u8; (self.width * self.height * 4) as usize];

        if !image.read_pixels(&info, &mut pixel_data, row_bytes, (0, 0), CachingHint::Disallow) {
            return Err(anyhow!("Failed to read pixels from Skia surface"));
        }

        let width_nz = NonZeroU32::new(self.width).ok_or_else(|| anyhow!("Width must be > 0"))?;
        let height_nz = NonZeroU32::new(self.height).ok_or_else(|| anyhow!("Height must be > 0"))?;

        let mut buffer = self
            .softbuffer_surface
            .buffer_mut()
            .map_err(|e| anyhow!("Failed to get buffer: {}", e))?;

        // Convert RGBA to ARGB for softbuffer
        for (i, chunk) in pixel_data.chunks_exact(4).enumerate() {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3] as u32;

            buffer[i] = (a << 24) | (r << 16) | (g << 8) | b;
        }

        buffer
            .present()
            .map_err(|e| anyhow!("Failed to present buffer: {}", e))?;

        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.width = width.max(1);
        self.height = height.max(1);

        let info = ImageInfo::new(
            ISize::new(self.width as i32, self.height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        self.surface = skia_safe::surfaces::raster(&info, None, None)
            .ok_or_else(|| anyhow!("Failed to resize CPU surface"))?;

        self.softbuffer_surface
            .resize(
                NonZeroU32::new(self.width).unwrap(),
                NonZeroU32::new(self.height).unwrap(),
            )
            .map_err(|e| anyhow!("Failed to resize softbuffer: {}", e))?;

        Ok(())
    }

    fn cleanup(&mut self) {
        println!("[Skia CPU] Cleaning up renderer");
    }

    fn name(&self) -> &str {
        "Skia CPU"
    }
}
```

## File: `./src/render/skia_opengl.rs`
```rs
use anyhow::{Context, Result};
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::surface::{GlSurface, Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use skia_safe::{gpu, ColorType, Surface};
use std::ffi::CString;
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use super::RenderBackend;
use crate::core::render_object::RenderObject;
use crate::render::rendering_impl::SkiaRenderer;
use winit::dpi::PhysicalSize;

pub struct SkiaOpenGLRenderer {
    gl_context: PossiblyCurrentContext,
    gl_surface: GlutinSurface<WindowSurface>,
    skia_context: gpu::DirectContext,
    skia_surface: Option<Surface>,
    skia_renderer: SkiaRenderer,
    width: u32,
    height: u32,
    window: Arc<Window>,
}

unsafe impl Send for SkiaOpenGLRenderer {}

impl SkiaOpenGLRenderer {
    pub fn new(window: Arc<Window>, event_loop: &ActiveEventLoop) -> Result<Self> {
        println!("[Skia OpenGL] Initializing renderer...");
        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        println!("[Skia OpenGL] Window size: {}x{}", width, height);

        // WAYLAND COMPATIBLE CONFIG
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_depth_size(24)
            .with_stencil_size(8)
            .with_transparency(false)
            .prefer_hardware_accelerated(Some(true));

        println!("[Skia OpenGL] Creating display...");
        let display_builder = DisplayBuilder::new().with_window(Some(window.clone()));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);
                        if transparency || config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .expect("No suitable GL config found")
            })
            .map_err(|e| anyhow::anyhow!("Failed to build display: {}", e))?;

        if let Some(w) = window {
            // Drop the old window reference if needed
            println!("[Skia OpenGL] Using existing window");
        }

        println!("[Skia OpenGL] Display created");
        println!("[Skia OpenGL] Config: samples={}, stencil={}, depth={}",
                 gl_config.num_samples(),
                 gl_config.stencil_size(),
                 gl_config.depth_size());

        let raw_window_handle = window
            .as_ref()
            .unwrap_or(&window)
            .window_handle()
            .context("Failed to get window handle")?
            .as_raw();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::Gles(Some(
                glutin::context::Version::new(3, 0),
            )))
            .build(Some(raw_window_handle));

        println!("[Skia OpenGL] Creating GL context...");
        let gl_display = gl_config.display();
        let gl_context = unsafe {
            gl_display.create_context(&gl_config, &context_attributes)
        }.or_else(|_| {
            println!("[Skia OpenGL] GLES failed, trying OpenGL 3.3...");
            let attrs = ContextAttributesBuilder::new()
                .with_context_api(glutin::context::ContextApi::OpenGl(Some(
                    glutin::context::Version::new(3, 3),
                )))
                .build(Some(raw_window_handle));
            unsafe { gl_display.create_context(&gl_config, &attrs) }
        })
            .context("Failed to create GL context")?;

        let size = window.as_ref().unwrap_or(&window).inner_size();
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new()
            .build(
                raw_window_handle,
                NonZeroU32::new(size.width.max(1)).unwrap(),
                NonZeroU32::new(size.height.max(1)).unwrap(),
            );

        println!("[Skia OpenGL] Creating window surface...");
        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &surface_attributes)
                .context("Failed to create window surface")?
        };

        let gl_context = gl_context
            .make_current(&gl_surface)
            .context("Failed to make context current")?;

        println!("[Skia OpenGL] Loading GL functions...");
        gl::load_with(|symbol| {
            let cstr = CString::new(symbol).unwrap();
            gl_display.get_proc_address(cstr.as_c_str()) as *const _
        });

        println!("[Skia OpenGL] Creating Skia GL interface...");
        let interface = gpu::gl::Interface::new_load_with(|name| {
            let cstr = CString::new(name).unwrap();
            gl_display.get_proc_address(cstr.as_c_str())
        })
            .context("Failed to create Skia GL interface")?;

        println!("[Skia OpenGL] Creating Skia DirectContext...");
        let mut skia_context = gpu::direct_contexts::make_gl(interface, None)
            .context("Failed to create Skia DirectContext")?;

        // Initialize Skia surface later after resize
        println!("[Skia OpenGL] Renderer initialized successfully!");

        Ok(Self {
            gl_context,
            gl_surface,
            skia_context,
            skia_surface: None,
            skia_renderer: SkiaRenderer::new(),
            width: width as u32,
            height: height as u32,
            window: window.unwrap_or(window.clone()),
        })
    }

    fn recreate_skia_surface(&mut self) -> Result<()> {
        let size = self.window.inner_size();
        let width = size.width.max(1) as i32;
        let height = size.height.max(1) as i32;

        let mut fboid: i32 = 0;
        unsafe {
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid);
        }

        let fb_info = gpu::gl::FramebufferInfo {
            fboid: fboid as u32,
            format: gpu::gl::Format::RGBA8.into(),
            ..Default::default()
        };

        let samples = 0; // No MSAA for now
        let stencil = 8;

        let backend_render_target = gpu::backend_render_targets::make_gl(
            (width, height),
            samples,
            stencil,
            fb_info,
        );

        self.skia_surface = Some(gpu::surfaces::wrap_backend_render_target(
            &mut self.skia_context,
            &backend_render_target,
            gpu::SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        ).context("Failed to create Skia surface")?);

        Ok(())
    }
}

impl RenderBackend for SkiaOpenGLRenderer {
    fn draw(&mut self, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        if self.skia_surface.is_none() {
            self.recreate_skia_surface()?;
        }

        if let Some(surface) = &self.skia_surface {
            let canvas = surface.canvas();
            self.skia_renderer.clear(canvas, crate::core::Color::from_hex(0xFFFFFF));
            self.skia_context.flush_and_submit();
        }

        Ok(())
    }

    fn draw_render_object(&mut self, render_obj: &RenderObject, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        if self.skia_surface.is_none() {
            self.recreate_skia_surface()?;
        }

        if let Some(surface) = &self.skia_surface {
            let canvas = surface.canvas();
            self.skia_renderer.clear(canvas, crate::core::Color::from_hex(0xFFFFFF));
            self.skia_renderer.render(canvas, render_obj);
            self.skia_context.flush_and_submit();
        }

        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        self.gl_surface
            .swap_buffers(&self.gl_context)
            .context("Failed to swap buffers")?;
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        let width = width.max(1);
        let height = height.max(1);
        println!("[Skia OpenGL] Resizing to {}x{}", width, height);

        self.width = width;
        self.height = height;

        let size = PhysicalSize::new(width, height);
        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        // Recreate Skia surface with new dimensions
        self.recreate_skia_surface()?;

        Ok(())
    }

    fn cleanup(&mut self) {
        println!("[Skia OpenGL] Cleaning up renderer");
        self.skia_surface = None;
    }

    fn name(&self) -> &str {
        "Skia OpenGL"
    }
}
```

## File: `./src/render/pipeline.rs`
```rs
use anyhow::Result;
use std::collections::HashMap;
use crate::core::{ElementId, Rect, RenderObject};

/// Damage region tracking for efficient partial redraws
#[derive(Debug, Clone)]
pub struct DamageRegion {
    pub rects: Vec<Rect>,
}

impl DamageRegion {
    pub fn new() -> Self {
        Self { rects: Vec::new() }
    }

    pub fn add(&mut self, rect: Rect) {
        self.rects.push(rect);
    }

    pub fn merge(&mut self) -> Option<Rect> {
        if self.rects.is_empty() {
            return None;
        }

        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for rect in &self.rects {
            min_x = min_x.min(rect.x);
            min_y = min_y.min(rect.y);
            max_x = max_x.max(rect.x + rect.width);
            max_y = max_y.max(rect.y + rect.height);
        }

        Some(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
    }

    pub fn clear(&mut self) {
        self.rects.clear();
    }
}

/// Display list for batched rendering
#[derive(Debug, Clone)]
pub struct DisplayList {
    pub items: Vec<DisplayItem>,
}

#[derive(Debug, Clone)]
pub struct DisplayItem {
    pub render_object: RenderObject,
    pub bounds: Rect,
    pub transform: crate::core::render_object::Matrix,
    pub opacity: f32,
    pub clip: Option<Rect>,
}

impl DisplayList {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: DisplayItem) {
        self.items.push(item);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Cull items outside viewport
    pub fn cull(&mut self, viewport: Rect) {
        self.items.retain(|item| {
            item.bounds.x < viewport.x + viewport.width &&
                item.bounds.x + item.bounds.width > viewport.x &&
                item.bounds.y < viewport.y + viewport.height &&
                item.bounds.y + item.bounds.height > viewport.y
        });
    }
}

/// Rendering pipeline coordinator
pub struct RenderPipeline {
    pub damage: DamageRegion,
    pub display_list: DisplayList,
    pub layer_cache: HashMap<ElementId, RenderObject>,
    pub viewport: Rect,
}

impl RenderPipeline {
    pub fn new(viewport: Rect) -> Self {
        Self {
            damage: DamageRegion::new(),
            display_list: DisplayList::new(),
            layer_cache: HashMap::new(),
            viewport,
        }
    }

    /// Mark element as dirty and add to damage region
    pub fn mark_dirty(&mut self, element_id: ElementId, bounds: Rect) {
        self.damage.add(bounds);
        self.layer_cache.remove(&element_id);
    }

    /// Build display list from render tree
    pub fn build_display_list(&mut self, root: &RenderObject) {
        self.display_list.clear();
        self.build_display_list_recursive(
            root,
            crate::core::render_object::Matrix::identity(),
            1.0,
            None,
        );
        self.display_list.cull(self.viewport);
    }

    fn build_display_list_recursive(
        &mut self,
        obj: &RenderObject,
        transform: crate::core::render_object::Matrix,
        opacity: f32,
        clip: Option<Rect>,
    ) {
        match obj {
            RenderObject::Group { children } => {
                for child in children {
                    self.build_display_list_recursive(child, transform, opacity, clip);
                }
            }
            RenderObject::Transform { matrix, child } => {
                // Multiply transforms
                let new_transform = self.multiply_matrices(&transform, matrix);
                self.build_display_list_recursive(child, new_transform, opacity, clip);
            }
            RenderObject::Clip { rect, child } => {
                let new_clip = Some(self.transform_rect(*rect, &transform));
                self.build_display_list_recursive(child, transform, opacity, new_clip);
            }
            _ => {
                // Add to display list
                let bounds = self.calculate_bounds(obj, &transform);
                self.display_list.add(DisplayItem {
                    render_object: obj.clone(),
                    bounds,
                    transform,
                    opacity,
                    clip,
                });
            }
        }
    }

    fn multiply_matrices(
        &self,
        a: &crate::core::render_object::Matrix,
        b: &crate::core::render_object::Matrix,
    ) -> crate::core::render_object::Matrix {
        let mut result = crate::core::render_object::Matrix::identity();
        for i in 0..3 {
            for j in 0..3 {
                result.values[i][j] = 0.0;
                for k in 0..3 {
                    result.values[i][j] += a.values[i][k] * b.values[k][j];
                }
            }
        }
        result
    }

    fn transform_rect(&self, rect: Rect, matrix: &crate::core::render_object::Matrix) -> Rect {
        // Transform rect corners
        let x1 = rect.x * matrix.values[0][0] + rect.y * matrix.values[0][1] + matrix.values[0][2];
        let y1 = rect.x * matrix.values[1][0] + rect.y * matrix.values[1][1] + matrix.values[1][2];

        let x2 = (rect.x + rect.width) * matrix.values[0][0] + (rect.y + rect.height) * matrix.values[0][1] + matrix.values[0][2];
        let y2 = (rect.x + rect.width) * matrix.values[1][0] + (rect.y + rect.height) * matrix.values[1][1] + matrix.values[1][2];

        Rect::new(
            x1.min(x2),
            y1.min(y2),
            (x2 - x1).abs(),
            (y2 - y1).abs(),
        )
    }

    fn calculate_bounds(&self, obj: &RenderObject, transform: &crate::core::render_object::Matrix) -> Rect {
        match obj {
            RenderObject::Rect { rect, .. } => self.transform_rect(*rect, transform),
            RenderObject::Text { position, .. } => {
                // Approximate text bounds
                self.transform_rect(Rect::new(position.x, position.y, 100.0, 20.0), transform)
            }
            RenderObject::Image { size } => {
                self.transform_rect(Rect::from_size(*size), transform)
            }
            _ => Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn update_viewport(&mut self, viewport: Rect) {
        self.viewport = viewport;
    }

    pub fn has_damage(&self) -> bool {
        !self.damage.rects.is_empty()
    }

    pub fn clear_damage(&mut self) {
        self.damage.clear();
    }
}
```

## File: `./src/render/rendering_impl.rs`
```rs
use crate::core::render_object::{Color as OxColor, Matrix, Point, Rect, RenderObject, TextStyle};
use skia_safe::{Canvas, Color as SkColor, Font, FontMgr, FontStyle, Paint, PaintStyle, Typeface};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle as SkTextStyle};

pub struct SkiaRenderer {
    font_cache: std::collections::HashMap<String, Typeface>,
    font_mgr: FontMgr,
    font_collection: FontCollection,
}

impl SkiaRenderer {
    pub fn new() -> Self {
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::new(), None);

        Self {
            font_cache: std::collections::HashMap::new(),
            font_mgr: FontMgr::new(),
            font_collection,
        }
    }

    pub fn render(&mut self, canvas: &Canvas, render_obj: &RenderObject) {
        match render_obj {
            RenderObject::Rect { rect, paint } => {
                self.draw_rect(canvas, rect, &paint.color);
            }
            RenderObject::Text { content, style, position } => {
                self.draw_text(canvas, content, style, position);
            }
            RenderObject::Image { size } => {
                self.draw_image_placeholder(canvas, *size);
            }
            RenderObject::Clip { rect, child } => {
                canvas.save();
                canvas.clip_rect(rect.to_skia_rect(), None, None);
                self.render(canvas, child);
                canvas.restore();
            }
            RenderObject::Transform { matrix, child } => {
                canvas.save();
                canvas.concat(&self.matrix_to_skia(matrix));
                self.render(canvas, child);
                canvas.restore();
            }
            RenderObject::Group { children } => {
                for child in children {
                    self.render(canvas, child);
                }
            }
            RenderObject::None => {}
        }
    }

    fn draw_rect(&self, canvas: &Canvas, rect: &Rect, color: &OxColor) {
        let mut paint = Paint::default();
        paint.set_color(SkColor::from_argb(color.a, color.r, color.g, color.b));
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        canvas.draw_rect(rect.to_skia_rect(), &paint);
    }

    fn draw_text(&mut self, canvas: &Canvas, content: &str, style: &TextStyle, position: &Point) {
        let mut paragraph_style = ParagraphStyle::new();
        let mut text_style = SkTextStyle::new();

        let typeface = self.get_or_create_typeface(&style.font_family, style.bold, style.italic);
        text_style.set_typeface(Some(typeface));
        text_style.set_font_size(style.font_size);
        text_style.set_color(SkColor::from_argb(
            style.color.a,
            style.color.r,
            style.color.g,
            style.color.b,
        ));

        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, self.font_collection.clone());
        paragraph_builder.push_style(&text_style);
        paragraph_builder.add_text(content);

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(f32::INFINITY);

        canvas.draw_text_blob(
            paragraph.get_glyphs(0),
            (position.x, position.y + style.font_size),
            &Paint::default(),
        );
    }

    fn get_or_create_typeface(&mut self, family: &str, bold: bool, italic: bool) -> Typeface {
        let cache_key = format!(
            "{}_{}{}",
            family,
            if bold { "b" } else { "" },
            if italic { "i" } else { "" }
        );

        if let Some(typeface) = self.font_cache.get(&cache_key) {
            return typeface.clone();
        }

        let font_style = match (bold, italic) {
            (true, true) => FontStyle::bold_italic(),
            (true, false) => FontStyle::bold(),
            (false, true) => FontStyle::italic(),
            (false, false) => FontStyle::normal(),
        };

        let typeface = self
            .font_mgr
            .match_family_style(family, font_style)
            .or_else(|| self.font_mgr.match_family_style("sans-serif", font_style))
            .or_else(|| self.font_mgr.match_family_style("", font_style))
            .unwrap_or_else(|| {
                self.font_mgr
                    .legacy_make_typeface("", font_style)
                    .expect("Failed to create any typeface")
            });

        self.font_cache.insert(cache_key.clone(), typeface.clone());
        typeface
    }

    fn draw_image_placeholder(&self, canvas: &Canvas, size: crate::layout::Size) {
        let mut paint = Paint::default();
        paint.set_color(SkColor::from_rgb(200, 200, 200));
        paint.set_anti_alias(true);
        let rect = skia_safe::Rect::from_xywh(0.0, 0.0, size.width, size.height);
        canvas.draw_rect(rect, &paint);

        paint.set_color(SkColor::from_rgb(150, 150, 150));
        paint.set_stroke_width(2.0);
        paint.set_style(PaintStyle::Stroke);
        canvas.draw_line((0.0, 0.0), (size.width, size.height), &paint);
        canvas.draw_line((size.width, 0.0), (0.0, size.height), &paint);
    }

    fn matrix_to_skia(&self, matrix: &Matrix) -> skia_safe::Matrix {
        skia_safe::Matrix::new_all(
            matrix.values[0][0],
            matrix.values[0][1],
            matrix.values[0][2],
            matrix.values[1][0],
            matrix.values[1][1],
            matrix.values[1][2],
            matrix.values[2][0],
            matrix.values[2][1],
            matrix.values[2][2],
        )
    }

    pub fn clear(&mut self, canvas: &Canvas, color: OxColor) {
        canvas.clear(SkColor::from_argb(color.a, color.r, color.g, color.b));
    }
}

impl Default for SkiaRenderer {
    fn default() -> Self {
        Self::new()
    }
}
```

## File: `./src/render/text.rs`
```rs
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::core::render_object::{Color, Point, TextStyle};

/// Font weight enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

/// Font style enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Font descriptor for loading and caching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontDescriptor {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
}

impl FontDescriptor {
    pub fn new(family: impl Into<String>) -> Self {
        Self {
            family: family.into(),
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
        }
    }

    pub fn bold(mut self) -> Self {
        self.weight = FontWeight::Bold;
        self
    }

    pub fn italic(mut self) -> Self {
        self.style = FontStyle::Italic;
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }
}

/// Glyph information
#[derive(Debug, Clone)]
pub struct GlyphInfo {
    pub glyph_id: u32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub x_advance: f32,
    pub y_advance: f32,
}

/// Shaped text result
#[derive(Debug, Clone)]
pub struct ShapedText {
    pub glyphs: Vec<GlyphInfo>,
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
}

/// Text metrics
#[derive(Debug, Clone, Copy)]
pub struct TextMetrics {
    pub width: f32,
    pub height: f32,
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
}

/// Font manager for loading and caching fonts
pub struct FontManager {
    font_cache: Arc<RwLock<HashMap<FontDescriptor, Vec<u8>>>>,
    system_fonts: Vec<String>,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            font_cache: Arc::new(RwLock::new(HashMap::new())),
            system_fonts: Self::enumerate_system_fonts(),
        }
    }

    fn enumerate_system_fonts() -> Vec<String> {
        // Platform-specific font enumeration
        #[cfg(target_os = "linux")]
        {
            vec![
                "DejaVu Sans".to_string(),
                "Liberation Sans".to_string(),
                "Ubuntu".to_string(),
                "Noto Sans".to_string(),
            ]
        }

        #[cfg(target_os = "macos")]
        {
            vec![
                "SF Pro Display".to_string(),
                "Helvetica Neue".to_string(),
                "Arial".to_string(),
            ]
        }

        #[cfg(target_os = "windows")]
        {
            vec![
                "Segoe UI".to_string(),
                "Arial".to_string(),
                "Tahoma".to_string(),
            ]
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            vec!["sans-serif".to_string()]
        }
    }

    pub fn load_font(&self, descriptor: &FontDescriptor) -> Result<Vec<u8>> {
        // Check cache first
        {
            let cache = self.font_cache.read();
            if let Some(data) = cache.get(descriptor) {
                return Ok(data.clone());
            }
        }

        // Try to load from system
        let data = self.load_system_font(descriptor)?;

        // Cache it
        self.font_cache.write().insert(descriptor.clone(), data.clone());

        Ok(data)
    }

    fn load_system_font(&self, descriptor: &FontDescriptor) -> Result<Vec<u8>> {
        // Platform-specific font loading
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let paths = vec![
                format!("/usr/share/fonts/truetype/{}.ttf", descriptor.family.to_lowercase().replace(" ", "-")),
                format!("/usr/share/fonts/TTF/{}.ttf", descriptor.family),
                format!("/usr/local/share/fonts/{}.ttf", descriptor.family),
            ];

            for path in paths {
                if let Ok(data) = fs::read(&path) {
                    return Ok(data);
                }
            }
        }

        Err(anyhow!("Font not found: {}", descriptor.family))
    }

    pub fn measure_text(&self, text: &str, style: &TextStyle) -> Result<TextMetrics> {
        // Simplified measurement - in production, use HarfBuzz or similar
        let char_count = text.chars().count();
        let avg_char_width = style.font_size * 0.6;

        Ok(TextMetrics {
            width: avg_char_width * char_count as f32,
            height: style.font_size * 1.2,
            ascent: style.font_size * 0.8,
            descent: style.font_size * 0.2,
            line_gap: style.font_size * 0.2,
        })
    }

    pub fn shape_text(&self, text: &str, style: &TextStyle) -> Result<ShapedText> {
        // Simplified shaping - production should use HarfBuzz
        let metrics = self.measure_text(text, style)?;

        let mut glyphs = Vec::new();
        let mut x_pos = 0.0;

        for (i, _ch) in text.chars().enumerate() {
            glyphs.push(GlyphInfo {
                glyph_id: i as u32,
                x_offset: x_pos,
                y_offset: 0.0,
                x_advance: metrics.width / text.chars().count() as f32,
                y_advance: 0.0,
            });
            x_pos += metrics.width / text.chars().count() as f32;
        }

        Ok(ShapedText {
            glyphs,
            width: metrics.width,
            height: metrics.height,
            baseline: metrics.ascent,
        })
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Text layout engine for multi-line text
pub struct TextLayout {
    font_manager: Arc<FontManager>,
}

impl TextLayout {
    pub fn new(font_manager: Arc<FontManager>) -> Self {
        Self { font_manager }
    }

    pub fn layout_text(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f32>,
    ) -> Result<Vec<ShapedText>> {
        if let Some(max_width) = max_width {
            self.layout_multiline(text, style, max_width)
        } else {
            Ok(vec![self.font_manager.shape_text(text, style)?])
        }
    }

    fn layout_multiline(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: f32,
    ) -> Result<Vec<ShapedText>> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0.0;

        for word in text.split_whitespace() {
            let word_metrics = self.font_manager.measure_text(word, style)?;

            if current_width + word_metrics.width > max_width && !current_line.is_empty() {
                // Start new line
                lines.push(self.font_manager.shape_text(&current_line, style)?);
                current_line.clear();
                current_width = 0.0;
            }

            if !current_line.is_empty() {
                current_line.push(' ');
                current_width += word_metrics.width / word.len() as f32; // Space width approximation
            }

            current_line.push_str(word);
            current_width += word_metrics.width;
        }

        if !current_line.is_empty() {
            lines.push(self.font_manager.shape_text(&current_line, style)?);
        }

        Ok(lines)
    }
}

/// Text rendering cache for performance
pub struct TextCache {
    cache: Arc<RwLock<HashMap<String, ShapedText>>>,
}

impl TextCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_or_shape(
        &self,
        text: &str,
        style: &TextStyle,
        font_manager: &FontManager,
    ) -> Result<ShapedText> {
        let cache_key = format!("{}:{}:{}", text, style.font_family, style.font_size);

        // Check cache
        {
            let cache = self.cache.read();
            if let Some(shaped) = cache.get(&cache_key) {
                return Ok(shaped.clone());
            }
        }

        // Shape and cache
        let shaped = font_manager.shape_text(text, style)?;
        self.cache.write().insert(cache_key, shaped.clone());

        Ok(shaped)
    }

    pub fn clear(&self) {
        self.cache.write().clear();
    }
}

impl Default for TextCache {
    fn default() -> Self {
        Self::new()
    }
}
```

## File: `./src/platform/mod.rs`
```rs

```

## File: `./src/layout/mod.rs`
```rs
mod advanced;
pub mod constraints;

pub use crate::layout::advanced::{
    AlignContent, AlignItems, FlexDirection, FlexLayout, FlexWrap, GridLayout, GridTrack,
    JustifyContent, LayoutEngine,
};
pub use constraints::{Alignment, Constraints, EdgeInsets, Size};
```

## File: `./src/layout/constraints.rs`
```rs
//! Layout constraints system for OxideUI
//!
//! Implements a constraint-based layout model similar to Flutter's BoxConstraints.
//! Parent passes constraints down, child measures itself, returns size up.

/// Size in logical pixels
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub const fn zero() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }

    pub const fn infinite() -> Self {
        Self {
            width: f32::INFINITY,
            height: f32::INFINITY,
        }
    }

    pub fn constrain(&self, constraints: &Constraints) -> Self {
        Self {
            width: self
                .width
                .clamp(constraints.min_width, constraints.max_width),
            height: self
                .height
                .clamp(constraints.min_height, constraints.max_height),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::zero()
    }
}

/// Layout constraints - defines the range of acceptable sizes
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Constraints {
    /// Create tight constraints (fixed size)
    pub const fn tight(size: Size) -> Self {
        Self {
            min_width: size.width,
            max_width: size.width,
            min_height: size.height,
            max_height: size.height,
        }
    }

    /// Create loose constraints (0 to specified max)
    pub const fn loose(max_size: Size) -> Self {
        Self {
            min_width: 0.0,
            max_width: max_size.width,
            min_height: 0.0,
            max_height: max_size.height,
        }
    }

    /// Create unbounded constraints
    pub const fn unbounded() -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }

    /// Create unconstrained constraints (for root)
    pub const fn unconstrained() -> Self {
        Self::unbounded()
    }

    /// Create constraints with specific bounds
    pub const fn new(min_width: f32, max_width: f32, min_height: f32, max_height: f32) -> Self {
        Self {
            min_width,
            max_width,
            min_height,
            max_height,
        }
    }

    /// Check if width is bounded
    pub fn has_bounded_width(&self) -> bool {
        self.max_width.is_finite()
    }

    /// Check if height is bounded
    pub fn has_bounded_height(&self) -> bool {
        self.max_height.is_finite()
    }

    /// Check if constraints are tight (fixed size)
    pub fn is_tight(&self) -> bool {
        self.min_width == self.max_width && self.min_height == self.max_height
    }

    /// Get the biggest size that satisfies these constraints
    pub fn biggest(&self) -> Size {
        Size::new(
            if self.max_width.is_finite() {
                self.max_width
            } else {
                0.0
            },
            if self.max_height.is_finite() {
                self.max_height
            } else {
                0.0
            },
        )
    }

    /// Get the smallest size that satisfies these constraints
    pub fn smallest(&self) -> Size {
        Size::new(self.min_width, self.min_height)
    }

    /// Constrain the given size to fit within these constraints
    pub fn constrain(&self, size: Size) -> Size {
        Size::new(
            size.width.clamp(self.min_width, self.max_width),
            size.height.clamp(self.min_height, self.max_height),
        )
    }

    /// Create new constraints with width constrained
    pub fn constrain_width(&self, width: f32) -> Self {
        Self {
            min_width: width.max(self.min_width).min(self.max_width),
            max_width: width.max(self.min_width).min(self.max_width),
            ..*self
        }
    }

    /// Create new constraints with height constrained
    pub fn constrain_height(&self, height: f32) -> Self {
        Self {
            min_height: height.max(self.min_height).min(self.max_height),
            max_height: height.max(self.min_height).min(self.max_height),
            ..*self
        }
    }

    /// Deflate constraints by the given amount
    pub fn deflate(&self, amount: EdgeInsets) -> Self {
        let horizontal = amount.left + amount.right;
        let vertical = amount.top + amount.bottom;

        Self {
            min_width: (self.min_width - horizontal).max(0.0),
            max_width: (self.max_width - horizontal).max(0.0),
            min_height: (self.min_height - vertical).max(0.0),
            max_height: (self.max_height - vertical).max(0.0),
        }
    }

    /// Loosen the constraints (allow 0 as minimum)
    pub fn loosen(&self) -> Self {
        Self {
            min_width: 0.0,
            min_height: 0.0,
            ..*self
        }
    }

    /// Tighten to a specific size
    pub fn tighten(&self, size: Size) -> Self {
        Self::tight(size)
    }
}

impl Default for Constraints {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Edge insets for padding/margin
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeInsets {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl EdgeInsets {
    pub const fn all(value: f32) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }

    pub const fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        }
    }

    pub const fn only(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub const fn zero() -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }

    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl Default for EdgeInsets {
    fn default() -> Self {
        Self::zero()
    }
}

/// Alignment options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Alignment {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Alignment {
    pub fn align(&self, size: Size, container_size: Size) -> (f32, f32) {
        let x = match self {
            Alignment::TopLeft | Alignment::CenterLeft | Alignment::BottomLeft => 0.0,
            Alignment::TopCenter | Alignment::Center | Alignment::BottomCenter => {
                (container_size.width - size.width) / 2.0
            }
            Alignment::TopRight | Alignment::CenterRight | Alignment::BottomRight => {
                container_size.width - size.width
            }
        };

        let y = match self {
            Alignment::TopLeft | Alignment::TopCenter | Alignment::TopRight => 0.0,
            Alignment::CenterLeft | Alignment::Center | Alignment::CenterRight => {
                (container_size.height - size.height) / 2.0
            }
            Alignment::BottomLeft | Alignment::BottomCenter | Alignment::BottomRight => {
                container_size.height - size.height
            }
        };

        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraints_tight() {
        let constraints = Constraints::tight(Size::new(100.0, 200.0));
        assert_eq!(constraints.min_width, 100.0);
        assert_eq!(constraints.max_width, 100.0);
        assert_eq!(constraints.min_height, 200.0);
        assert_eq!(constraints.max_height, 200.0);
        assert!(constraints.is_tight());
    }

    #[test]
    fn test_constraints_loose() {
        let constraints = Constraints::loose(Size::new(100.0, 200.0));
        assert_eq!(constraints.min_width, 0.0);
        assert_eq!(constraints.max_width, 100.0);
        assert_eq!(constraints.min_height, 0.0);
        assert_eq!(constraints.max_height, 200.0);
        assert!(!constraints.is_tight());
    }

    #[test]
    fn test_size_constrain() {
        let constraints = Constraints::new(10.0, 100.0, 20.0, 200.0);
        let size = Size::new(150.0, 250.0);
        let constrained = constraints.constrain(size);
        assert_eq!(constrained.width, 100.0);
        assert_eq!(constrained.height, 200.0);
    }
}
```

## File: `./src/layout/advanced.rs`
```rs
// File: ./oxideui/src/layout/advanced.rs
//! Advanced layout engine with flexbox, grid, and absolute positioning

use crate::layout::constraints::{Constraints, Size, EdgeInsets, Alignment};
use std::collections::HashMap;

/// Layout node in the layout tree
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: u64,
    pub constraints: Constraints,
    pub size: Size,
    pub position: (f32, f32),
    pub children: Vec<LayoutNode>,
    pub layout_type: LayoutType,
}

/// Layout algorithm type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutType {
    Flex,
    Grid,
    Absolute,
    Stack,
}

/// Flexbox layout properties
#[derive(Debug, Clone, Copy)]
pub struct FlexLayout {
    pub direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub wrap: FlexWrap,
    pub gap: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

/// Flex item properties
#[derive(Debug, Clone, Copy)]
pub struct FlexItem {
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<f32>,
    pub align_self: Option<AlignItems>,
}

impl Default for FlexItem {
    fn default() -> Self {
        Self {
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            align_self: None,
        }
    }
}

/// Grid layout properties
#[derive(Debug, Clone)]
pub struct GridLayout {
    pub columns: Vec<GridTrack>,
    pub rows: Vec<GridTrack>,
    pub column_gap: f32,
    pub row_gap: f32,
    pub auto_flow: GridAutoFlow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridTrack {
    Fixed(f32),
    Flex(f32),
    Auto,
    MinContent,
    MaxContent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridAutoFlow {
    Row,
    Column,
    RowDense,
    ColumnDense,
}

/// Grid item placement
#[derive(Debug, Clone, Copy)]
pub struct GridItem {
    pub column_start: Option<usize>,
    pub column_end: Option<usize>,
    pub row_start: Option<usize>,
    pub row_end: Option<usize>,
}

/// Layout engine
pub struct LayoutEngine {
    cache: HashMap<u64, LayoutNode>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Layout a node and its children
    pub fn layout(&mut self, node: &mut LayoutNode) {
        match node.layout_type {
            LayoutType::Flex => self.layout_flex(node),
            LayoutType::Grid => self.layout_grid(node),
            LayoutType::Absolute => self.layout_absolute(node),
            LayoutType::Stack => self.layout_stack(node),
        }
    }

    /// Flexbox layout algorithm
    fn layout_flex(&self, node: &mut LayoutNode) {
        let is_row = true;
        let mut position = 0.0;

        for child in &mut node.children {
            let child_size = child.constraints.biggest();

            if is_row {
                child.position = (position, 0.0);
                position += child_size.width;
            } else {
                child.position = (0.0, position);
                position += child_size.height;
            }

            child.size = child_size;
        }

        node.size = if is_row {
            Size::new(position, node.constraints.max_height)
        } else {
            Size::new(node.constraints.max_width, position)
        };
    }

    /// Grid layout algorithm - FIXED TYPE ANNOTATIONS
    fn layout_grid(&self, node: &mut LayoutNode) {
        let columns = 3;
        let gap = 10.0;

        let available_width = node.constraints.max_width - (gap * (columns - 1) as f32);
        let cell_width = available_width / columns as f32;
        let cell_height = 100.0;

        for (i, child) in node.children.iter_mut().enumerate() {
            let col = i % columns;
            let row = i / columns;

            child.position = (
                col as f32 * (cell_width + gap),
                row as f32 * (cell_height + gap),
            );
            child.size = Size::new(cell_width, cell_height);
        }

        let rows = (node.children.len() + columns - 1) / columns;
        node.size = Size::new(
            node.constraints.max_width,
            rows as f32 * cell_height + (rows - 1) as f32 * gap,
        );
    }

    fn layout_absolute(&self, node: &mut LayoutNode) {
        for child in &mut node.children {
            child.size = child.constraints.biggest();
        }
        node.size = node.constraints.biggest();
    }

    fn layout_stack(&self, node: &mut LayoutNode) {
        let mut max_width: f32 = 0.0;  // FIX: Explicit type annotation
        let mut max_height: f32 = 0.0; // FIX: Explicit type annotation

        for child in &mut node.children {
            child.position = (0.0, 0.0);
            child.size = child.constraints.biggest();

            max_width = max_width.max(child.size.width);
            max_height = max_height.max(child.size.height);
        }

        node.size = Size::new(max_width, max_height);
    }

    pub fn measure_intrinsic(&self, node: &LayoutNode) -> Size {
        match node.layout_type {
            LayoutType::Flex => self.measure_flex_intrinsic(node),
            LayoutType::Grid => self.measure_grid_intrinsic(node),
            _ => node.constraints.smallest(),
        }
    }

    fn measure_flex_intrinsic(&self, node: &LayoutNode) -> Size {
        let is_row = true;
        let mut total_width = 0.0;
        let mut total_height: f32 = 0.0; // FIX: Explicit type annotation

        for child in &node.children {
            let child_size = self.measure_intrinsic(child);

            if is_row {
                total_width += child_size.width;
                total_height = total_height.max(child_size.height);
            } else {
                total_width = total_width.max(child_size.width);
                total_height += child_size.height;
            }
        }

        Size::new(total_width, total_height)
    }

    fn measure_grid_intrinsic(&self, node: &LayoutNode) -> Size {
        let columns = 3;
        let rows = (node.children.len() + columns - 1) / columns;

        Size::new(
            300.0 * columns as f32,
            100.0 * rows as f32,
        )
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LayoutSolver {
    variables: HashMap<String, f32>,
}

impl LayoutSolver {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn solve(&mut self, constraints: &[LayoutConstraint]) -> bool {
        for constraint in constraints {
            match constraint {
                LayoutConstraint::Equal(var, value) => {
                    self.variables.insert(var.clone(), *value);
                }
                LayoutConstraint::GreaterThan(var, value) => {
                    let current = self.variables.get(var).copied().unwrap_or(0.0);
                    if current < *value {
                        self.variables.insert(var.clone(), *value);
                    }
                }
                LayoutConstraint::LessThan(var, value) => {
                    let current = self.variables.get(var).copied().unwrap_or(f32::INFINITY);
                    if current > *value {
                        self.variables.insert(var.clone(), *value);
                    }
                }
            }
        }
        true
    }

    pub fn get_value(&self, var: &str) -> Option<f32> {
        self.variables.get(var).copied()
    }
}

#[derive(Debug, Clone)]
pub enum LayoutConstraint {
    Equal(String, f32),
    GreaterThan(String, f32),
    LessThan(String, f32),
}

impl Default for LayoutSolver {
    fn default() -> Self {
        Self::new()
    }
}
```

## File: `./src/core/element.rs`
```rs
//! Element tree implementation - the retained-mode runtime layer
//!
//! Elements are the mutable runtime instances that correspond to widgets.
//! They hold state, manage relationships, and track what needs rebuilding.

use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

use crate::core::render_object::RenderObject;
use crate::core::widget::{Widget, WidgetKey};
use crate::layout::constraints::{Constraints, Size};

/// Unique identifier for elements in the tree
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ElementId(u64);

impl ElementId {
    pub fn new(id: u64) -> Self {
        ElementId(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// An element in the element tree - the runtime representation of a widget
pub struct Element {
    /// Unique identifier for this element
    pub id: ElementId,

    /// The type of widget this element corresponds to
    pub widget_type: TypeId,

    /// Parent element ID (None for root)
    pub parent: Option<ElementId>,

    /// Child element IDs
    pub children: Vec<ElementId>,

    /// Widget state (for stateful widgets)
    pub state: Option<Box<dyn Any + Send + Sync>>,

    /// Position in parent's child list
    pub slot_index: usize,

    /// Optional key for reconciliation
    pub key: Option<WidgetKey>,

    /// Whether this element needs rebuilding
    pub dirty: bool,

    /// The render object produced by this element
    pub render_object: Option<RenderObject>,

    /// Layout constraints for this element
    pub constraints: Constraints,

    /// Computed size after layout
    pub size: Size,
}

/// The element tree - manages all elements and their relationships
pub struct ElementTree {
    /// All elements indexed by ID
    elements: HashMap<ElementId, Element>,

    /// Root element ID
    root: Option<ElementId>,

    /// Next available element ID
    next_id: u64,
}

impl ElementTree {
    /// Create a new empty element tree
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            root: None,
            next_id: 1,
        }
    }

    /// Create a new element
    pub fn create_element(
        &mut self,
        widget: &dyn Widget,
        parent: Option<ElementId>,
        slot_index: usize,
    ) -> ElementId {
        let id = ElementId(self.next_id);
        self.next_id += 1;

        let element = Element {
            id,
            widget_type: widget.type_id(),
            parent,
            children: Vec::new(),
            state: None,
            slot_index,
            key: widget.key(),
            dirty: true,
            render_object: None,
            constraints: Constraints::default(),
            size: Size::default(),
        };

        self.elements.insert(id, element);

        // Add to parent's children if parent exists
        if let Some(parent_id) = parent {
            if let Some(parent_element) = self.elements.get_mut(&parent_id) {
                parent_element.children.push(id);
            }
        }

        // Set as root if this is the first element
        if self.root.is_none() {
            self.root = Some(id);
        }

        id
    }

    /// Get an element by ID
    pub fn get(&self, id: ElementId) -> Option<&Element> {
        self.elements.get(&id)
    }

    /// Get a mutable reference to an element
    pub fn get_mut(&mut self, id: ElementId) -> Option<&mut Element> {
        self.elements.get_mut(&id)
    }

    /// Get the root element ID
    pub fn root(&self) -> Option<ElementId> {
        self.root
    }

    /// Set the root element
    pub fn set_root(&mut self, id: ElementId) {
        self.root = Some(id);
    }

    /// Mark an element as dirty (needs rebuilding)
    pub fn mark_dirty(&mut self, id: ElementId) {
        if let Some(element) = self.elements.get_mut(&id) {
            element.dirty = true;

            // Propagate dirty flag up the tree
            if let Some(parent_id) = element.parent {
                self.mark_dirty(parent_id);
            }
        }
    }

    /// Remove an element and all its children
    pub fn remove_element(&mut self, id: ElementId) {
        // FIX: Collect children IDs FIRST before removing anything
        let children = if let Some(element) = self.elements.get(&id) {
            element.children.clone()
        } else {
            Vec::new()
        };

        // Remove all children recursively FIRST
        for child_id in children {
            self.remove_element(child_id);
        }

        // Now remove the element itself
        if let Some(element) = self.elements.remove(&id) {
            // Remove from parent's child list
            if let Some(parent_id) = element.parent {
                if let Some(parent) = self.elements.get_mut(&parent_id) {
                    parent.children.retain(|&child| child != id);
                }
            }

            // Clear root if this was the root
            if self.root == Some(id) {
                self.root = None;
            }
        }
    }

    /// Get parent of an element
    pub fn get_parent(&self, id: ElementId) -> Option<ElementId> {
        self.elements.get(&id).and_then(|e| e.parent)
    }

    /// Get children of an element
    pub fn get_children(&self, id: ElementId) -> Vec<ElementId> {
        self.elements
            .get(&id)
            .map(|e| e.children.clone())
            .unwrap_or_default()
    }

    /// Find an ancestor element of a specific widget type
    pub fn find_ancestor(&self, id: ElementId, widget_type: TypeId) -> Option<ElementId> {
        let mut current = Some(id);

        while let Some(element_id) = current {
            if let Some(element) = self.elements.get(&element_id) {
                if element.widget_type == widget_type {
                    return Some(element_id);
                }
                current = element.parent;
            } else {
                break;
            }
        }

        None
    }

    /// Collect all dirty elements
    pub fn collect_dirty(&self) -> Vec<ElementId> {
        self.elements
            .values()
            .filter(|e| e.dirty)
            .map(|e| e.id)
            .collect()
    }

    /// Clear all dirty flags
    pub fn clear_dirty(&mut self) {
        for element in self.elements.values_mut() {
            element.dirty = false;
        }
    }

    /// Get the number of elements in the tree
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Check if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl Default for ElementTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe wrapper around ElementTree
pub type SharedElementTree = Arc<RwLock<ElementTree>>;

/// Create a new shared element tree
pub fn new_shared_element_tree() -> SharedElementTree {
    Arc::new(RwLock::new(ElementTree::new()))
}
```

## File: `./src/core/event.rs`
```rs
use crate::core::element::ElementId;
use crate::core::render_object::Point;
use winit::event::MouseButton as WinitMouseButton;
use winit::keyboard::{KeyCode, ModifiersState};

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

impl From<WinitMouseButton> for MouseButton {
    fn from(button: WinitMouseButton) -> Self {
        match button {
            WinitMouseButton::Left => MouseButton::Left,
            WinitMouseButton::Right => MouseButton::Right,
            WinitMouseButton::Middle => MouseButton::Middle,
            WinitMouseButton::Back => MouseButton::Back,
            WinitMouseButton::Forward => MouseButton::Forward,
            WinitMouseButton::Other(n) => MouseButton::Other(n),
        }
    }
}

/// 2D vector for mouse deltas
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Vector2 = Vector2::new(0.0, 0.0);
}

/// Keyboard modifiers state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl From<ModifiersState> for Modifiers {
    fn from(state: ModifiersState) -> Self {
        Self {
            shift: state.shift_key(),
            ctrl: state.control_key(),
            alt: state.alt_key(),
            meta: state.super_key(),
        }
    }
}

/// UI event types
/// FIXED: Removed Clone and PartialEq derives because Custom variant contains Box<dyn Any>
#[derive(Debug)]
pub enum UiEvent {
    PointerDown {
        id: u64,
        position: Point,
        button: MouseButton,
    },
    PointerUp {
        id: u64,
        position: Point,
        button: MouseButton,
    },
    PointerMove {
        id: u64,
        position: Point,
        delta: Vector2,
    },
    Scroll {
        position: Point,
        delta: Vector2,
    },
    KeyDown {
        key: KeyCode,
        modifiers: Modifiers,
        repeat: bool,
    },
    KeyUp {
        key: KeyCode,
        modifiers: Modifiers,
    },
    TextInput {
        character: char,
    },
    Focus,
    Blur,
    Custom {
        name: String,
        data: Box<dyn std::any::Any + Send + Sync>,
    },
}

impl UiEvent {
    pub fn position(&self) -> Option<Point> {
        match self {
            UiEvent::PointerDown { position, .. }
            | UiEvent::PointerUp { position, .. }
            | UiEvent::PointerMove { position, .. }
            | UiEvent::Scroll { position, .. } => Some(*position),
            _ => None,
        }
    }

    pub fn is_pointer_event(&self) -> bool {
        matches!(
            self,
            UiEvent::PointerDown { .. }
                | UiEvent::PointerUp { .. }
                | UiEvent::PointerMove { .. }
                | UiEvent::Scroll { .. }
        )
    }

    pub fn is_keyboard_event(&self) -> bool {
        matches!(
            self,
            UiEvent::KeyDown { .. } | UiEvent::KeyUp { .. } | UiEvent::TextInput { .. }
        )
    }
}

/// Event propagation phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPhase {
    Capturing,
    AtTarget,
    Bubbling,
}

/// Event context passed to event handlers
pub struct EventContext {
    pub target: ElementId,
    pub current_target: ElementId,
    pub phase: EventPhase,
    pub handled: bool,
    pub default_prevented: bool,
}

impl EventContext {
    pub fn new(target: ElementId, current_target: ElementId, phase: EventPhase) -> Self {
        Self {
            target,
            current_target,
            phase,
            handled: false,
            default_prevented: false,
        }
    }

    pub fn stop_propagation(&mut self) {
        self.handled = true;
    }

    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }

    pub fn is_at_target(&self) -> bool {
        self.target == self.current_target
    }
}

/// Event path through the element tree
pub struct EventPath {
    pub capturing: Vec<ElementId>,
    pub target: ElementId,
    pub bubbling: Vec<ElementId>,
}

impl EventPath {
    pub fn new(target: ElementId) -> Self {
        Self {
            capturing: Vec::new(),
            target,
            bubbling: Vec::new(),
        }
    }
}

/// Event handler result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    Unhandled,
    Handled,
    Stopped,
}

impl EventResult {
    pub fn should_stop(&self) -> bool {
        matches!(self, EventResult::Stopped)
    }

    pub fn is_handled(&self) -> bool {
        matches!(self, EventResult::Handled | EventResult::Stopped)
    }
}
```

## File: `./src/core/mod.rs`
```rs
pub mod context;
pub mod element;
pub mod event;
pub mod event_dispatcher;
mod event_system;
pub mod reconcile;
pub mod render_object;
pub(crate) mod state_driven;
pub mod widget;

pub use crate::core::event_system::{
    AccessibilityManager, AccessibilityRole, FocusManager, GestureRecognizer, GestureType,
    InputMethodManager,
};
pub use crate::core::state_driven::{
    DerivedState, EffectRunner, ReactiveState, StateBatch, StateChange, StateToken, StateTracker,
};
pub use context::{BuildContext, Theme};
pub use element::{Element, ElementId, ElementTree, SharedElementTree, new_shared_element_tree};
pub use event::{
    EventContext, EventPath, EventPhase, EventResult, Modifiers, MouseButton, UiEvent, Vector2,
};
pub use event_dispatcher::EventDispatcher;
pub use reconcile::Reconciler;
pub use render_object::{Color, Matrix, Paint, Point, Rect, RenderObject, TextStyle};
pub use widget::{StatefulWidget, StatelessWidget, Widget, WidgetKey, WidgetNode, WidgetState};
```

## File: `./src/core/context.rs`
```rs
//! Build context - safe access to element tree during widget building

use std::any::TypeId;
use std::sync::Arc;
use crate::Color;
use crate::core::element::{ElementId, SharedElementTree};
use crate::layout::constraints::Constraints;
use crate::theming::ThemeConfig;

/// Theme data with Radix UI inspired colors
#[derive(Clone, Debug)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub card: Color,
    pub card_foreground: Color,
    pub popover: Color,
    pub popover_foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub destructive: Color,
    pub destructive_foreground: Color,
    pub border: Color,
    pub input: Color,
    pub ring: Color,
    pub sidebar: Color,
    pub sidebar_foreground: Color,
    pub sidebar_primary: Color,
    pub sidebar_primary_foreground: Color,
    pub sidebar_accent: Color,
    pub sidebar_accent_foreground: Color,
    pub sidebar_border: Color,
    pub sidebar_ring: Color,
    pub font_sans: String,
    pub font_mono: String,
    pub radius: f32,
    pub is_dark: bool,
    pub shadow_x: f32,
    pub shadow_y: f32,
    pub shadow_blur: f32,
    pub shadow_spread: f32,
    pub shadow_opacity: f32,
    pub chart_1: Color,
    pub chart_2: Color,
    pub chart_3: Color,
    pub chart_4: Color,
    pub chart_5: Color,
}

impl Theme {
    pub fn from_config(config: &ThemeConfig, is_dark: bool) -> Self {
        let colors = if is_dark { &config.dark } else { &config.light };

        Self {
            background: colors.get_color("background"),
            foreground: colors.get_color("foreground"),
            card: colors.get_color("card"),
            card_foreground: colors.get_color("card_foreground"),
            popover: colors.get_color("popover"),
            popover_foreground: colors.get_color("popover_foreground"),
            primary: colors.get_color("primary"),
            primary_foreground: colors.get_color("primary_foreground"),
            secondary: colors.get_color("secondary"),
            secondary_foreground: colors.get_color("secondary_foreground"),
            muted: colors.get_color("muted"),
            muted_foreground: colors.get_color("muted_foreground"),
            accent: colors.get_color("accent"),
            accent_foreground: colors.get_color("accent_foreground"),
            destructive: colors.get_color("destructive"),
            destructive_foreground: colors.get_color("destructive_foreground"),
            border: colors.get_color("border"),
            input: colors.get_color("input"),
            ring: colors.get_color("ring"),
            sidebar: colors.get_color("sidebar"),
            sidebar_foreground: colors.get_color("sidebar_foreground"),
            sidebar_primary: colors.get_color("sidebar_primary"),
            sidebar_primary_foreground: colors.get_color("sidebar_primary_foreground"),
            sidebar_accent: colors.get_color("sidebar_accent"),
            sidebar_accent_foreground: colors.get_color("sidebar_accent_foreground"),
            sidebar_border: colors.get_color("sidebar_border"),
            sidebar_ring: colors.get_color("sidebar_ring"),
            font_sans: config.font_sans.clone(),
            font_mono: config.font_mono.clone(),
            radius: config.radius,
            is_dark,
            shadow_x: colors.shadow_x,
            shadow_y: colors.shadow_y,
            shadow_blur: colors.shadow_blur,
            shadow_spread: colors.shadow_spread,
            shadow_opacity: colors.shadow_opacity,
            chart_1: colors.get_color("chart_1"),
            chart_2: colors.get_color("chart_2"),
            chart_3: colors.get_color("chart_3"),
            chart_4: colors.get_color("chart_4"),
            chart_5: colors.get_color("chart_5"),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        let config = ThemeConfig::default();
        Theme::from_config(&config, false)
    }
}

/// Theme provider trait for widgets
pub trait ThemeProvider {
    fn theme(&self) -> &Theme;
    fn is_dark(&self) -> bool {
        self.theme().is_dark
    }
}

/// Build context provides safe access to the element tree during widget building
pub struct BuildContext {
    /// The element being built
    pub element_id: ElementId,

    /// Reference to the element tree
    pub element_tree: SharedElementTree,

    /// Layout constraints for this element
    pub constraints: Constraints,

    /// Current theme
    pub theme: Arc<Theme>,
}

impl BuildContext {
    /// Create a new build context
    pub fn new(
        element_id: ElementId,
        element_tree: SharedElementTree,
        constraints: Constraints,
        theme: Arc<Theme>,
    ) -> Self {
        Self {
            element_id,
            element_tree,
            constraints,
            theme,
        }
    }

    /// Get the parent element ID
    pub fn parent(&self) -> Option<ElementId> {
        self.element_tree.read().get_parent(self.element_id)
    }

    /// Get the children of this element
    pub fn children(&self) -> Vec<ElementId> {
        self.element_tree.read().get_children(self.element_id)
    }

    /// Find an ancestor element of a specific widget type
    pub fn find_ancestor<W: 'static>(&self) -> Option<ElementId> {
        self.element_tree
            .read()
            .find_ancestor(self.element_id, TypeId::of::<W>())
    }

    /// Mark the current element as dirty (needs rebuilding)
    pub fn mark_dirty(&self) {
        self.element_tree.write().mark_dirty(self.element_id);
    }

    /// Create a child context
    pub fn child_context(&self, child_id: ElementId, constraints: Constraints) -> BuildContext {
        BuildContext {
            element_id: child_id,
            element_tree: self.element_tree.clone(),
            constraints,
            theme: self.theme.clone(),
        }
    }
}

impl ThemeProvider for BuildContext {
    fn theme(&self) -> &Theme {
        &self.theme
    }
}

impl Clone for BuildContext {
    fn clone(&self) -> Self {
        Self {
            element_id: self.element_id,
            element_tree: self.element_tree.clone(),
            constraints: self.constraints,
            theme: self.theme.clone(),
        }
    }
}
```

## File: `./src/core/reconcile.rs`
```rs
//! Reconciliation engine - React-style diffing algorithm
//!
//! The reconciler compares old and new widget trees and efficiently updates
//! the element tree by reusing elements where possible.

use std::any::TypeId;

use crate::core::element::{ElementId, ElementTree};
use crate::core::widget::{Widget, WidgetKey};
use std::sync::Arc;

/// Reconciler handles efficient updates to the element tree
pub struct Reconciler;

impl Reconciler {
    /// Reconcile a widget against the element tree
    ///
    /// This is the entry point for the diffing algorithm. It compares the new widget
    /// with what's currently in the element tree and updates accordingly.
    pub fn reconcile(
        element_tree: &mut ElementTree,
        new_widget: Box<dyn Widget>,
        parent: Option<ElementId>,
        slot_index: usize,
        theme: Arc<crate::core::context::Theme>,
    ) -> ElementId {
        // Try to find an existing element we can reuse
        let existing = Self::find_reusable_element(element_tree, &*new_widget, parent, slot_index);

        match existing {
            Some(element_id) => {
                // Reuse existing element
                Self::update_element(element_tree, element_id, new_widget);
                element_id
            }
            None => {
                // Create new element
                Self::mount_element(element_tree, new_widget, parent, slot_index)
            }
        }
    }

    /// Check if two widgets can be updated (same type and key)
    fn can_update(old_type: TypeId, old_key: &Option<WidgetKey>, new_widget: &dyn Widget) -> bool {
        let type_matches = old_type == new_widget.type_id();
        let key_matches = match (&old_key, new_widget.key()) {
            (Some(old), Some(new)) => old == &new,
            (None, None) => true,
            _ => false,
        };

        type_matches && key_matches
    }

    /// Find an existing element that can be reused for this widget
    fn find_reusable_element(
        element_tree: &ElementTree,
        new_widget: &dyn Widget,
        parent: Option<ElementId>,
        slot_index: usize,
    ) -> Option<ElementId> {
        // If there's a parent, look through its children at the slot index
        if let Some(parent_id) = parent {
            if let Some(parent_element) = element_tree.get(parent_id) {
                if slot_index < parent_element.children.len() {
                    let child_id = parent_element.children[slot_index];
                    if let Some(child_element) = element_tree.get(child_id) {
                        if Self::can_update(
                            child_element.widget_type,
                            &child_element.key,
                            new_widget,
                        ) {
                            return Some(child_id);
                        }
                    }
                }
            }
        }

        None
    }

    /// Update an existing element with a new widget
    fn update_element(
        element_tree: &mut ElementTree,
        element_id: ElementId,
        new_widget: Box<dyn Widget>,
    ) {
        if let Some(element) = element_tree.get_mut(element_id) {
            element.dirty = true;
            element.widget_type = new_widget.type_id();
            // State is preserved automatically
        }
    }

    /// Mount a new element for a widget
    fn mount_element(
        element_tree: &mut ElementTree,
        widget: Box<dyn Widget>,
        parent: Option<ElementId>,
        slot_index: usize,
    ) -> ElementId {
        let element_id = element_tree.create_element(&*widget, parent, slot_index);

        // Initialize state for stateful widgets
        // This would require downcasting to StatefulWidget trait
        // For now, state initialization happens in the widget's build method

        element_id
    }

    /// Unmount an element and its children
    pub fn unmount_element(element_tree: &mut ElementTree, element_id: ElementId) {
        // Recursively unmount children first
        let children = element_tree.get_children(element_id);
        for child_id in children {
            Self::unmount_element(element_tree, child_id);
        }

        // Call will_unmount lifecycle if the element has state
        // (This would require storing the widget alongside the element)

        // Remove the element
        element_tree.remove_element(element_id);
    }

    /// Reconcile a list of children
    pub fn reconcile_children(
        element_tree: &mut ElementTree,
        parent_id: ElementId,
        new_children: Vec<Box<dyn Widget>>,
        theme: Arc<crate::core::context::Theme>,
    ) {
        // Get current children
        let old_children = element_tree.get_children(parent_id);

        // Reconcile each new child
        let mut new_child_ids = Vec::new();
        for (index, child_widget) in new_children.into_iter().enumerate() {
            let child_id = Self::reconcile(
                element_tree,
                child_widget,
                Some(parent_id),
                index,
                theme.clone(),
            );
            new_child_ids.push(child_id);
        }

        // Remove old children that are no longer present
        for old_child_id in old_children {
            if !new_child_ids.contains(&old_child_id) {
                Self::unmount_element(element_tree, old_child_id);
            }
        }

        // Update parent's children list
        if let Some(parent) = element_tree.get_mut(parent_id) {
            parent.children = new_child_ids;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::context::BuildContext;
    use crate::core::widget::{Widget, WidgetKey, WidgetNode};
    use std::any::Any;

    #[derive(Clone)]
    struct TestWidget {
        key: Option<WidgetKey>,
    }

    impl Widget for TestWidget {
        fn build(&self, _ctx: &BuildContext) -> WidgetNode {
            WidgetNode::None
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

    #[test]
    fn test_can_update() {
        let widget1 = TestWidget { key: None };
        let widget2 = TestWidget {
            key: Some(WidgetKey::string("test")),
        };

        // Same type, no keys - can update
        assert!(Reconciler::can_update(
            TypeId::of::<TestWidget>(),
            &None,
            &widget1
        ));

        // Same type, different keys - cannot update
        assert!(!Reconciler::can_update(
            TypeId::of::<TestWidget>(),
            &None,
            &widget2
        ));

        // Same type, same keys - can update
        assert!(Reconciler::can_update(
            TypeId::of::<TestWidget>(),
            &Some(WidgetKey::string("test")),
            &widget2
        ));
    }
}
```

## File: `./src/core/widget.rs`
```rs
//! Core widget trait definitions and types for OxideUI
//!
//! This module defines the fundamental widget system following the three-tree architecture:
//! - Widget Tree (immutable, declarative)
//! - Element Tree (mutable, runtime state)
//! - Render Tree (GPU primitives)

use std::any::{Any, TypeId};
use std::fmt;

use crate::core::context::BuildContext;
use crate::core::event::{EventContext, EventResult, UiEvent};
use crate::core::render_object::RenderObject;

/// Unique identifier for widgets to aid in reconciliation
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WidgetKey {
    String(String),
    U64(u64),
    TypeId(TypeId),
}

impl WidgetKey {
    pub fn string(s: impl Into<String>) -> Self {
        WidgetKey::String(s.into())
    }

    pub fn u64(n: u64) -> Self {
        WidgetKey::U64(n)
    }

    pub fn type_id(t: TypeId) -> Self {
        WidgetKey::TypeId(t)
    }
}

/// The result of building a widget - can be a leaf, container, or nothing
pub enum WidgetNode {
    /// A leaf widget that directly produces a render object
    Leaf(RenderObject),
    /// A container widget that has children
    Container { children: Vec<Box<dyn Widget>> },
    /// An empty widget (renders nothing)
    None,
}

/// Core trait that all widgets must implement
///
/// Widgets are immutable, cheap to create, and describe UI intent declaratively.
/// They are rebuilt frequently and should not hold mutable state.
pub trait Widget: Send + Sync + 'static {
    /// Build this widget into a WidgetNode
    fn build(&self, ctx: &BuildContext) -> WidgetNode;

    /// Handle an event targeting this widget
    ///
    /// Return EventResult to control propagation:
    /// - Unhandled: Event continues propagating
    /// - Handled: Event was processed but continues propagating
    /// - Stopped: Event was processed and propagation stops
    fn handle_event(&self, _event: &UiEvent, _context: &mut EventContext) -> EventResult {
        EventResult::Unhandled // Default: don't handle events
    }

    /// Optional key for reconciliation (helps preserve state during updates)
    fn key(&self) -> Option<WidgetKey> {
        None
    }

    /// Get the TypeId of this widget for type checking
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    /// Upcast to Any for downcasting support
    fn as_any(&self) -> &dyn Any;

    /// Clone this widget into a Box
    fn clone_box(&self) -> Box<dyn Widget>;
}

/// Marker trait for stateless widgets (widgets without internal state)
pub trait StatelessWidget: Widget {
    /// Build the widget without any state
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode;
}

/// Trait for stateful widgets - widgets that maintain mutable state
pub trait StatefulWidget: Widget {
    /// The type of state this widget manages
    type State: WidgetState;

    /// Create the initial state for this widget
    fn create_state(&self) -> Self::State;

    /// Build the widget using the provided state
    fn build_stateful(&self, state: &Self::State, ctx: &BuildContext) -> WidgetNode;

    /// Called when the widget is first mounted to the element tree
    fn did_mount(&self, _state: &mut Self::State, _ctx: &BuildContext) {}

    /// Called when the widget is updated (parent rebuilt)
    fn did_update(&self, _prev: &Self, _state: &mut Self::State, _ctx: &BuildContext) {}

    /// Called just before the widget is removed from the tree
    fn will_unmount(&self, _state: &mut Self::State, _ctx: &BuildContext) {}
}

/// Trait that all widget state must implement
pub trait WidgetState: Send + Sync + 'static {
    /// Process an action and return whether the state changed
    fn reduce(&mut self, action: Box<dyn Any + Send>) -> bool {
        // Default implementation does nothing
        false
    }

    /// Upcast to Any for downcasting support
    fn as_any(&self) -> &dyn Any;

    /// Upcast to mutable Any for downcasting support
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Implement Display for WidgetKey for debugging
impl fmt::Display for WidgetKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidgetKey::String(s) => write!(f, "Key(\"{}\")", s),
            WidgetKey::U64(n) => write!(f, "Key({})", n),
            WidgetKey::TypeId(t) => write!(f, "Key({:?})", t),
        }
    }
}
```

## File: `./src/core/event_dispatcher.rs`
```rs
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::core::element::{ElementId, ElementTree};
use crate::core::event::{EventContext, EventPath, EventPhase, EventResult, UiEvent};
use crate::core::render_object::Point;
use crate::core::widget::Widget;

/// Event dispatcher handles routing events through the widget tree
pub struct EventDispatcher {
    /// Currently focused element
    focused_element: Option<ElementId>,

    /// Element currently under the pointer
    hovered_element: Option<ElementId>,

    /// Current pointer position
    pointer_position: Option<Point>,

    /// Widget registry - maps ElementId to Widget for event handling
    /// CRITICAL: This is needed to actually call widget.handle_event()
    widget_handlers: Arc<RwLock<HashMap<ElementId, Box<dyn Widget>>>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            focused_element: None,
            hovered_element: None,
            pointer_position: None,
            widget_handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a widget for event handling
    /// CRITICAL: Call this when creating/mounting elements
    pub fn register_widget(&mut self, element_id: ElementId, widget: Box<dyn Widget>) {
        self.widget_handlers.write().insert(element_id, widget);
    }

    /// Unregister a widget when element is unmounted
    pub fn unregister_widget(&mut self, element_id: ElementId) {
        self.widget_handlers.write().remove(&element_id);
    }

    /// Dispatch an event through the element tree
    pub fn dispatch_event(&mut self, event: &UiEvent, element_tree: &ElementTree) -> EventResult {
        // Update pointer position for pointer events
        if let Some(pos) = event.position() {
            self.pointer_position = Some(pos);
        }

        // Determine target element
        let target_id = match event {
            UiEvent::PointerDown { position, .. }
            | UiEvent::PointerUp { position, .. }
            | UiEvent::PointerMove { position, .. }
            | UiEvent::Scroll { position, .. } => {
                // Hit test to find which element was clicked/touched
                self.hit_test(*position, element_tree)
            }
            UiEvent::KeyDown { .. } | UiEvent::KeyUp { .. } | UiEvent::TextInput { .. } => {
                // Keyboard events go to focused element
                self.focused_element
            }
            UiEvent::Focus | UiEvent::Blur => {
                // Focus events target the focused element
                self.focused_element
            }
            UiEvent::Custom { .. } => {
                // Custom events go to focused element by default
                self.focused_element
            }
        };

        let Some(target_id) = target_id else {
            // No target found, event is unhandled
            return EventResult::Unhandled;
        };

        // Update hover state for pointer events
        if event.is_pointer_event() {
            self.update_hover_state(target_id, element_tree);
        }

        // Build event path (root → target → root)
        let event_path = self.build_event_path(target_id, element_tree);

        // Execute event propagation
        self.propagate_event(event, &event_path, element_tree)
    }

    /// Hit test to find which element is at the given position
    fn hit_test(&self, position: Point, element_tree: &ElementTree) -> Option<ElementId> {
        // Start from root and traverse down
        let root_id = element_tree.root()?;

        self.hit_test_recursive(position, root_id, element_tree)
    }

    /// Recursive hit testing
    fn hit_test_recursive(
        &self,
        position: Point,
        element_id: ElementId,
        element_tree: &ElementTree,
    ) -> Option<ElementId> {
        let element = element_tree.get(element_id)?;

        // Check if point is within this element's bounds
        if let Some(render_obj) = &element.render_object {
            if !self.point_in_render_object(position, render_obj) {
                return None;
            }
        }

        // Check children (front to back - last child is on top)
        for &child_id in element.children.iter().rev() {
            if let Some(hit) = self.hit_test_recursive(position, child_id, element_tree) {
                return Some(hit);
            }
        }

        // No child was hit, this element is the target
        Some(element_id)
    }

    /// Check if a point is within a render object's bounds
    fn point_in_render_object(
        &self,
        point: Point,
        render_obj: &crate::core::render_object::RenderObject,
    ) -> bool {
        use crate::core::render_object::RenderObject;

        match render_obj {
            RenderObject::Rect { rect, .. } => {
                point.x >= rect.x
                    && point.x <= rect.x + rect.width
                    && point.y >= rect.y
                    && point.y <= rect.y + rect.height
            }
            RenderObject::Text { position, .. } => {
                // Simplified: just check if point is near text position
                // TODO: Proper text bounds checking with actual text layout
                let margin = 20.0;
                (point.x - position.x).abs() < margin && (point.y - position.y).abs() < margin
            }
            RenderObject::Group { children } => {
                // Check any child
                children
                    .iter()
                    .any(|child| self.point_in_render_object(point, child))
            }
            RenderObject::Transform { child, matrix: _ } => {
                // TODO: Transform point by inverse matrix
                // For now, just check child directly
                self.point_in_render_object(point, child)
            }
            RenderObject::Clip { rect, child } => {
                // Check if point is in clip rect, then check child
                let in_clip = point.x >= rect.x
                    && point.x <= rect.x + rect.width
                    && point.y >= rect.y
                    && point.y <= rect.y + rect.height;

                in_clip && self.point_in_render_object(point, child)
            }
            RenderObject::Image { .. } => {
                // TODO: Proper image bounds
                false
            }
            RenderObject::None => false,
        }
    }

    /// Build the event propagation path (ancestors from root to target)
    fn build_event_path(&self, target_id: ElementId, element_tree: &ElementTree) -> EventPath {
        let mut path = EventPath::new(target_id);

        // Build bubbling path (target → root)
        let mut current = Some(target_id);
        while let Some(element_id) = current {
            path.bubbling.push(element_id);
            current = element_tree.get_parent(element_id);
        }

        // Build capturing path (root → target) - reverse of bubbling
        path.capturing = path.bubbling.clone();
        path.capturing.reverse();

        path
    }

    /// Propagate event through the path
    fn propagate_event(
        &self,
        event: &UiEvent,
        path: &EventPath,
        element_tree: &ElementTree,
    ) -> EventResult {
        // Phase 1: Capturing (root → target)
        for &element_id in &path.capturing {
            if element_id == path.target {
                break; // Don't process target in capturing phase
            }

            let mut context = EventContext::new(path.target, element_id, EventPhase::Capturing);

            if let Some(result) =
                self.dispatch_to_element(event, element_id, &mut context, element_tree)
            {
                if result.should_stop() {
                    return result;
                }
            }
        }

        // Phase 2: At Target
        let mut context = EventContext::new(path.target, path.target, EventPhase::AtTarget);
        if let Some(result) =
            self.dispatch_to_element(event, path.target, &mut context, element_tree)
        {
            if result.should_stop() {
                return result;
            }
        }

        // Phase 3: Bubbling (target → root)
        for &element_id in &path.bubbling {
            if element_id == path.target {
                continue; // Already processed in at-target phase
            }

            let mut context = EventContext::new(path.target, element_id, EventPhase::Bubbling);

            if let Some(result) =
                self.dispatch_to_element(event, element_id, &mut context, element_tree)
            {
                if result.should_stop() {
                    return result;
                }
            }
        }

        EventResult::Unhandled
    }

    /// Dispatch event to a specific element
    /// FIXED: Now actually calls widget.handle_event()
    fn dispatch_to_element(
        &self,
        event: &UiEvent,
        element_id: ElementId,
        context: &mut EventContext,
        _element_tree: &ElementTree,
    ) -> Option<EventResult> {
        // Get the widget for this element
        let handlers = self.widget_handlers.read();
        let widget = handlers.get(&element_id)?;

        // Call the widget's event handler
        Some(widget.handle_event(event, context))
    }

    /// Update hover state when pointer moves
    fn update_hover_state(&mut self, new_target: ElementId, element_tree: &ElementTree) {
        if self.hovered_element == Some(new_target) {
            return; // No change
        }

        // Element lost hover
        if let Some(old_target) = self.hovered_element {
            if element_tree.get(old_target).is_some() {
                // TODO: Trigger hover leave event
            }
        }

        // Element gained hover
        self.hovered_element = Some(new_target);
        // TODO: Trigger hover enter event
    }

    /// Set the focused element
    pub fn set_focus(&mut self, element_id: Option<ElementId>) {
        if self.focused_element == element_id {
            return;
        }

        // TODO: Dispatch blur event to old focused element
        // TODO: Dispatch focus event to new focused element

        self.focused_element = element_id;
    }

    /// Get currently focused element
    pub fn focused_element(&self) -> Option<ElementId> {
        self.focused_element
    }

    /// Get element under pointer
    pub fn hovered_element(&self) -> Option<ElementId> {
        self.hovered_element
    }

    /// Get current pointer position
    pub fn pointer_position(&self) -> Option<Point> {
        self.pointer_position
    }

    /// Get shared access to widget handlers (for external registration)
    pub fn widget_handlers(&self) -> Arc<RwLock<HashMap<ElementId, Box<dyn Widget>>>> {
        self.widget_handlers.clone()
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
```

## File: `./src/core/render_object.rs`
```rs
use crate::layout::constraints::Size;

/// A color in RGBA format
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn with_alpha(&self, alpha: u8) -> Self {
        Color::rgba(self.r, self.g, self.b, alpha)
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }

    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);
}

/// 2D point
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Point = Point::new(0.0, 0.0);
}

/// Rectangle - OUR custom rect type
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_size(size: Size) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: size.width,
            height: size.height,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Convert to skia_safe::Rect
    pub fn to_skia_rect(&self) -> skia_safe::Rect {
        skia_safe::Rect::from_xywh(self.x, self.y, self.width, self.height)
    }
}

/// Text style configuration
#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
    pub font_family: String,
    pub font_size: f32,
    pub color: Color,
    pub bold: bool,
    pub italic: bool,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_family: "sans-serif".to_string(),
            font_size: 16.0,
            color: Color::BLACK,
            bold: false,
            italic: false,
        }
    }
}

/// Paint style for drawing operations
#[derive(Clone, Debug, PartialEq)]
pub struct Paint {
    pub color: Color,
    pub stroke_width: f32,
    pub anti_alias: bool,
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            stroke_width: 1.0,
            anti_alias: true,
        }
    }
}

/// 2D transformation matrix
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix {
    pub values: [[f32; 3]; 3],
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            values: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn translate(x: f32, y: f32) -> Self {
        Self {
            values: [[1.0, 0.0, x], [0.0, 1.0, y], [0.0, 0.0, 1.0]],
        }
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            values: [[sx, 0.0, 0.0], [0.0, sy, 0.0], [0.0, 0.0, 1.0]],
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

/// Backend-agnostic rendering primitives
#[derive(Clone, Debug, PartialEq)]
pub enum RenderObject {
    Rect { rect: Rect, paint: Paint },
    Text { content: String, style: TextStyle, position: Point },
    Image { size: Size },
    Clip { rect: Rect, child: Box<RenderObject> },
    Transform { matrix: Matrix, child: Box<RenderObject> },
    Group { children: Vec<RenderObject> },
    None,
}

impl RenderObject {
    pub fn rect(rect: Rect, color: Color) -> Self {
        RenderObject::Rect {
            rect,
            paint: Paint {
                color,
                ..Default::default()
            },
        }
    }

    pub fn text(content: String, style: TextStyle, position: Point) -> Self {
        RenderObject::Text { content, style, position }
    }

    pub fn transform(matrix: Matrix, child: RenderObject) -> Self {
        RenderObject::Transform {
            matrix,
            child: Box::new(child),
        }
    }

    pub fn clip(rect: Rect, child: RenderObject) -> Self {
        RenderObject::Clip {
            rect,
            child: Box::new(child),
        }
    }

    pub fn group(children: Vec<RenderObject>) -> Self {
        RenderObject::Group { children }
    }
}
```

## File: `./src/window_features/menubar.rs`
```rs

```

## File: `./src/core/event_system.rs`
```rs
// File: ./oxideui/src/core/event_system.rs
//! Complete event dispatching with gesture recognition and focus management

use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::core::element::ElementId;
use crate::core::event::{UiEvent, EventResult, MouseButton, Vector2};
use crate::core::render_object::Point;

/// Gesture recognizer for touch/mouse gestures
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GestureType {
    Tap,
    DoubleTap,
    LongPress,
    Pan,
    Pinch,
    Rotate,
}

/// Gesture state
#[derive(Debug, Clone)]
pub struct GestureState {
    pub gesture_type: GestureType,
    pub start_position: Point,
    pub current_position: Point,
    pub start_time: Instant,
    pub velocity: Vector2,
    pub scale: f32,
    pub rotation: f32,
}

impl GestureState {
    pub fn new(gesture_type: GestureType, position: Point) -> Self {
        Self {
            gesture_type,
            start_position: position,
            current_position: position,
            start_time: Instant::now(),
            velocity: Vector2::ZERO,
            scale: 1.0,
            rotation: 0.0,
        }
    }

    pub fn update(&mut self, position: Point) {
        let dt = self.start_time.elapsed().as_secs_f32();
        if dt > 0.0 {
            self.velocity = Vector2::new(
                (position.x - self.current_position.x) / dt,
                (position.y - self.current_position.y) / dt,
            );
        }
        self.current_position = position;
    }

    pub fn distance(&self) -> f32 {
        let dx = self.current_position.x - self.start_position.x;
        let dy = self.current_position.y - self.start_position.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Gesture recognizer
pub struct GestureRecognizer {
    active_gestures: HashMap<u64, GestureState>,
    tap_threshold: f32,
    long_press_duration: Duration,
    double_tap_duration: Duration,
    last_tap: Option<(Instant, Point)>,
}

impl GestureRecognizer {
    pub fn new() -> Self {
        Self {
            active_gestures: HashMap::new(),
            tap_threshold: 10.0,
            long_press_duration: Duration::from_millis(500),
            double_tap_duration: Duration::from_millis(300),
            last_tap: None,
        }
    }

    pub fn handle_pointer_down(&mut self, id: u64, position: Point) -> Option<GestureType> {
        // Check for double tap
        if let Some((last_time, last_pos)) = self.last_tap {
            if last_time.elapsed() < self.double_tap_duration {
                let dx = position.x - last_pos.x;
                let dy = position.y - last_pos.y;
                if (dx * dx + dy * dy).sqrt() < self.tap_threshold {
                    self.last_tap = None;
                    return Some(GestureType::DoubleTap);
                }
            }
        }

        // Start new gesture
        self.active_gestures.insert(id, GestureState::new(GestureType::Tap, position));
        None
    }

    pub fn handle_pointer_move(&mut self, id: u64, position: Point) -> Option<GestureType> {
        if let Some(gesture) = self.active_gestures.get_mut(&id) {
            gesture.update(position);

            // Check if moved beyond tap threshold
            if gesture.distance() > self.tap_threshold && gesture.gesture_type == GestureType::Tap {
                gesture.gesture_type = GestureType::Pan;
                return Some(GestureType::Pan);
            }

            // Check for long press
            if gesture.duration() > self.long_press_duration && gesture.gesture_type == GestureType::Tap {
                gesture.gesture_type = GestureType::LongPress;
                return Some(GestureType::LongPress);
            }
        }
        None
    }

    pub fn handle_pointer_up(&mut self, id: u64) -> Option<GestureType> {
        if let Some(gesture) = self.active_gestures.remove(&id) {
            if gesture.gesture_type == GestureType::Tap && gesture.distance() < self.tap_threshold {
                self.last_tap = Some((Instant::now(), gesture.start_position));
                return Some(GestureType::Tap);
            }
            return Some(gesture.gesture_type);
        }
        None
    }

    pub fn get_gesture(&self, id: u64) -> Option<&GestureState> {
        self.active_gestures.get(&id)
    }
}

impl Default for GestureRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Focus manager for keyboard navigation
pub struct FocusManager {
    focused: Option<ElementId>,
    focus_history: Vec<ElementId>,
    tab_order: Vec<ElementId>,
    focus_listeners: HashMap<ElementId, Vec<Box<dyn Fn(bool) + Send + Sync>>>,
}

impl FocusManager {
    pub fn new() -> Self {
        Self {
            focused: None,
            focus_history: Vec::new(),
            tab_order: Vec::new(),
            focus_listeners: HashMap::new(),
        }
    }

    pub fn set_focus(&mut self, element: Option<ElementId>) {
        if self.focused == element {
            return;
        }

        // Notify old focused element
        if let Some(old_focused) = self.focused {
            if let Some(listeners) = self.focus_listeners.get(&old_focused) {
                for listener in listeners {
                    listener(false);
                }
            }
        }

        // Update focus
        if let Some(new_focused) = element {
            self.focus_history.push(new_focused);
            if self.focus_history.len() > 10 {
                self.focus_history.remove(0);
            }
        }

        self.focused = element;

        // Notify new focused element
        if let Some(new_focused) = element {
            if let Some(listeners) = self.focus_listeners.get(&new_focused) {
                for listener in listeners {
                    listener(true);
                }
            }
        }
    }

    pub fn get_focused(&self) -> Option<ElementId> {
        self.focused
    }

    pub fn focus_next(&mut self) {
        if self.tab_order.is_empty() {
            return;
        }

        let current_index = self.focused
            .and_then(|f| self.tab_order.iter().position(|&e| e == f))
            .unwrap_or(0);

        let next_index = (current_index + 1) % self.tab_order.len();
        self.set_focus(Some(self.tab_order[next_index]));
    }

    pub fn focus_previous(&mut self) {
        if self.tab_order.is_empty() {
            return;
        }

        let current_index = self.focused
            .and_then(|f| self.tab_order.iter().position(|&e| e == f))
            .unwrap_or(0);

        let prev_index = if current_index == 0 {
            self.tab_order.len() - 1
        } else {
            current_index - 1
        };

        self.set_focus(Some(self.tab_order[prev_index]));
    }

    pub fn register_focusable(&mut self, element: ElementId) {
        if !self.tab_order.contains(&element) {
            self.tab_order.push(element);
        }
    }

    pub fn unregister_focusable(&mut self, element: ElementId) {
        self.tab_order.retain(|&e| e != element);
        if self.focused == Some(element) {
            self.set_focus(None);
        }
    }

    pub fn add_focus_listener<F>(&mut self, element: ElementId, listener: F)
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        self.focus_listeners
            .entry(element)
            .or_insert_with(Vec::new)
            .push(Box::new(listener));
    }
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Input method manager for text input
pub struct InputMethodManager {
    composition: Option<String>,
    composition_range: Option<(usize, usize)>,
    active_input: Option<ElementId>,
}

impl InputMethodManager {
    pub fn new() -> Self {
        Self {
            composition: None,
            composition_range: None,
            active_input: None,
        }
    }

    pub fn start_composition(&mut self, element: ElementId) {
        self.active_input = Some(element);
        self.composition = Some(String::new());
    }

    pub fn update_composition(&mut self, text: String, cursor: (usize, usize)) {
        self.composition = Some(text);
        self.composition_range = Some(cursor);
    }

    pub fn commit_composition(&mut self) -> Option<String> {
        let text = self.composition.take();
        self.composition_range = None;
        text
    }

    pub fn cancel_composition(&mut self) {
        self.composition = None;
        self.composition_range = None;
    }

    pub fn get_composition(&self) -> Option<&str> {
        self.composition.as_deref()
    }

    pub fn get_active_input(&self) -> Option<ElementId> {
        self.active_input
    }
}

impl Default for InputMethodManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Accessibility manager
pub struct AccessibilityManager {
    labels: HashMap<ElementId, String>,
    roles: HashMap<ElementId, AccessibilityRole>,
    screen_reader_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilityRole {
    Button,
    Text,
    TextField,
    Image,
    Link,
    Checkbox,
    RadioButton,
    Slider,
    List,
    ListItem,
    Heading,
}

impl AccessibilityManager {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            roles: HashMap::new(),
            screen_reader_enabled: Self::detect_screen_reader(),
        }
    }

    fn detect_screen_reader() -> bool {
        // Platform-specific detection
        #[cfg(target_os = "linux")]
        {
            std::env::var("ACCESSIBILITY_ENABLED").is_ok()
        }

        #[cfg(target_os = "macos")]
        {
            // Check VoiceOver status
            false // Placeholder
        }

        #[cfg(target_os = "windows")]
        {
            // Check Narrator status
            false // Placeholder
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            false
        }
    }

    pub fn set_label(&mut self, element: ElementId, label: String) {
        self.labels.insert(element, label);
    }

    pub fn set_role(&mut self, element: ElementId, role: AccessibilityRole) {
        self.roles.insert(element, role);
    }

    pub fn get_label(&self, element: ElementId) -> Option<&str> {
        self.labels.get(&element).map(|s| s.as_str())
    }

    pub fn get_role(&self, element: ElementId) -> Option<AccessibilityRole> {
        self.roles.get(&element).copied()
    }

    pub fn is_screen_reader_enabled(&self) -> bool {
        self.screen_reader_enabled
    }

    pub fn announce(&self, message: &str) {
        if self.screen_reader_enabled {
            // Platform-specific announcement
            println!("ACCESSIBILITY: {}", message);
        }
    }
}

impl Default for AccessibilityManager {
    fn default() -> Self {
        Self::new()
    }
}
```

## File: `./src/window_features/mod.rs`
```rs
mod menubar;
mod title_bar;
```

## File: `./src/native_events/sent_notification.rs`
```rs

```

## File: `./src/core/state_driven.rs`
```rs
// File: ./oxideui/src/core/state_driven.rs
//! State-driven rebuild system with granular updates

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use parking_lot::RwLock;
use crate::core::element::ElementId;

/// State subscription token
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateToken(u64);

static STATE_TOKEN_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

impl StateToken {
    pub fn new() -> Self {
        StateToken(STATE_TOKEN_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
    }
}

/// State change notification
#[derive(Debug, Clone)]
pub struct StateChange {
    pub token: StateToken,
    pub affected_elements: HashSet<ElementId>,
}

/// State tracker for fine-grained reactivity
pub struct StateTracker {
    /// Maps state tokens to affected elements
    subscriptions: Arc<RwLock<HashMap<StateToken, HashSet<ElementId>>>>,
    /// Maps elements to state tokens they depend on
    dependencies: Arc<RwLock<HashMap<ElementId, HashSet<StateToken>>>>,
    /// Pending state changes
    pending_changes: Arc<RwLock<Vec<StateChange>>>,
    /// Dirty elements that need rebuild
    dirty_elements: Arc<RwLock<HashSet<ElementId>>>,
}

impl StateTracker {
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            dependencies: Arc::new(RwLock::new(HashMap::new())),
            pending_changes: Arc::new(RwLock::new(Vec::new())),
            dirty_elements: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Subscribe an element to a state token
    pub fn subscribe(&self, element: ElementId, token: StateToken) {
        self.subscriptions
            .write()
            .entry(token)
            .or_insert_with(HashSet::new)
            .insert(element);

        self.dependencies
            .write()
            .entry(element)
            .or_insert_with(HashSet::new)
            .insert(token);
    }

    /// Unsubscribe an element from all states
    pub fn unsubscribe_all(&self, element: ElementId) {
        let tokens = {
            let deps = self.dependencies.read();
            deps.get(&element).cloned().unwrap_or_default()
        };

        let mut subscriptions = self.subscriptions.write();
        for token in tokens {
            if let Some(elements) = subscriptions.get_mut(&token) {
                elements.remove(&element);
            }
        }

        self.dependencies.write().remove(&element);
    }

    /// Notify that a state has changed
    pub fn notify_change(&self, token: StateToken) {
        let affected = {
            let subs = self.subscriptions.read();
            subs.get(&token).cloned().unwrap_or_default()
        };

        if !affected.is_empty() {
            self.pending_changes.write().push(StateChange {
                token,
                affected_elements: affected.clone(),
            });

            let mut dirty = self.dirty_elements.write();
            for element in affected {
                dirty.insert(element);
            }
        }
    }

    /// Get all dirty elements
    pub fn get_dirty_elements(&self) -> HashSet<ElementId> {
        self.dirty_elements.read().clone()
    }

    /// Clear dirty elements
    pub fn clear_dirty(&self) {
        self.dirty_elements.write().clear();
    }

    /// Get and clear pending changes
    pub fn drain_pending_changes(&self) -> Vec<StateChange> {
        let mut changes = self.pending_changes.write();
        changes.drain(..).collect()
    }

    /// Mark element as dirty manually
    pub fn mark_dirty(&self, element: ElementId) {
        self.dirty_elements.write().insert(element);
    }
}

impl Default for StateTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Reactive state container
pub struct ReactiveState<T: Clone + Send + Sync + 'static> {
    token: StateToken,
    value: Arc<RwLock<T>>,
    tracker: Arc<StateTracker>,
}

impl<T: Clone + Send + Sync + 'static> ReactiveState<T> {
    pub fn new(initial: T, tracker: Arc<StateTracker>) -> Self {
        Self {
            token: StateToken::new(),
            value: Arc::new(RwLock::new(initial)),
            tracker,
        }
    }

    /// Get current value
    pub fn get(&self) -> T {
        self.value.read().clone()
    }

    /// Set new value and notify subscribers
    pub fn set(&self, new_value: T) {
        *self.value.write() = new_value;
        self.tracker.notify_change(self.token);
    }

    /// Update value with function and notify
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        {
            let mut value = self.value.write();
            f(&mut value);
        }
        self.tracker.notify_change(self.token);
    }

    /// Subscribe element to this state
    pub fn subscribe(&self, element: ElementId) {
        self.tracker.subscribe(element, self.token);
    }

    /// Get state token
    pub fn token(&self) -> StateToken {
        self.token
    }
}

impl<T: Clone + Send + Sync + 'static> Clone for ReactiveState<T> {
    fn clone(&self) -> Self {
        Self {
            token: self.token,
            value: self.value.clone(),
            tracker: self.tracker.clone(),
        }
    }
}

/// Derived state that depends on other states
pub struct DerivedState<T: Clone + Send + Sync + 'static> {
    token: StateToken,
    compute: Arc<dyn Fn() -> T + Send + Sync>,
    cache: Arc<RwLock<Option<T>>>,
    tracker: Arc<StateTracker>,
}

impl<T: Clone + Send + Sync + 'static> DerivedState<T> {
    pub fn new<F>(compute: F, tracker: Arc<StateTracker>) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            token: StateToken::new(),
            compute: Arc::new(compute),
            cache: Arc::new(RwLock::new(None)),
            tracker,
        }
    }

    /// Get current value (recompute if needed)
    pub fn get(&self) -> T {
        let cached = self.cache.read().clone();
        if let Some(value) = cached {
            return value;
        }

        let value = (self.compute)();
        *self.cache.write() = Some(value.clone());
        value
    }

    /// Invalidate cache
    pub fn invalidate(&self) {
        *self.cache.write() = None;
        self.tracker.notify_change(self.token);
    }

    /// Subscribe element to this derived state
    pub fn subscribe(&self, element: ElementId) {
        self.tracker.subscribe(element, self.token);
    }
}

/// Effect runner for side effects
pub struct EffectRunner {
    effects: Arc<RwLock<Vec<Box<dyn Fn() + Send + Sync>>>>,
}

impl EffectRunner {
    pub fn new() -> Self {
        Self {
            effects: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register an effect
    pub fn register<F>(&self, effect: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.effects.write().push(Box::new(effect));
    }

    /// Run all effects
    pub fn run_all(&self) {
        let effects = self.effects.read();
        for effect in effects.iter() {
            effect();
        }
    }

    /// Clear all effects
    pub fn clear(&self) {
        self.effects.write().clear();
    }
}

impl Default for EffectRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Batch state updates
pub struct StateBatch {
    tracker: Arc<StateTracker>,
    changes: Vec<StateToken>,
}

impl StateBatch {
    pub fn new(tracker: Arc<StateTracker>) -> Self {
        Self {
            tracker,
            changes: Vec::new(),
        }
    }

    /// Queue a state change
    pub fn queue_change(&mut self, token: StateToken) {
        self.changes.push(token);
    }

    /// Commit all changes at once
    pub fn commit(self) {
        for token in self.changes {
            self.tracker.notify_change(token);
        }
    }
}
```

## File: `./src/native_events/mod.rs`
```rs
mod sent_notification;
```

## File: `./src/window_features/title_bar.rs`
```rs

```

## File: `./src/state_management/mod.rs`
```rs
pub mod state;
pub mod effect;
pub mod store;
pub mod derived;
pub mod pre_effect;
pub mod props;
pub mod bindable;


pub use state::State;
```

## File: `./src/state_management/derived.rs`
```rs

```

## File: `./src/state_management/effect.rs`
```rs

```

## File: `./src/state_management/pre_effect.rs`
```rs

```

## File: `./src/state_management/state.rs`
```rs
use parking_lot::RwLock;
use std::sync::Arc;

/// Reactive state container with observer pattern
#[derive(Clone)]
pub struct State<T: Clone + Send + Sync + 'static> {
    value: Arc<RwLock<T>>,
    listeners: Arc<RwLock<Vec<Box<dyn Fn(&T) + Send + Sync>>>>,
}



impl<T: Clone + Send + Sync + 'static> State<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(initial)),
            listeners: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn get(&self) -> T {
        self.value.read().clone()
    }

    pub fn set(&self, new_value: T) {
        *self.value.write() = new_value.clone();

        let listeners = self.listeners.read();
        for listener in listeners.iter() {
            listener(&new_value);
        }
    }

    pub fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut T),
    {
        let mut value = self.value.write();
        updater(&mut value);
        let new_value = value.clone();
        drop(value);

        let listeners = self.listeners.read();
        for listener in listeners.iter() {
            listener(&new_value);
        }
    }

    pub fn subscribe<F>(&self, listener: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        self.listeners.write().push(Box::new(listener));
    }
}
```

## File: `./src/state_management/store.rs`
```rs

```

## File: `./src/state_management/props.rs`
```rs

```

## File: `./src/theming/mod.rs`
```rs
mod default_theme;
mod theme_loader;

pub use default_theme::{LIGHT_THEME, DARK_THEME, ColorRGB, Theme as DefaultTheme};
pub use theme_loader::{ThemeConfig, ThemeColors, load_theme_from_file};

pub struct ThemeManager {
    config: ThemeConfig,
    is_dark: bool,
}

impl ThemeManager {
    pub fn new(config: ThemeConfig, is_dark: bool) -> Self {
        Self { config, is_dark }
    }

    pub fn toggle_dark_mode(&mut self) {
        self.is_dark = !self.is_dark;
    }

    pub fn set_dark_mode(&mut self, dark: bool) {
        self.is_dark = dark;
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark
    }

    pub fn get_css_variables(&self) -> String {
        self.config.to_css_variables(self.is_dark)
    }

    pub fn get_current_colors(&self) -> &ThemeColors {
        if self.is_dark {
            &self.config.dark
        } else {
            &self.config.light
        }
    }
}
```

## File: `./src/theming/default_theme.rs`
```rs
#[derive(Debug, Clone, Copy)]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: ColorRGB,
    pub foreground: ColorRGB,
    pub card: ColorRGB,
    pub card_foreground: ColorRGB,
    pub popover: ColorRGB,
    pub popover_foreground: ColorRGB,

    pub primary: ColorRGB,
    pub primary_foreground: ColorRGB,
    pub secondary: ColorRGB,
    pub secondary_foreground: ColorRGB,

    pub muted: ColorRGB,
    pub muted_foreground: ColorRGB,
    pub accent: ColorRGB,
    pub accent_foreground: ColorRGB,

    pub destructive: ColorRGB,
    pub destructive_foreground: ColorRGB,

    pub border: ColorRGB,
    pub input: ColorRGB,
    pub ring: ColorRGB,

    pub chart_1: ColorRGB,
    pub chart_2: ColorRGB,
    pub chart_3: ColorRGB,
    pub chart_4: ColorRGB,
    pub chart_5: ColorRGB,

    pub sidebar: ColorRGB,
    pub sidebar_foreground: ColorRGB,
    pub sidebar_primary: ColorRGB,
    pub sidebar_primary_foreground: ColorRGB,
    pub sidebar_accent: ColorRGB,
    pub sidebar_accent_foreground: ColorRGB,
    pub sidebar_border: ColorRGB,
    pub sidebar_ring: ColorRGB,

    pub font_sans: &'static str,
    pub font_mono: &'static str,
    pub font_serif: &'static str,

    pub radius: f32,

    pub shadow_x: f32,
    pub shadow_y: f32,
    pub shadow_blur: f32,
    pub shadow_spread: f32,
    pub shadow_opacity: f32,
}

pub const LIGHT_THEME: Theme = Theme {
    background: ColorRGB::new(255, 255, 255),
    foreground: ColorRGB::new(17, 24, 39),
    card: ColorRGB::new(255, 255, 255),
    card_foreground: ColorRGB::new(17, 24, 39),
    popover: ColorRGB::new(255, 255, 255),
    popover_foreground: ColorRGB::new(17, 24, 39),

    primary: ColorRGB::new(216, 121, 67),
    primary_foreground: ColorRGB::new(255, 255, 255),
    secondary: ColorRGB::new(82, 117, 117),
    secondary_foreground: ColorRGB::new(255, 255, 255),

    muted: ColorRGB::new(243, 244, 246),
    muted_foreground: ColorRGB::new(107, 114, 128),
    accent: ColorRGB::new(238, 238, 238),
    accent_foreground: ColorRGB::new(17, 24, 39),

    destructive: ColorRGB::new(239, 68, 68),
    destructive_foreground: ColorRGB::new(250, 250, 250),

    border: ColorRGB::new(229, 231, 235),
    input: ColorRGB::new(229, 231, 235),
    ring: ColorRGB::new(216, 121, 67),

    chart_1: ColorRGB::new(95, 135, 135),
    chart_2: ColorRGB::new(231, 138, 83),
    chart_3: ColorRGB::new(251, 203, 151),
    chart_4: ColorRGB::new(136, 136, 136),
    chart_5: ColorRGB::new(153, 153, 153),

    sidebar: ColorRGB::new(243, 244, 246),
    sidebar_foreground: ColorRGB::new(17, 24, 39),
    sidebar_primary: ColorRGB::new(216, 121, 67),
    sidebar_primary_foreground: ColorRGB::new(255, 255, 255),
    sidebar_accent: ColorRGB::new(255, 255, 255),
    sidebar_accent_foreground: ColorRGB::new(17, 24, 39),
    sidebar_border: ColorRGB::new(229, 231, 235),
    sidebar_ring: ColorRGB::new(216, 121, 67),

    font_sans: "Inter",
    font_mono: "JetBrains Mono",
    font_serif: "serif",

    radius: 0.75,

    shadow_x: 0.0,
    shadow_y: 1.0,
    shadow_blur: 4.0,
    shadow_spread: 0.0,
    shadow_opacity: 0.05,
};

pub const DARK_THEME: Theme = Theme {
    background: ColorRGB::new(18, 17, 19),
    foreground: ColorRGB::new(193, 193, 193),
    card: ColorRGB::new(18, 18, 18),
    card_foreground: ColorRGB::new(193, 193, 193),
    popover: ColorRGB::new(18, 17, 19),
    popover_foreground: ColorRGB::new(193, 193, 193),

    primary: ColorRGB::new(231, 138, 83),
    primary_foreground: ColorRGB::new(18, 17, 19),
    secondary: ColorRGB::new(95, 135, 135),
    secondary_foreground: ColorRGB::new(18, 17, 19),

    muted: ColorRGB::new(34, 34, 34),
    muted_foreground: ColorRGB::new(136, 136, 136),
    accent: ColorRGB::new(51, 51, 51),
    accent_foreground: ColorRGB::new(193, 193, 193),

    destructive: ColorRGB::new(95, 135, 135),
    destructive_foreground: ColorRGB::new(18, 17, 19),

    border: ColorRGB::new(34, 34, 34),
    input: ColorRGB::new(34, 34, 34),
    ring: ColorRGB::new(231, 138, 83),

    chart_1: ColorRGB::new(95, 135, 135),
    chart_2: ColorRGB::new(231, 138, 83),
    chart_3: ColorRGB::new(251, 203, 151),
    chart_4: ColorRGB::new(136, 136, 136),
    chart_5: ColorRGB::new(153, 153, 153),

    sidebar: ColorRGB::new(18, 18, 18),
    sidebar_foreground: ColorRGB::new(193, 193, 193),
    sidebar_primary: ColorRGB::new(231, 138, 83),
    sidebar_primary_foreground: ColorRGB::new(18, 17, 19),
    sidebar_accent: ColorRGB::new(51, 51, 51),
    sidebar_accent_foreground: ColorRGB::new(193, 193, 193),
    sidebar_border: ColorRGB::new(34, 34, 34),
    sidebar_ring: ColorRGB::new(231, 138, 83),

    font_sans: "Inter",
    font_mono: "JetBrains Mono",
    font_serif: "serif",

    radius: 0.75,

    shadow_x: 0.0,
    shadow_y: 1.0,
    shadow_blur: 4.0,
    shadow_spread: 0.0,
    shadow_opacity: 0.05,
};
```

## File: `./src/theming/theme_loader.rs`
```rs
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;
use anyhow::{Result, Context};
use crate::core::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub light: ThemeColors,
    pub dark: ThemeColors,
    #[serde(default)]
    pub css_variables: HashMap<String, String>,
    #[serde(default = "default_font_sans")]
    pub font_sans: String,
    #[serde(default = "default_font_mono")]
    pub font_mono: String,
    #[serde(default = "default_radius")]
    pub radius: f32,
    #[serde(default)]
    pub is_dark: bool,
}

fn default_font_sans() -> String {
    "Inter".to_string()
}

fn default_font_mono() -> String {
    "JetBrains Mono".to_string()
}

fn default_radius() -> f32 {
    0.5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    // Background
    pub background: [u8; 3],
    pub foreground: [u8; 3],

    // Cards
    pub card: [u8; 3],
    pub card_foreground: [u8; 3],

    // Popover
    pub popover: [u8; 3],
    pub popover_foreground: [u8; 3],

    // Primary
    pub primary: [u8; 3],
    pub primary_foreground: [u8; 3],

    // Secondary
    pub secondary: [u8; 3],
    pub secondary_foreground: [u8; 3],

    // Muted
    pub muted: [u8; 3],
    pub muted_foreground: [u8; 3],

    // Accent
    pub accent: [u8; 3],
    pub accent_foreground: [u8; 3],

    // Destructive
    pub destructive: [u8; 3],
    pub destructive_foreground: [u8; 3],

    // Borders & Inputs
    pub border: [u8; 3],
    pub input: [u8; 3],
    pub ring: [u8; 3],

    // Charts
    #[serde(default = "default_charts")]
    pub chart_1: [u8; 3],
    #[serde(default = "default_charts")]
    pub chart_2: [u8; 3],
    #[serde(default = "default_charts")]
    pub chart_3: [u8; 3],
    #[serde(default = "default_charts")]
    pub chart_4: [u8; 3],
    #[serde(default = "default_charts")]
    pub chart_5: [u8; 3],

    // Sidebar (Radix UI inspired)
    #[serde(default)]
    pub sidebar: [u8; 3],
    #[serde(default)]
    pub sidebar_foreground: [u8; 3],
    #[serde(default)]
    pub sidebar_primary: [u8; 3],
    #[serde(default)]
    pub sidebar_primary_foreground: [u8; 3],
    #[serde(default)]
    pub sidebar_accent: [u8; 3],
    #[serde(default)]
    pub sidebar_accent_foreground: [u8; 3],
    #[serde(default)]
    pub sidebar_border: [u8; 3],
    #[serde(default)]
    pub sidebar_ring: [u8; 3],

    // Shadows (like shadcn)
    #[serde(default = "default_shadow_x")]
    pub shadow_x: f32,
    #[serde(default = "default_shadow_y")]
    pub shadow_y: f32,
    #[serde(default = "default_shadow_blur")]
    pub shadow_blur: f32,
    #[serde(default = "default_shadow_spread")]
    pub shadow_spread: f32,
    #[serde(default = "default_shadow_opacity")]
    pub shadow_opacity: f32,
}

fn default_charts() -> [u8; 3] {
    [0, 0, 0]
}

fn default_shadow_x() -> f32 { 0.0 }
fn default_shadow_y() -> f32 { 1.0 }
fn default_shadow_blur() -> f32 { 4.0 }
fn default_shadow_spread() -> f32 { 0.0 }
fn default_shadow_opacity() -> f32 { 0.05 }

impl ThemeConfig {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read theme file: {}", path))?;
        let mut theme: Self = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse theme JSON: {}", path))?;

        // Auto-calculate sidebar colors if not provided
        theme.calculate_sidebar_colors();

        Ok(theme)
    }

    fn calculate_sidebar_colors(&mut self) {
        // If sidebar colors aren't set, calculate them from other colors
        if self.light.sidebar == [0, 0, 0] {
            // Light sidebar is slightly darker than background
            self.light.sidebar = self.light.muted;
            self.light.sidebar_foreground = self.light.foreground;
            self.light.sidebar_primary = self.light.primary;
            self.light.sidebar_primary_foreground = self.light.primary_foreground;
            self.light.sidebar_accent = self.light.accent;
            self.light.sidebar_accent_foreground = self.light.accent_foreground;
            self.light.sidebar_border = self.light.border;
            self.light.sidebar_ring = self.light.ring;
        }

        if self.dark.sidebar == [0, 0, 0] {
            // Dark sidebar is slightly lighter than background
            let mut sidebar = self.dark.background;
            for i in 0..3 {
                sidebar[i] = sidebar[i].saturating_add(10);
            }
            self.dark.sidebar = sidebar;
            self.dark.sidebar_foreground = self.dark.foreground;
            self.dark.sidebar_primary = self.dark.primary;
            self.dark.sidebar_primary_foreground = self.dark.primary_foreground;
            self.dark.sidebar_accent = self.dark.accent;
            self.dark.sidebar_accent_foreground = self.dark.accent_foreground;
            self.dark.sidebar_border = self.dark.border;
            self.dark.sidebar_ring = self.dark.ring;
        }
    }

    pub fn to_css_variables(&self, is_dark: bool) -> String {
        let colors = if is_dark { &self.dark } else { &self.light };

        let mut css = String::from(":root {\n");

        // Background colors
        css.push_str(&format!("  --background: {} {} {};\n", colors.background[0], colors.background[1], colors.background[2]));
        css.push_str(&format!("  --foreground: {} {} {};\n", colors.foreground[0], colors.foreground[1], colors.foreground[2]));

        // Card colors
        css.push_str(&format!("  --card: {} {} {};\n", colors.card[0], colors.card[1], colors.card[2]));
        css.push_str(&format!("  --card-foreground: {} {} {};\n", colors.card_foreground[0], colors.card_foreground[1], colors.card_foreground[2]));

        // Primary colors
        css.push_str(&format!("  --primary: {} {} {};\n", colors.primary[0], colors.primary[1], colors.primary[2]));
        css.push_str(&format!("  --primary-foreground: {} {} {};\n", colors.primary_foreground[0], colors.primary_foreground[1], colors.primary_foreground[2]));

        // Secondary colors
        css.push_str(&format!("  --secondary: {} {} {};\n", colors.secondary[0], colors.secondary[1], colors.secondary[2]));
        css.push_str(&format!("  --secondary-foreground: {} {} {};\n", colors.secondary_foreground[0], colors.secondary_foreground[1], colors.secondary_foreground[2]));

        // Muted colors
        css.push_str(&format!("  --muted: {} {} {};\n", colors.muted[0], colors.muted[1], colors.muted[2]));
        css.push_str(&format!("  --muted-foreground: {} {} {};\n", colors.muted_foreground[0], colors.muted_foreground[1], colors.muted_foreground[2]));

        // Accent colors
        css.push_str(&format!("  --accent: {} {} {};\n", colors.accent[0], colors.accent[1], colors.accent[2]));
        css.push_str(&format!("  --accent-foreground: {} {} {};\n", colors.accent_foreground[0], colors.accent_foreground[1], colors.accent_foreground[2]));

        // Destructive colors
        css.push_str(&format!("  --destructive: {} {} {};\n", colors.destructive[0], colors.destructive[1], colors.destructive[2]));
        css.push_str(&format!("  --destructive-foreground: {} {} {};\n", colors.destructive_foreground[0], colors.destructive_foreground[1], colors.destructive_foreground[2]));

        // Borders & Inputs
        css.push_str(&format!("  --border: {} {} {};\n", colors.border[0], colors.border[1], colors.border[2]));
        css.push_str(&format!("  --input: {} {} {};\n", colors.input[0], colors.input[1], colors.input[2]));
        css.push_str(&format!("  --ring: {} {} {};\n", colors.ring[0], colors.ring[1], colors.ring[2]));

        // Sidebar colors
        css.push_str(&format!("  --sidebar: {} {} {};\n", colors.sidebar[0], colors.sidebar[1], colors.sidebar[2]));
        css.push_str(&format!("  --sidebar-foreground: {} {} {};\n", colors.sidebar_foreground[0], colors.sidebar_foreground[1], colors.sidebar_foreground[2]));
        css.push_str(&format!("  --sidebar-primary: {} {} {};\n", colors.sidebar_primary[0], colors.sidebar_primary[1], colors.sidebar_primary[2]));
        css.push_str(&format!("  --sidebar-primary-foreground: {} {} {};\n", colors.sidebar_primary_foreground[0], colors.sidebar_primary_foreground[1], colors.sidebar_primary_foreground[2]));

        // Fonts
        css.push_str(&format!("  --font-sans: '{}';\n", self.font_sans));
        css.push_str(&format!("  --font-mono: '{}';\n", self.font_mono));

        // Border radius
        css.push_str(&format!("  --radius: {}rem;\n", self.radius));

        // Shadows
        css.push_str(&format!("  --shadow: {}px {}px {}px {}px rgba(0, 0, 0, {});\n",
                              colors.shadow_x, colors.shadow_y, colors.shadow_blur, colors.shadow_spread, colors.shadow_opacity));

        css.push_str("}\n");

        css
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let light = ThemeColors {
            background: [255, 255, 255],
            foreground: [17, 24, 39],
            card: [255, 255, 255],
            card_foreground: [17, 24, 39],
            popover: [255, 255, 255],
            popover_foreground: [17, 24, 39],
            primary: [216, 121, 67],
            primary_foreground: [255, 255, 255],
            secondary: [82, 117, 117],
            secondary_foreground: [255, 255, 255],
            muted: [243, 244, 246],
            muted_foreground: [107, 114, 128],
            accent: [238, 238, 238],
            accent_foreground: [17, 24, 39],
            destructive: [239, 68, 68],
            destructive_foreground: [250, 250, 250],
            border: [229, 231, 235],
            input: [229, 231, 235],
            ring: [216, 121, 67],
            chart_1: [95, 135, 135],
            chart_2: [231, 138, 83],
            chart_3: [251, 203, 151],
            chart_4: [136, 136, 136],
            chart_5: [153, 153, 153],
            sidebar: [243, 244, 246],
            sidebar_foreground: [17, 24, 39],
            sidebar_primary: [216, 121, 67],
            sidebar_primary_foreground: [255, 255, 255],
            sidebar_accent: [255, 255, 255],
            sidebar_accent_foreground: [17, 24, 39],
            sidebar_border: [229, 231, 235],
            sidebar_ring: [216, 121, 67],
            shadow_x: 0.0,
            shadow_y: 1.0,
            shadow_blur: 4.0,
            shadow_spread: 0.0,
            shadow_opacity: 0.05,
        };

        let dark = ThemeColors {
            background: [18, 17, 19],
            foreground: [193, 193, 193],
            card: [18, 18, 18],
            card_foreground: [193, 193, 193],
            popover: [18, 17, 19],
            popover_foreground: [193, 193, 193],
            primary: [231, 138, 83],
            primary_foreground: [18, 17, 19],
            secondary: [95, 135, 135],
            secondary_foreground: [18, 17, 19],
            muted: [34, 34, 34],
            muted_foreground: [136, 136, 136],
            accent: [51, 51, 51],
            accent_foreground: [193, 193, 193],
            destructive: [95, 135, 135],
            destructive_foreground: [18, 17, 19],
            border: [34, 34, 34],
            input: [34, 34, 34],
            ring: [231, 138, 83],
            chart_1: [95, 135, 135],
            chart_2: [231, 138, 83],
            chart_3: [251, 203, 151],
            chart_4: [136, 136, 136],
            chart_5: [153, 153, 153],
            sidebar: [18, 18, 18],
            sidebar_foreground: [193, 193, 193],
            sidebar_primary: [231, 138, 83],
            sidebar_primary_foreground: [18, 17, 19],
            sidebar_accent: [51, 51, 51],
            sidebar_accent_foreground: [193, 193, 193],
            sidebar_border: [34, 34, 34],
            sidebar_ring: [231, 138, 83],
            shadow_x: 0.0,
            shadow_y: 1.0,
            shadow_blur: 4.0,
            shadow_spread: 0.0,
            shadow_opacity: 0.05,
        };

        Self {
            light,
            dark,
            css_variables: HashMap::new(),
            font_sans: "Inter".to_string(),
            font_mono: "JetBrains Mono".to_string(),
            radius: 0.5,
            is_dark: false,
        }
    }
}

pub fn load_theme_from_file(path: &str) -> Result<ThemeConfig> {
    ThemeConfig::load_from_file(path)
}

impl ThemeColors {
    pub fn get_color(&self, name: &str) -> Color {
        let rgb = match name {
            "background" => self.background,
            "foreground" => self.foreground,
            "card" => self.card,
            "card_foreground" => self.card_foreground,
            "popover" => self.popover,
            "popover_foreground" => self.popover_foreground,
            "primary" => self.primary,
            "primary_foreground" => self.primary_foreground,
            "secondary" => self.secondary,
            "secondary_foreground" => self.secondary_foreground,
            "muted" => self.muted,
            "muted_foreground" => self.muted_foreground,
            "accent" => self.accent,
            "accent_foreground" => self.accent_foreground,
            "destructive" => self.destructive,
            "destructive_foreground" => self.destructive_foreground,
            "border" => self.border,
            "input" => self.input,
            "ring" => self.ring,
            "sidebar" => self.sidebar,
            "sidebar_foreground" => self.sidebar_foreground,
            "sidebar_primary" => self.sidebar_primary,
            "sidebar_primary_foreground" => self.sidebar_primary_foreground,
            "sidebar_accent" => self.sidebar_accent,
            "sidebar_accent_foreground" => self.sidebar_accent_foreground,
            "sidebar_border" => self.sidebar_border,
            "sidebar_ring" => self.sidebar_ring,
            "chart_1" => self.chart_1,
            "chart_2" => self.chart_2,
            "chart_3" => self.chart_3,
            "chart_4" => self.chart_4,
            "chart_5" => self.chart_5,
            _ => self.foreground,
        };
        Color::rgb(rgb[0], rgb[1], rgb[2])
    }
}
```

## File: `./src/animation/mod.rs`
```rs
mod animations;

pub use animations::{
    Animation, AnimationController, EasingCurve,
    AnimationId, AnimationRepeat, Interpolate
};
```

## File: `./src/state_management/bindable.rs`
```rs

```

## File: `./src/animation/animations.rs`
```rs
//! Complete animation system with curves, springs, and keyframes
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fmt;

/// Animation ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnimationId(u64);

static ANIMATION_ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

impl AnimationId {
    pub fn new() -> Self {
        AnimationId(ANIMATION_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
    }
}

/// Easing curve for animations
#[derive(Debug, Clone, Copy)]
pub enum EasingCurve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Cubic(f32, f32, f32, f32), // Bezier control points
    Spring { damping: f32, stiffness: f32 },
}

impl EasingCurve {
    pub fn evaluate(&self, t: f32) -> f32 {
        match self {
            EasingCurve::Linear => t,
            EasingCurve::EaseIn => t * t,
            EasingCurve::EaseOut => t * (2.0 - t),
            EasingCurve::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingCurve::Cubic(x1, y1, x2, y2) => {
                self.cubic_bezier(t, *x1, *y1, *x2, *y2)
            }
            EasingCurve::Spring { damping, stiffness } => {
                self.spring_evaluation(t, *damping, *stiffness)
            }
        }
    }

    fn cubic_bezier(&self, t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        // Simplified cubic bezier calculation
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        // Only using y control points for the value
        mt3 * 0.0 + 3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3 * 1.0
    }

    fn spring_evaluation(&self, t: f32, damping: f32, stiffness: f32) -> f32 {
        let omega = stiffness.sqrt();
        let zeta = damping / (2.0 * omega);

        if zeta < 1.0 {
            // Underdamped
            let omega_d = omega * (1.0 - zeta * zeta).sqrt();
            let a = (-zeta * omega * t).exp();
            let b = (omega_d * t).cos();
            let c = (zeta / (1.0 - zeta * zeta).sqrt()) * (omega_d * t).sin();
            1.0 - a * (b + c)
        } else {
            // Critically damped or overdamped
            let a = (-omega * t).exp();
            1.0 - a * (1.0 + omega * t)
        }
    }
}

/// Animated value
#[derive(Debug, Clone)]
pub struct AnimatedValue<T> {
    pub start: T,
    pub end: T,
    pub current: T,
}

impl<T: Interpolate> AnimatedValue<T> {
    pub fn new(start: T, end: T) -> Self {
        Self {
            current: start.clone(),
            start,
            end,
        }
    }

    pub fn update(&mut self, t: f32) {
        self.current = self.start.interpolate(&self.end, t);
    }
}

/// Trait for types that can be interpolated
pub trait Interpolate: Clone {
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}

impl Interpolate for f32 {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Interpolate for (f32, f32) {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        (
            self.0 + (other.0 - self.0) * t,
            self.1 + (other.1 - self.1) * t,
        )
    }
}

impl Interpolate for crate::core::Color {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        crate::core::Color::rgba(
            (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
            (self.a as f32 + (other.a as f32 - self.a as f32) * t) as u8,
        )
    }
}

/// Animation state
#[derive(Clone)]
pub struct Animation<T: Interpolate> {
    pub id: AnimationId,
    pub value: AnimatedValue<T>,
    pub duration: Duration,
    pub curve: EasingCurve,
    pub start_time: Instant,
    pub repeat: AnimationRepeat,
    pub on_complete: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationRepeat {
    Once,
    Loop,
    Reverse,
    Count(usize),
}

impl<T: Interpolate> Animation<T> {
    pub fn new(start: T, end: T, duration: Duration) -> Self {
        Self {
            id: AnimationId::new(),
            value: AnimatedValue::new(start, end),
            duration,
            curve: EasingCurve::Linear,
            start_time: Instant::now(),
            repeat: AnimationRepeat::Once,
            on_complete: None,
        }
    }

    pub fn with_curve(mut self, curve: EasingCurve) -> Self {
        self.curve = curve;
        self
    }

    pub fn with_repeat(mut self, repeat: AnimationRepeat) -> Self {
        self.repeat = repeat;
        self
    }

    pub fn with_on_complete<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_complete = Some(std::sync::Arc::new(callback));
        self
    }

    pub fn update(&mut self) -> bool {
        let elapsed = self.start_time.elapsed();
        let t = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        let eased_t = self.curve.evaluate(t);
        self.value.update(eased_t);

        if t >= 1.0 {
            match self.repeat {
                AnimationRepeat::Once => {
                    if let Some(callback) = &self.on_complete {
                        callback();
                    }
                    return false; // Animation complete
                }
                AnimationRepeat::Loop => {
                    self.start_time = Instant::now();
                }
                AnimationRepeat::Reverse => {
                    std::mem::swap(&mut self.value.start, &mut self.value.end);
                    self.start_time = Instant::now();
                }
                AnimationRepeat::Count(n) if n > 1 => {
                    self.repeat = AnimationRepeat::Count(n - 1);
                    self.start_time = Instant::now();
                }
                AnimationRepeat::Count(_) => {
                    if let Some(callback) = &self.on_complete {
                        callback();
                    }
                    return false;
                }
            }
        }
        true // Animation continues
    }

    pub fn current_value(&self) -> &T {
        &self.value.current
    }
}

// Manual Debug implementation that doesn't require Debug for on_complete
impl<T: Interpolate + fmt::Debug> fmt::Debug for Animation<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Animation")
            .field("id", &self.id)
            .field("value", &self.value)
            .field("duration", &self.duration)
            .field("curve", &self.curve)
            .field("start_time", &self.start_time)
            .field("repeat", &self.repeat)
            .finish()
    }
}

/// Animation controller
pub struct AnimationController<T: Interpolate> {
    animations: HashMap<AnimationId, Animation<T>>,
}

impl<T: Interpolate> AnimationController<T> {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }

    pub fn add(&mut self, animation: Animation<T>) -> AnimationId {
        let id = animation.id;
        self.animations.insert(id, animation);
        id
    }

    pub fn remove(&mut self, id: AnimationId) {
        self.animations.remove(&id);
    }

    pub fn update_all(&mut self) {
        self.animations.retain(|_, anim| anim.update());
    }

    pub fn get(&self, id: AnimationId) -> Option<&Animation<T>> {
        self.animations.get(&id)
    }

    pub fn get_mut(&mut self, id: AnimationId) -> Option<&mut Animation<T>> {
        self.animations.get_mut(&id)
    }

    pub fn clear(&mut self) {
        self.animations.clear();
    }
}

impl<T: Interpolate> Default for AnimationController<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Keyframe animation
#[derive(Debug, Clone)]
pub struct Keyframe<T: Clone> {
    pub time: f32, // 0.0 to 1.0
    pub value: T,
    pub curve: EasingCurve,
}

pub struct KeyframeAnimation<T: Interpolate + Clone> {
    pub id: AnimationId,
    pub keyframes: Vec<Keyframe<T>>,
    pub duration: Duration,
    pub start_time: Instant,
    pub current_value: T,
}

impl<T: Interpolate + Clone> KeyframeAnimation<T> {
    pub fn new(keyframes: Vec<Keyframe<T>>, duration: Duration) -> Self {
        let current_value = keyframes.first().unwrap().value.clone();
        Self {
            id: AnimationId::new(),
            keyframes,
            duration,
            start_time: Instant::now(),
            current_value,
        }
    }

    pub fn update(&mut self) -> bool {
        let elapsed = self.start_time.elapsed();
        let t = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);

        // Find surrounding keyframes
        let mut prev_kf = &self.keyframes[0];
        let mut next_kf = &self.keyframes[0];

        for kf in &self.keyframes {
            if kf.time <= t {
                prev_kf = kf;
            }
            if kf.time >= t {
                next_kf = kf;
                break;
            }
        }

        // Interpolate between keyframes
        if prev_kf.time == next_kf.time {
            self.current_value = prev_kf.value.clone();
        } else {
            let segment_t = (t - prev_kf.time) / (next_kf.time - prev_kf.time);
            let eased_t = prev_kf.curve.evaluate(segment_t);
            self.current_value = prev_kf.value.interpolate(&next_kf.value, eased_t);
        }

        t < 1.0
    }
}

/// Transition builder for implicit animations
pub struct TransitionBuilder<T: Interpolate> {
    value: AnimatedValue<T>,
    duration: Duration,
    curve: EasingCurve,
}

impl<T: Interpolate> TransitionBuilder<T> {
    pub fn new(from: T, to: T) -> Self {
        Self {
            value: AnimatedValue::new(from, to),
            duration: Duration::from_millis(300),
            curve: EasingCurve::EaseInOut,
        }
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn curve(mut self, curve: EasingCurve) -> Self {
        self.curve = curve;
        self
    }

    pub fn build(self) -> Animation<T> {
        Animation::new(self.value.start, self.value.end, self.duration)
            .with_curve(self.curve)
    }
}
```

## File: `./src/production/mod.rs`
```rs
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use crate::core::element::ElementId;
use crate::core::state_driven::StateTracker;
use crate::widgets::scrolling::ScrollController;

pub struct ProductionRuntime {
    animation_frame_callbacks: Vec<Arc<dyn Fn(f32) + Send + Sync>>,
    scroll_controllers: HashMap<u64, ScrollController>,
    state_tracker: Arc<StateTracker>,
    last_frame_time: Instant,
    frame_count: u64,
}

#[derive(Default)]
pub struct ProductionRuntimeBuilder {
    animation_frame_callbacks: Vec<Arc<dyn Fn(f32) + Send + Sync>>,
    scroll_controllers: HashMap<u64, ScrollController>,
    state_tracker: Option<Arc<StateTracker>>,
}

impl ProductionRuntime {
    pub fn new() -> Self {
        Self {
            animation_frame_callbacks: Vec::new(),
            scroll_controllers: HashMap::new(),
            state_tracker: Arc::new(StateTracker::new()),
            last_frame_time: Instant::now(),
            frame_count: 0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update animations
        for callback in &self.animation_frame_callbacks {
            callback(dt);
        }

        // Update scroll momentum
        for controller in self.scroll_controllers.values_mut() {
            controller.update_momentum(dt);
        }

        // Process state changes
        let dirty_elements = self.state_tracker.get_dirty_elements();
        if !dirty_elements.is_empty() {
            // Mark elements for rebuild
            for element_id in dirty_elements {
                // In a real implementation this would trigger rebuild
                let _ = element_id;
            }
            self.state_tracker.clear_dirty();
        }

        // Track frame time
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;
        self.frame_count += 1;

        if self.frame_count % 60 == 0 {
            let fps = 1.0 / frame_time.as_secs_f32();
            println!("FPS: {:.1}", fps);
        }
    }

    pub fn add_animation_frame_callback<F>(&mut self, callback: F)
    where
        F: Fn(f32) + Send + Sync + 'static,
    {
        self.animation_frame_callbacks.push(Arc::new(callback));
    }

    pub fn add_scroll_controller(&mut self, id: u64, controller: ScrollController) {
        self.scroll_controllers.insert(id, controller);
    }

    pub fn get_state_tracker(&self) -> Arc<StateTracker> {
        self.state_tracker.clone()
    }
}

impl ProductionRuntimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_animation_frame_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32) + Send + Sync + 'static,
    {
        self.animation_frame_callbacks.push(Arc::new(callback));
        self
    }

    pub fn with_scroll_controller(mut self, id: u64, controller: ScrollController) -> Self {
        self.scroll_controllers.insert(id, controller);
        self
    }

    pub fn with_state_tracker(mut self, tracker: Arc<StateTracker>) -> Self {
        self.state_tracker = Some(tracker);
        self
    }

    pub fn build(self) -> ProductionRuntime {
        ProductionRuntime {
            animation_frame_callbacks: self.animation_frame_callbacks,
            scroll_controllers: self.scroll_controllers,
            state_tracker: self.state_tracker.unwrap_or_else(|| Arc::new(StateTracker::new())),
            last_frame_time: Instant::now(),
            frame_count: 0,
        }
    }
}
```

## File: `./Cargo.toml`
```toml
[package]
name = "oxideui"
version = "0.1.0"
edition = "2021"
description = "A modern UI framework inspired by Jetpack Compose and Flutter"
authors = ["xOphiuchus"]
license = "Apache-2.0"
repository = "https://github.com/TungstenDevs/OxideUI"

[dependencies]
anyhow = "1.0.100"
gl = "0.14.0"
glutin = { version = "0.32.3", optional = true, features = ["egl", "glx", "wgl"] }
glutin-winit = "0.5.0"
oneshot = "0.1.11"
parking_lot = "0.12.5"
raw-window-handle = "0.6.2"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.149"
skia-safe = { version = "0.91.1", features = ["gl", "save-svg-images", "textlayout", "svg", "webp"], optional = true }
softbuffer = "0.4.8"
winit = { version = "0.30.12", features = ["wayland", "x11", "rwh_06"] }
winit_input_helper = "0.17.0"

[features]
default = ["skia-cpu"]
skia-cpu = ["dep:skia-safe", "dep:glutin"]
skia-opengl = ["dep:skia-safe", "dep:glutin", "skia-safe/gl"]
production = []
```

## File: `./src/production/state.rs`
```rs
pub use crate::core::state_driven::{
    ReactiveState,
    StateTracker,
    DerivedState,
    StateToken,
    StateChange,
    StateBatch,
    EffectRunner,
};
```

## File: `./LICENSE`
```
                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      "control" means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      "You" (or "Your") shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      "Source" form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      "Object" form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      "Work" shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      "Derivative Works" shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      "Contribution" shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, "submitted"
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as "Not a Contribution."

      "Contributor" shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a "NOTICE" text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an "AS IS" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets "[]"
      replaced with your own identifying information. (Don't include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same "printed page" as the copyright notice for easier
      identification within third-party archives.

   Copyright [2026] [xOphiuchus, TungstenDevs]

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```

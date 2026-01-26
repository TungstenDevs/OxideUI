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
pub use widgets::basic::{Container, Text, Column, Row, Center};
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
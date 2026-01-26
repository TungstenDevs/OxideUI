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
    fn reduce(&mut self, _action: Box<dyn Any + Send>) -> bool {
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
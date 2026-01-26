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
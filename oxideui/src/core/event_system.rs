// File: ./oxideui/src/core/event_system.rs
//! Complete event dispatching with gesture recognition and focus management

use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::core::element::ElementId;
use crate::core::event::{Vector2};
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
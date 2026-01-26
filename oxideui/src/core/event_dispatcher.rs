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
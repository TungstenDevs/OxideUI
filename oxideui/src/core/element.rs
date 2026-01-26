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

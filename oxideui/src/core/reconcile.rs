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
        _theme: Arc<crate::core::context::Theme>,
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

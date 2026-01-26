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
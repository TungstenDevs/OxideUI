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

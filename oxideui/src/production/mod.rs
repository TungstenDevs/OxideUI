use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
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
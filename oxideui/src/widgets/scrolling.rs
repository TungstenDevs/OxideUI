// File: ./oxideui/src/widgets/scrolling.rs
//! Advanced scrolling and clipping with momentum and snap points

use std::time::{Duration, Instant};
use crate::core::render_object::{Point, Rect};
use crate::core::event::Vector2;

/// Scroll physics for natural scrolling behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollPhysics {
    /// Bouncy iOS-style scrolling
    Bouncing,
    /// Clamped Android-style scrolling
    Clamping,
    /// No physics, direct positioning
    Never,
}

/// Scroll controller for programmatic scrolling
pub struct ScrollController {
    pub offset: Vector2,
    pub max_offset: Vector2,
    pub physics: ScrollPhysics,
    velocity: Vector2,
    last_update: Instant,
    is_scrolling: bool,
    momentum_enabled: bool,
}

impl ScrollController {
    pub fn new() -> Self {
        Self {
            offset: Vector2::ZERO,
            max_offset: Vector2::ZERO,
            physics: ScrollPhysics::Bouncing,
            velocity: Vector2::ZERO,
            last_update: Instant::now(),
            is_scrolling: false,
            momentum_enabled: true,
        }
    }

    /// Update scroll position with delta
    pub fn scroll(&mut self, delta: Vector2) {
        let new_offset = Vector2::new(
            self.offset.x + delta.x,
            self.offset.y + delta.y,
        );

        self.offset = self.apply_physics(new_offset);
        self.is_scrolling = true;

        // Update velocity for momentum
        let dt = self.last_update.elapsed().as_secs_f32();
        if dt > 0.0 && self.momentum_enabled {
            self.velocity = Vector2::new(delta.x / dt, delta.y / dt);
        }

        self.last_update = Instant::now();
    }

    /// Apply momentum scrolling
    pub fn update_momentum(&mut self, dt: f32) {
        if !self.momentum_enabled || self.velocity.x.abs() < 0.1 && self.velocity.y.abs() < 0.1 {
            self.velocity = Vector2::ZERO;
            self.is_scrolling = false;
            return;
        }

        // Apply friction
        let friction = 0.95;
        self.velocity.x *= friction;
        self.velocity.y *= friction;

        // Apply velocity
        let delta = Vector2::new(
            self.velocity.x * dt,
            self.velocity.y * dt,
        );

        let new_offset = Vector2::new(
            self.offset.x + delta.x,
            self.offset.y + delta.y,
        );

        self.offset = self.apply_physics(new_offset);
    }

    fn apply_physics(&self, offset: Vector2) -> Vector2 {
        match self.physics {
            ScrollPhysics::Clamping => {
                Vector2::new(
                    offset.x.clamp(0.0, self.max_offset.x),
                    offset.y.clamp(0.0, self.max_offset.y),
                )
            }
            ScrollPhysics::Bouncing => {
                // Allow overscroll with resistance
                let overscroll_resistance = 0.3;

                let x = if offset.x < 0.0 {
                    offset.x * overscroll_resistance
                } else if offset.x > self.max_offset.x {
                    self.max_offset.x + (offset.x - self.max_offset.x) * overscroll_resistance
                } else {
                    offset.x
                };

                let y = if offset.y < 0.0 {
                    offset.y * overscroll_resistance
                } else if offset.y > self.max_offset.y {
                    self.max_offset.y + (offset.y - self.max_offset.y) * overscroll_resistance
                } else {
                    offset.y
                };

                Vector2::new(x, y)
            }
            ScrollPhysics::Never => offset,
        }
    }

    /// Animate to specific position
    pub fn animate_to(&mut self, target: Vector2, _duration: Duration) {
        // Would use animation system
        self.offset = target;
    }

    /// Jump to position immediately
    pub fn jump_to(&mut self, position: Vector2) {
        self.offset = self.apply_physics(position);
        self.velocity = Vector2::ZERO;
    }

    /// Set content size to calculate max offset
    pub fn set_content_size(&mut self, content_size: Vector2, viewport_size: Vector2) {
        self.max_offset = Vector2::new(
            (content_size.x - viewport_size.x).max(0.0),
            (content_size.y - viewport_size.y).max(0.0),
        );
    }

    pub fn is_scrolling(&self) -> bool {
        self.is_scrolling
    }

    pub fn stop(&mut self) {
        self.velocity = Vector2::ZERO;
        self.is_scrolling = false;
    }
}

impl Default for ScrollController {
    fn default() -> Self {
        Self::new()
    }
}

/// Snap point for scroll snapping
#[derive(Debug, Clone, Copy)]
pub struct SnapPoint {
    pub offset: f32,
    pub strength: f32, // 0.0 to 1.0
}

/// Scroll snap controller
pub struct ScrollSnapController {
    pub snap_points: Vec<SnapPoint>,
    pub snap_threshold: f32,
    pub axis: SnapAxis,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnapAxis {
    Horizontal,
    Vertical,
    Both,
}

impl ScrollSnapController {
    pub fn new(axis: SnapAxis) -> Self {
        Self {
            snap_points: Vec::new(),
            snap_threshold: 50.0,
            axis,
        }
    }

    /// Find nearest snap point
    pub fn find_snap_point(&self, current_offset: Vector2) -> Option<Vector2> {
        if self.snap_points.is_empty() {
            return None;
        }

        let offset = match self.axis {
            SnapAxis::Horizontal => current_offset.x,
            SnapAxis::Vertical => current_offset.y,
            SnapAxis::Both => current_offset.x, // Simplified
        };

        let mut nearest: Option<&SnapPoint> = None;
        let mut min_distance = f32::INFINITY;

        for snap in &self.snap_points {
            let distance = (snap.offset - offset).abs();
            if distance < min_distance && distance < self.snap_threshold {
                min_distance = distance;
                nearest = Some(snap);
            }
        }

        nearest.map(|snap| match self.axis {
            SnapAxis::Horizontal => Vector2::new(snap.offset, current_offset.y),
            SnapAxis::Vertical => Vector2::new(current_offset.x, snap.offset),
            SnapAxis::Both => Vector2::new(snap.offset, snap.offset),
        })
    }

    pub fn add_snap_point(&mut self, point: SnapPoint) {
        self.snap_points.push(point);
        self.snap_points.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());
    }
}

/// Clipping rectangle manager
pub struct ClipManager {
    clip_stack: Vec<Rect>,
}

impl ClipManager {
    pub fn new() -> Self {
        Self {
            clip_stack: Vec::new(),
        }
    }

    /// Push a clip rect
    pub fn push_clip(&mut self, rect: Rect) {
        if let Some(current) = self.clip_stack.last() {
            // Intersect with current clip
            let intersected = self.intersect_rects(*current, rect);
            self.clip_stack.push(intersected);
        } else {
            self.clip_stack.push(rect);
        }
    }

    /// Pop the current clip
    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    /// Get current clip rect
    pub fn current_clip(&self) -> Option<Rect> {
        self.clip_stack.last().copied()
    }

    /// Check if point is clipped
    pub fn is_clipped(&self, point: Point) -> bool {
        if let Some(clip) = self.current_clip() {
            !clip.contains(point.x, point.y)
        } else {
            false
        }
    }

    /// Check if rect is clipped
    pub fn is_rect_clipped(&self, rect: Rect) -> bool {
        if let Some(clip) = self.current_clip() {
            // Check if completely outside
            rect.x + rect.width < clip.x ||
                rect.x > clip.x + clip.width ||
                rect.y + rect.height < clip.y ||
                rect.y > clip.y + clip.height
        } else {
            false
        }
    }

    fn intersect_rects(&self, a: Rect, b: Rect) -> Rect {
        let x = a.x.max(b.x);
        let y = a.y.max(b.y);
        let width = (a.x + a.width).min(b.x + b.width) - x;
        let height = (a.y + a.height).min(b.y + b.height) - y;

        Rect::new(x, y, width.max(0.0), height.max(0.0))
    }

    pub fn clear(&mut self) {
        self.clip_stack.clear();
    }
}

impl Default for ClipManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Virtual scrolling for large lists
pub struct VirtualScroller {
    pub item_height: f32,
    pub viewport_height: f32,
    pub total_items: usize,
    pub buffer_size: usize,
}

impl VirtualScroller {
    pub fn new(item_height: f32, viewport_height: f32) -> Self {
        Self {
            item_height,
            viewport_height,
            total_items: 0,
            buffer_size: 3,
        }
    }

    /// Calculate which items are visible
    pub fn visible_range(&self, scroll_offset: f32) -> (usize, usize) {
        let start_index = (scroll_offset / self.item_height).floor() as usize;
        let visible_count = (self.viewport_height / self.item_height).ceil() as usize;

        let start = start_index.saturating_sub(self.buffer_size);
        let end = (start_index + visible_count + self.buffer_size).min(self.total_items);

        (start, end)
    }

    /// Get total content height
    pub fn content_height(&self) -> f32 {
        self.item_height * self.total_items as f32
    }

    /// Get item position
    pub fn item_position(&self, index: usize) -> f32 {
        index as f32 * self.item_height
    }

    pub fn set_total_items(&mut self, count: usize) {
        self.total_items = count;
    }
}
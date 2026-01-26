//! Complete animation system with curves, springs, and keyframes
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fmt;

/// Animation ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnimationId(u64);

static ANIMATION_ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

impl AnimationId {
    pub fn new() -> Self {
        AnimationId(ANIMATION_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
    }
}

/// Easing curve for animations
#[derive(Debug, Clone, Copy)]
pub enum EasingCurve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Cubic(f32, f32, f32, f32), // Bezier control points
    Spring { damping: f32, stiffness: f32 },
}

impl EasingCurve {
    pub fn evaluate(&self, t: f32) -> f32 {
        match self {
            EasingCurve::Linear => t,
            EasingCurve::EaseIn => t * t,
            EasingCurve::EaseOut => t * (2.0 - t),
            EasingCurve::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingCurve::Cubic(x1, y1, x2, y2) => {
                self.cubic_bezier(t, *x1, *y1, *x2, *y2)
            }
            EasingCurve::Spring { damping, stiffness } => {
                self.spring_evaluation(t, *damping, *stiffness)
            }
        }
    }

    fn cubic_bezier(&self, t: f32, _x1: f32, y1: f32, _x2: f32, y2: f32) -> f32 {
        // Simplified cubic bezier calculation
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        // Only using y control points for the value
        mt3 * 0.0 + 3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3 * 1.0
    }

    fn spring_evaluation(&self, t: f32, damping: f32, stiffness: f32) -> f32 {
        let omega = stiffness.sqrt();
        let zeta = damping / (2.0 * omega);

        if zeta < 1.0 {
            // Underdamped
            let omega_d = omega * (1.0 - zeta * zeta).sqrt();
            let a = (-zeta * omega * t).exp();
            let b = (omega_d * t).cos();
            let c = (zeta / (1.0 - zeta * zeta).sqrt()) * (omega_d * t).sin();
            1.0 - a * (b + c)
        } else {
            // Critically damped or overdamped
            let a = (-omega * t).exp();
            1.0 - a * (1.0 + omega * t)
        }
    }
}

/// Animated value
#[derive(Debug, Clone)]
pub struct AnimatedValue<T> {
    pub start: T,
    pub end: T,
    pub current: T,
}

impl<T: Interpolate> AnimatedValue<T> {
    pub fn new(start: T, end: T) -> Self {
        Self {
            current: start.clone(),
            start,
            end,
        }
    }

    pub fn update(&mut self, t: f32) {
        self.current = self.start.interpolate(&self.end, t);
    }
}

/// Trait for types that can be interpolated
pub trait Interpolate: Clone {
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}

impl Interpolate for f32 {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Interpolate for (f32, f32) {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        (
            self.0 + (other.0 - self.0) * t,
            self.1 + (other.1 - self.1) * t,
        )
    }
}

impl Interpolate for crate::core::Color {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        crate::core::Color::rgba(
            (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
            (self.a as f32 + (other.a as f32 - self.a as f32) * t) as u8,
        )
    }
}

/// Animation state
#[derive(Clone)]
pub struct Animation<T: Interpolate> {
    pub id: AnimationId,
    pub value: AnimatedValue<T>,
    pub duration: Duration,
    pub curve: EasingCurve,
    pub start_time: Instant,
    pub repeat: AnimationRepeat,
    pub on_complete: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationRepeat {
    Once,
    Loop,
    Reverse,
    Count(usize),
}

impl<T: Interpolate> Animation<T> {
    pub fn new(start: T, end: T, duration: Duration) -> Self {
        Self {
            id: AnimationId::new(),
            value: AnimatedValue::new(start, end),
            duration,
            curve: EasingCurve::Linear,
            start_time: Instant::now(),
            repeat: AnimationRepeat::Once,
            on_complete: None,
        }
    }

    pub fn with_curve(mut self, curve: EasingCurve) -> Self {
        self.curve = curve;
        self
    }

    pub fn with_repeat(mut self, repeat: AnimationRepeat) -> Self {
        self.repeat = repeat;
        self
    }

    pub fn with_on_complete<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_complete = Some(std::sync::Arc::new(callback));
        self
    }

    pub fn update(&mut self) -> bool {
        let elapsed = self.start_time.elapsed();
        let t = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        let eased_t = self.curve.evaluate(t);
        self.value.update(eased_t);

        if t >= 1.0 {
            match self.repeat {
                AnimationRepeat::Once => {
                    if let Some(callback) = &self.on_complete {
                        callback();
                    }
                    return false; // Animation complete
                }
                AnimationRepeat::Loop => {
                    self.start_time = Instant::now();
                }
                AnimationRepeat::Reverse => {
                    std::mem::swap(&mut self.value.start, &mut self.value.end);
                    self.start_time = Instant::now();
                }
                AnimationRepeat::Count(n) if n > 1 => {
                    self.repeat = AnimationRepeat::Count(n - 1);
                    self.start_time = Instant::now();
                }
                AnimationRepeat::Count(_) => {
                    if let Some(callback) = &self.on_complete {
                        callback();
                    }
                    return false;
                }
            }
        }
        true // Animation continues
    }

    pub fn current_value(&self) -> &T {
        &self.value.current
    }
}

// Manual Debug implementation that doesn't require Debug for on_complete
impl<T: Interpolate + fmt::Debug> fmt::Debug for Animation<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Animation")
            .field("id", &self.id)
            .field("value", &self.value)
            .field("duration", &self.duration)
            .field("curve", &self.curve)
            .field("start_time", &self.start_time)
            .field("repeat", &self.repeat)
            .finish()
    }
}

/// Animation controller
pub struct AnimationController<T: Interpolate> {
    animations: HashMap<AnimationId, Animation<T>>,
}

impl<T: Interpolate> AnimationController<T> {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }

    pub fn add(&mut self, animation: Animation<T>) -> AnimationId {
        let id = animation.id;
        self.animations.insert(id, animation);
        id
    }

    pub fn remove(&mut self, id: AnimationId) {
        self.animations.remove(&id);
    }

    pub fn update_all(&mut self) {
        self.animations.retain(|_, anim| anim.update());
    }

    pub fn get(&self, id: AnimationId) -> Option<&Animation<T>> {
        self.animations.get(&id)
    }

    pub fn get_mut(&mut self, id: AnimationId) -> Option<&mut Animation<T>> {
        self.animations.get_mut(&id)
    }

    pub fn clear(&mut self) {
        self.animations.clear();
    }
}

impl<T: Interpolate> Default for AnimationController<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Keyframe animation
#[derive(Debug, Clone)]
pub struct Keyframe<T: Clone> {
    pub time: f32, // 0.0 to 1.0
    pub value: T,
    pub curve: EasingCurve,
}

pub struct KeyframeAnimation<T: Interpolate + Clone> {
    pub id: AnimationId,
    pub keyframes: Vec<Keyframe<T>>,
    pub duration: Duration,
    pub start_time: Instant,
    pub current_value: T,
}

impl<T: Interpolate + Clone> KeyframeAnimation<T> {
    pub fn new(keyframes: Vec<Keyframe<T>>, duration: Duration) -> Self {
        let current_value = keyframes.first().unwrap().value.clone();
        Self {
            id: AnimationId::new(),
            keyframes,
            duration,
            start_time: Instant::now(),
            current_value,
        }
    }

    pub fn update(&mut self) -> bool {
        let elapsed = self.start_time.elapsed();
        let t = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);

        // Find surrounding keyframes
        let mut prev_kf = &self.keyframes[0];
        let mut next_kf = &self.keyframes[0];

        for kf in &self.keyframes {
            if kf.time <= t {
                prev_kf = kf;
            }
            if kf.time >= t {
                next_kf = kf;
                break;
            }
        }

        // Interpolate between keyframes
        if prev_kf.time == next_kf.time {
            self.current_value = prev_kf.value.clone();
        } else {
            let segment_t = (t - prev_kf.time) / (next_kf.time - prev_kf.time);
            let eased_t = prev_kf.curve.evaluate(segment_t);
            self.current_value = prev_kf.value.interpolate(&next_kf.value, eased_t);
        }

        t < 1.0
    }
}

/// Transition builder for implicit animations
pub struct TransitionBuilder<T: Interpolate> {
    value: AnimatedValue<T>,
    duration: Duration,
    curve: EasingCurve,
}

impl<T: Interpolate> TransitionBuilder<T> {
    pub fn new(from: T, to: T) -> Self {
        Self {
            value: AnimatedValue::new(from, to),
            duration: Duration::from_millis(300),
            curve: EasingCurve::EaseInOut,
        }
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn curve(mut self, curve: EasingCurve) -> Self {
        self.curve = curve;
        self
    }

    pub fn build(self) -> Animation<T> {
        Animation::new(self.value.start, self.value.end, self.duration)
            .with_curve(self.curve)
    }
}
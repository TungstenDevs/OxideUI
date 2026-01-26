//! Layout constraints system for OxideUI
//!
//! Implements a constraint-based layout model similar to Flutter's BoxConstraints.
//! Parent passes constraints down, child measures itself, returns size up.

/// Size in logical pixels
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub const fn zero() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }

    pub const fn infinite() -> Self {
        Self {
            width: f32::INFINITY,
            height: f32::INFINITY,
        }
    }

    pub fn constrain(&self, constraints: &Constraints) -> Self {
        Self {
            width: self
                .width
                .clamp(constraints.min_width, constraints.max_width),
            height: self
                .height
                .clamp(constraints.min_height, constraints.max_height),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::zero()
    }
}

/// Layout constraints - defines the range of acceptable sizes
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Constraints {
    /// Create tight constraints (fixed size)
    pub const fn tight(size: Size) -> Self {
        Self {
            min_width: size.width,
            max_width: size.width,
            min_height: size.height,
            max_height: size.height,
        }
    }

    /// Create loose constraints (0 to specified max)
    pub const fn loose(max_size: Size) -> Self {
        Self {
            min_width: 0.0,
            max_width: max_size.width,
            min_height: 0.0,
            max_height: max_size.height,
        }
    }

    /// Create unbounded constraints
    pub const fn unbounded() -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }

    /// Create unconstrained constraints (for root)
    pub const fn unconstrained() -> Self {
        Self::unbounded()
    }

    /// Create constraints with specific bounds
    pub const fn new(min_width: f32, max_width: f32, min_height: f32, max_height: f32) -> Self {
        Self {
            min_width,
            max_width,
            min_height,
            max_height,
        }
    }

    /// Check if width is bounded
    pub fn has_bounded_width(&self) -> bool {
        self.max_width.is_finite()
    }

    /// Check if height is bounded
    pub fn has_bounded_height(&self) -> bool {
        self.max_height.is_finite()
    }

    /// Check if constraints are tight (fixed size)
    pub fn is_tight(&self) -> bool {
        self.min_width == self.max_width && self.min_height == self.max_height
    }

    /// Get the biggest size that satisfies these constraints
    pub fn biggest(&self) -> Size {
        Size::new(
            if self.max_width.is_finite() {
                self.max_width
            } else {
                0.0
            },
            if self.max_height.is_finite() {
                self.max_height
            } else {
                0.0
            },
        )
    }

    /// Get the smallest size that satisfies these constraints
    pub fn smallest(&self) -> Size {
        Size::new(self.min_width, self.min_height)
    }

    /// Constrain the given size to fit within these constraints
    pub fn constrain(&self, size: Size) -> Size {
        Size::new(
            size.width.clamp(self.min_width, self.max_width),
            size.height.clamp(self.min_height, self.max_height),
        )
    }

    /// Create new constraints with width constrained
    pub fn constrain_width(&self, width: f32) -> Self {
        Self {
            min_width: width.max(self.min_width).min(self.max_width),
            max_width: width.max(self.min_width).min(self.max_width),
            ..*self
        }
    }

    /// Create new constraints with height constrained
    pub fn constrain_height(&self, height: f32) -> Self {
        Self {
            min_height: height.max(self.min_height).min(self.max_height),
            max_height: height.max(self.min_height).min(self.max_height),
            ..*self
        }
    }

    /// Deflate constraints by the given amount
    pub fn deflate(&self, amount: EdgeInsets) -> Self {
        let horizontal = amount.left + amount.right;
        let vertical = amount.top + amount.bottom;

        Self {
            min_width: (self.min_width - horizontal).max(0.0),
            max_width: (self.max_width - horizontal).max(0.0),
            min_height: (self.min_height - vertical).max(0.0),
            max_height: (self.max_height - vertical).max(0.0),
        }
    }

    /// Loosen the constraints (allow 0 as minimum)
    pub fn loosen(&self) -> Self {
        Self {
            min_width: 0.0,
            min_height: 0.0,
            ..*self
        }
    }

    /// Tighten to a specific size
    pub fn tighten(&self, size: Size) -> Self {
        Self::tight(size)
    }
}

impl Default for Constraints {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Edge insets for padding/margin
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeInsets {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl EdgeInsets {
    pub const fn all(value: f32) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }

    pub const fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        }
    }

    pub const fn only(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub const fn zero() -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }

    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl Default for EdgeInsets {
    fn default() -> Self {
        Self::zero()
    }
}

/// Alignment options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Alignment {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Alignment {
    pub fn align(&self, size: Size, container_size: Size) -> (f32, f32) {
        let x = match self {
            Alignment::TopLeft | Alignment::CenterLeft | Alignment::BottomLeft => 0.0,
            Alignment::TopCenter | Alignment::Center | Alignment::BottomCenter => {
                (container_size.width - size.width) / 2.0
            }
            Alignment::TopRight | Alignment::CenterRight | Alignment::BottomRight => {
                container_size.width - size.width
            }
        };

        let y = match self {
            Alignment::TopLeft | Alignment::TopCenter | Alignment::TopRight => 0.0,
            Alignment::CenterLeft | Alignment::Center | Alignment::CenterRight => {
                (container_size.height - size.height) / 2.0
            }
            Alignment::BottomLeft | Alignment::BottomCenter | Alignment::BottomRight => {
                container_size.height - size.height
            }
        };

        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraints_tight() {
        let constraints = Constraints::tight(Size::new(100.0, 200.0));
        assert_eq!(constraints.min_width, 100.0);
        assert_eq!(constraints.max_width, 100.0);
        assert_eq!(constraints.min_height, 200.0);
        assert_eq!(constraints.max_height, 200.0);
        assert!(constraints.is_tight());
    }

    #[test]
    fn test_constraints_loose() {
        let constraints = Constraints::loose(Size::new(100.0, 200.0));
        assert_eq!(constraints.min_width, 0.0);
        assert_eq!(constraints.max_width, 100.0);
        assert_eq!(constraints.min_height, 0.0);
        assert_eq!(constraints.max_height, 200.0);
        assert!(!constraints.is_tight());
    }

    #[test]
    fn test_size_constrain() {
        let constraints = Constraints::new(10.0, 100.0, 20.0, 200.0);
        let size = Size::new(150.0, 250.0);
        let constrained = constraints.constrain(size);
        assert_eq!(constrained.width, 100.0);
        assert_eq!(constrained.height, 200.0);
    }
}

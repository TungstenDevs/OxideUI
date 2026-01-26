use crate::layout::constraints::Size;

/// A color in RGBA format
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn with_alpha(&self, alpha: u8) -> Self {
        Color::rgba(self.r, self.g, self.b, alpha)
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }

    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);
}

/// 2D point
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Point = Point::new(0.0, 0.0);
}

/// Rectangle - OUR custom rect type
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_size(size: Size) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: size.width,
            height: size.height,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Convert to skia_safe::Rect
    pub fn to_skia_rect(&self) -> skia_safe::Rect {
        skia_safe::Rect::from_xywh(self.x, self.y, self.width, self.height)
    }
}

/// Text style configuration
#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
    pub font_family: String,
    pub font_size: f32,
    pub color: Color,
    pub bold: bool,
    pub italic: bool,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_family: "sans-serif".to_string(),
            font_size: 16.0,
            color: Color::BLACK,
            bold: false,
            italic: false,
        }
    }
}

/// Paint style for drawing operations
#[derive(Clone, Debug, PartialEq)]
pub struct Paint {
    pub color: Color,
    pub stroke_width: f32,
    pub anti_alias: bool,
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            stroke_width: 1.0,
            anti_alias: true,
        }
    }
}

/// 2D transformation matrix
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix {
    pub values: [[f32; 3]; 3],
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            values: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn translate(x: f32, y: f32) -> Self {
        Self {
            values: [[1.0, 0.0, x], [0.0, 1.0, y], [0.0, 0.0, 1.0]],
        }
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            values: [[sx, 0.0, 0.0], [0.0, sy, 0.0], [0.0, 0.0, 1.0]],
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

/// Backend-agnostic rendering primitives
#[derive(Clone, Debug, PartialEq)]
pub enum RenderObject {
    Rect { rect: Rect, paint: Paint },
    Text { content: String, style: TextStyle, position: Point },
    Image { size: Size },
    Clip { rect: Rect, child: Box<RenderObject> },
    Transform { matrix: Matrix, child: Box<RenderObject> },
    Group { children: Vec<RenderObject> },
    None,
}

impl RenderObject {
    pub fn rect(rect: Rect, color: Color) -> Self {
        RenderObject::Rect {
            rect,
            paint: Paint {
                color,
                ..Default::default()
            },
        }
    }

    pub fn text(content: String, style: TextStyle, position: Point) -> Self {
        RenderObject::Text { content, style, position }
    }

    pub fn transform(matrix: Matrix, child: RenderObject) -> Self {
        RenderObject::Transform {
            matrix,
            child: Box::new(child),
        }
    }

    pub fn clip(rect: Rect, child: RenderObject) -> Self {
        RenderObject::Clip {
            rect,
            child: Box::new(child),
        }
    }

    pub fn group(children: Vec<RenderObject>) -> Self {
        RenderObject::Group { children }
    }
}
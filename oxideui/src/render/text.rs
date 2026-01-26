use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::core::render_object::{TextStyle};

/// Font weight enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

/// Font style enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Font descriptor for loading and caching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontDescriptor {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
}

impl FontDescriptor {
    pub fn new(family: impl Into<String>) -> Self {
        Self {
            family: family.into(),
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
        }
    }

    pub fn bold(mut self) -> Self {
        self.weight = FontWeight::Bold;
        self
    }

    pub fn italic(mut self) -> Self {
        self.style = FontStyle::Italic;
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }
}

/// Glyph information
#[derive(Debug, Clone)]
pub struct GlyphInfo {
    pub glyph_id: u32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub x_advance: f32,
    pub y_advance: f32,
}

/// Shaped text result
#[derive(Debug, Clone)]
pub struct ShapedText {
    pub glyphs: Vec<GlyphInfo>,
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
}

/// Text metrics
#[derive(Debug, Clone, Copy)]
pub struct TextMetrics {
    pub width: f32,
    pub height: f32,
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
}

/// Font manager for loading and caching fonts
pub struct FontManager {
    font_cache: Arc<RwLock<HashMap<FontDescriptor, Vec<u8>>>>,
    system_fonts: Vec<String>,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            font_cache: Arc::new(RwLock::new(HashMap::new())),
            system_fonts: Self::enumerate_system_fonts(),
        }
    }

    fn enumerate_system_fonts() -> Vec<String> {
        // Platform-specific font enumeration
        #[cfg(target_os = "linux")]
        {
            vec![
                "DejaVu Sans".to_string(),
                "Liberation Sans".to_string(),
                "Ubuntu".to_string(),
                "Noto Sans".to_string(),
            ]
        }

        #[cfg(target_os = "macos")]
        {
            vec![
                "SF Pro Display".to_string(),
                "Helvetica Neue".to_string(),
                "Arial".to_string(),
            ]
        }

        #[cfg(target_os = "windows")]
        {
            vec![
                "Segoe UI".to_string(),
                "Arial".to_string(),
                "Tahoma".to_string(),
            ]
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            vec!["sans-serif".to_string()]
        }
    }

    pub fn load_font(&self, descriptor: &FontDescriptor) -> Result<Vec<u8>> {
        // Check cache first
        {
            let cache = self.font_cache.read();
            if let Some(data) = cache.get(descriptor) {
                return Ok(data.clone());
            }
        }

        // Try to load from system
        let data = self.load_system_font(descriptor)?;

        // Cache it
        self.font_cache.write().insert(descriptor.clone(), data.clone());

        Ok(data)
    }

    fn load_system_font(&self, descriptor: &FontDescriptor) -> Result<Vec<u8>> {
        // Platform-specific font loading
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let paths = vec![
                format!("/usr/share/fonts/truetype/{}.ttf", descriptor.family.to_lowercase().replace(" ", "-")),
                format!("/usr/share/fonts/TTF/{}.ttf", descriptor.family),
                format!("/usr/local/share/fonts/{}.ttf", descriptor.family),
            ];

            for path in paths {
                if let Ok(data) = fs::read(&path) {
                    return Ok(data);
                }
            }
        }

        Err(anyhow!("Font not found: {}", descriptor.family))
    }

    pub fn measure_text(&self, text: &str, style: &TextStyle) -> Result<TextMetrics> {
        // Simplified measurement - in production, use HarfBuzz or similar
        let char_count = text.chars().count();
        let avg_char_width = style.font_size * 0.6;

        Ok(TextMetrics {
            width: avg_char_width * char_count as f32,
            height: style.font_size * 1.2,
            ascent: style.font_size * 0.8,
            descent: style.font_size * 0.2,
            line_gap: style.font_size * 0.2,
        })
    }

    pub fn shape_text(&self, text: &str, style: &TextStyle) -> Result<ShapedText> {
        // Simplified shaping - production should use HarfBuzz
        let metrics = self.measure_text(text, style)?;

        let mut glyphs = Vec::new();
        let mut x_pos = 0.0;

        for (i, _ch) in text.chars().enumerate() {
            glyphs.push(GlyphInfo {
                glyph_id: i as u32,
                x_offset: x_pos,
                y_offset: 0.0,
                x_advance: metrics.width / text.chars().count() as f32,
                y_advance: 0.0,
            });
            x_pos += metrics.width / text.chars().count() as f32;
        }

        Ok(ShapedText {
            glyphs,
            width: metrics.width,
            height: metrics.height,
            baseline: metrics.ascent,
        })
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Text layout engine for multi-line text
pub struct TextLayout {
    font_manager: Arc<FontManager>,
}

impl TextLayout {
    pub fn new(font_manager: Arc<FontManager>) -> Self {
        Self { font_manager }
    }

    pub fn layout_text(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f32>,
    ) -> Result<Vec<ShapedText>> {
        if let Some(max_width) = max_width {
            self.layout_multiline(text, style, max_width)
        } else {
            Ok(vec![self.font_manager.shape_text(text, style)?])
        }
    }

    fn layout_multiline(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: f32,
    ) -> Result<Vec<ShapedText>> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0.0;

        for word in text.split_whitespace() {
            let word_metrics = self.font_manager.measure_text(word, style)?;

            if current_width + word_metrics.width > max_width && !current_line.is_empty() {
                // Start new line
                lines.push(self.font_manager.shape_text(&current_line, style)?);
                current_line.clear();
                current_width = 0.0;
            }

            if !current_line.is_empty() {
                current_line.push(' ');
                current_width += word_metrics.width / word.len() as f32; // Space width approximation
            }

            current_line.push_str(word);
            current_width += word_metrics.width;
        }

        if !current_line.is_empty() {
            lines.push(self.font_manager.shape_text(&current_line, style)?);
        }

        Ok(lines)
    }
}

/// Text rendering cache for performance
pub struct TextCache {
    cache: Arc<RwLock<HashMap<String, ShapedText>>>,
}

impl TextCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_or_shape(
        &self,
        text: &str,
        style: &TextStyle,
        font_manager: &FontManager,
    ) -> Result<ShapedText> {
        let cache_key = format!("{}:{}:{}", text, style.font_family, style.font_size);

        // Check cache
        {
            let cache = self.cache.read();
            if let Some(shaped) = cache.get(&cache_key) {
                return Ok(shaped.clone());
            }
        }

        // Shape and cache
        let shaped = font_manager.shape_text(text, style)?;
        self.cache.write().insert(cache_key, shaped.clone());

        Ok(shaped)
    }

    pub fn clear(&self) {
        self.cache.write().clear();
    }
}

impl Default for TextCache {
    fn default() -> Self {
        Self::new()
    }
}
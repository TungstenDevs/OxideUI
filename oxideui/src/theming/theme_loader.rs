use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;
use anyhow::{Result, Context};
use crate::core::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub light: ThemeColors,
    pub dark: ThemeColors,
    #[serde(default)]
    pub css_variables: HashMap<String, String>,
    #[serde(default = "default_font_sans")]
    pub font_sans: String,
    #[serde(default = "default_font_mono")]
    pub font_mono: String,
    #[serde(default = "default_radius")]
    pub radius: f32,
    #[serde(default)]
    pub is_dark: bool,
}

fn default_font_sans() -> String {
    "Inter".to_string()
}

fn default_font_mono() -> String {
    "JetBrains Mono".to_string()
}

fn default_radius() -> f32 {
    0.5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    // Background
    pub background: [u8; 3],
    pub foreground: [u8; 3],

    // Cards
    pub card: [u8; 3],
    pub card_foreground: [u8; 3],

    // Popover
    pub popover: [u8; 3],
    pub popover_foreground: [u8; 3],

    // Primary
    pub primary: [u8; 3],
    pub primary_foreground: [u8; 3],

    // Secondary
    pub secondary: [u8; 3],
    pub secondary_foreground: [u8; 3],

    // Muted
    pub muted: [u8; 3],
    pub muted_foreground: [u8; 3],

    // Accent
    pub accent: [u8; 3],
    pub accent_foreground: [u8; 3],

    // Destructive
    pub destructive: [u8; 3],
    pub destructive_foreground: [u8; 3],

    // Borders & Inputs
    pub border: [u8; 3],
    pub input: [u8; 3],
    pub ring: [u8; 3],

    // Charts
    #[serde(default = "default_charts")]
    pub chart_1: [u8; 3],
    #[serde(default = "default_charts")]
    pub chart_2: [u8; 3],
    #[serde(default = "default_charts")]
    pub chart_3: [u8; 3],
    #[serde(default = "default_charts")]
    pub chart_4: [u8; 3],
    #[serde(default = "default_charts")]
    pub chart_5: [u8; 3],

    // Sidebar (Radix UI inspired)
    #[serde(default)]
    pub sidebar: [u8; 3],
    #[serde(default)]
    pub sidebar_foreground: [u8; 3],
    #[serde(default)]
    pub sidebar_primary: [u8; 3],
    #[serde(default)]
    pub sidebar_primary_foreground: [u8; 3],
    #[serde(default)]
    pub sidebar_accent: [u8; 3],
    #[serde(default)]
    pub sidebar_accent_foreground: [u8; 3],
    #[serde(default)]
    pub sidebar_border: [u8; 3],
    #[serde(default)]
    pub sidebar_ring: [u8; 3],

    // Shadows (like shadcn)
    #[serde(default = "default_shadow_x")]
    pub shadow_x: f32,
    #[serde(default = "default_shadow_y")]
    pub shadow_y: f32,
    #[serde(default = "default_shadow_blur")]
    pub shadow_blur: f32,
    #[serde(default = "default_shadow_spread")]
    pub shadow_spread: f32,
    #[serde(default = "default_shadow_opacity")]
    pub shadow_opacity: f32,
}

fn default_charts() -> [u8; 3] {
    [0, 0, 0]
}

fn default_shadow_x() -> f32 { 0.0 }
fn default_shadow_y() -> f32 { 1.0 }
fn default_shadow_blur() -> f32 { 4.0 }
fn default_shadow_spread() -> f32 { 0.0 }
fn default_shadow_opacity() -> f32 { 0.05 }

impl ThemeConfig {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read theme file: {}", path))?;
        let mut theme: Self = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse theme JSON: {}", path))?;

        // Auto-calculate sidebar colors if not provided
        theme.calculate_sidebar_colors();

        Ok(theme)
    }

    fn calculate_sidebar_colors(&mut self) {
        // If sidebar colors aren't set, calculate them from other colors
        if self.light.sidebar == [0, 0, 0] {
            // Light sidebar is slightly darker than background
            self.light.sidebar = self.light.muted;
            self.light.sidebar_foreground = self.light.foreground;
            self.light.sidebar_primary = self.light.primary;
            self.light.sidebar_primary_foreground = self.light.primary_foreground;
            self.light.sidebar_accent = self.light.accent;
            self.light.sidebar_accent_foreground = self.light.accent_foreground;
            self.light.sidebar_border = self.light.border;
            self.light.sidebar_ring = self.light.ring;
        }

        if self.dark.sidebar == [0, 0, 0] {
            // Dark sidebar is slightly lighter than background
            let mut sidebar = self.dark.background;
            for i in 0..3 {
                sidebar[i] = sidebar[i].saturating_add(10);
            }
            self.dark.sidebar = sidebar;
            self.dark.sidebar_foreground = self.dark.foreground;
            self.dark.sidebar_primary = self.dark.primary;
            self.dark.sidebar_primary_foreground = self.dark.primary_foreground;
            self.dark.sidebar_accent = self.dark.accent;
            self.dark.sidebar_accent_foreground = self.dark.accent_foreground;
            self.dark.sidebar_border = self.dark.border;
            self.dark.sidebar_ring = self.dark.ring;
        }
    }

    pub fn to_css_variables(&self, is_dark: bool) -> String {
        let colors = if is_dark { &self.dark } else { &self.light };

        let mut css = String::from(":root {\n");

        // Background colors
        css.push_str(&format!("  --background: {} {} {};\n", colors.background[0], colors.background[1], colors.background[2]));
        css.push_str(&format!("  --foreground: {} {} {};\n", colors.foreground[0], colors.foreground[1], colors.foreground[2]));

        // Card colors
        css.push_str(&format!("  --card: {} {} {};\n", colors.card[0], colors.card[1], colors.card[2]));
        css.push_str(&format!("  --card-foreground: {} {} {};\n", colors.card_foreground[0], colors.card_foreground[1], colors.card_foreground[2]));

        // Primary colors
        css.push_str(&format!("  --primary: {} {} {};\n", colors.primary[0], colors.primary[1], colors.primary[2]));
        css.push_str(&format!("  --primary-foreground: {} {} {};\n", colors.primary_foreground[0], colors.primary_foreground[1], colors.primary_foreground[2]));

        // Secondary colors
        css.push_str(&format!("  --secondary: {} {} {};\n", colors.secondary[0], colors.secondary[1], colors.secondary[2]));
        css.push_str(&format!("  --secondary-foreground: {} {} {};\n", colors.secondary_foreground[0], colors.secondary_foreground[1], colors.secondary_foreground[2]));

        // Muted colors
        css.push_str(&format!("  --muted: {} {} {};\n", colors.muted[0], colors.muted[1], colors.muted[2]));
        css.push_str(&format!("  --muted-foreground: {} {} {};\n", colors.muted_foreground[0], colors.muted_foreground[1], colors.muted_foreground[2]));

        // Accent colors
        css.push_str(&format!("  --accent: {} {} {};\n", colors.accent[0], colors.accent[1], colors.accent[2]));
        css.push_str(&format!("  --accent-foreground: {} {} {};\n", colors.accent_foreground[0], colors.accent_foreground[1], colors.accent_foreground[2]));

        // Destructive colors
        css.push_str(&format!("  --destructive: {} {} {};\n", colors.destructive[0], colors.destructive[1], colors.destructive[2]));
        css.push_str(&format!("  --destructive-foreground: {} {} {};\n", colors.destructive_foreground[0], colors.destructive_foreground[1], colors.destructive_foreground[2]));

        // Borders & Inputs
        css.push_str(&format!("  --border: {} {} {};\n", colors.border[0], colors.border[1], colors.border[2]));
        css.push_str(&format!("  --input: {} {} {};\n", colors.input[0], colors.input[1], colors.input[2]));
        css.push_str(&format!("  --ring: {} {} {};\n", colors.ring[0], colors.ring[1], colors.ring[2]));

        // Sidebar colors
        css.push_str(&format!("  --sidebar: {} {} {};\n", colors.sidebar[0], colors.sidebar[1], colors.sidebar[2]));
        css.push_str(&format!("  --sidebar-foreground: {} {} {};\n", colors.sidebar_foreground[0], colors.sidebar_foreground[1], colors.sidebar_foreground[2]));
        css.push_str(&format!("  --sidebar-primary: {} {} {};\n", colors.sidebar_primary[0], colors.sidebar_primary[1], colors.sidebar_primary[2]));
        css.push_str(&format!("  --sidebar-primary-foreground: {} {} {};\n", colors.sidebar_primary_foreground[0], colors.sidebar_primary_foreground[1], colors.sidebar_primary_foreground[2]));

        // Fonts
        css.push_str(&format!("  --font-sans: '{}';\n", self.font_sans));
        css.push_str(&format!("  --font-mono: '{}';\n", self.font_mono));

        // Border radius
        css.push_str(&format!("  --radius: {}rem;\n", self.radius));

        // Shadows
        css.push_str(&format!("  --shadow: {}px {}px {}px {}px rgba(0, 0, 0, {});\n",
                              colors.shadow_x, colors.shadow_y, colors.shadow_blur, colors.shadow_spread, colors.shadow_opacity));

        css.push_str("}\n");

        css
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let light = ThemeColors {
            background: [255, 255, 255],
            foreground: [17, 24, 39],
            card: [255, 255, 255],
            card_foreground: [17, 24, 39],
            popover: [255, 255, 255],
            popover_foreground: [17, 24, 39],
            primary: [216, 121, 67],
            primary_foreground: [255, 255, 255],
            secondary: [82, 117, 117],
            secondary_foreground: [255, 255, 255],
            muted: [243, 244, 246],
            muted_foreground: [107, 114, 128],
            accent: [238, 238, 238],
            accent_foreground: [17, 24, 39],
            destructive: [239, 68, 68],
            destructive_foreground: [250, 250, 250],
            border: [229, 231, 235],
            input: [229, 231, 235],
            ring: [216, 121, 67],
            chart_1: [95, 135, 135],
            chart_2: [231, 138, 83],
            chart_3: [251, 203, 151],
            chart_4: [136, 136, 136],
            chart_5: [153, 153, 153],
            sidebar: [243, 244, 246],
            sidebar_foreground: [17, 24, 39],
            sidebar_primary: [216, 121, 67],
            sidebar_primary_foreground: [255, 255, 255],
            sidebar_accent: [255, 255, 255],
            sidebar_accent_foreground: [17, 24, 39],
            sidebar_border: [229, 231, 235],
            sidebar_ring: [216, 121, 67],
            shadow_x: 0.0,
            shadow_y: 1.0,
            shadow_blur: 4.0,
            shadow_spread: 0.0,
            shadow_opacity: 0.05,
        };

        let dark = ThemeColors {
            background: [18, 17, 19],
            foreground: [193, 193, 193],
            card: [18, 18, 18],
            card_foreground: [193, 193, 193],
            popover: [18, 17, 19],
            popover_foreground: [193, 193, 193],
            primary: [231, 138, 83],
            primary_foreground: [18, 17, 19],
            secondary: [95, 135, 135],
            secondary_foreground: [18, 17, 19],
            muted: [34, 34, 34],
            muted_foreground: [136, 136, 136],
            accent: [51, 51, 51],
            accent_foreground: [193, 193, 193],
            destructive: [95, 135, 135],
            destructive_foreground: [18, 17, 19],
            border: [34, 34, 34],
            input: [34, 34, 34],
            ring: [231, 138, 83],
            chart_1: [95, 135, 135],
            chart_2: [231, 138, 83],
            chart_3: [251, 203, 151],
            chart_4: [136, 136, 136],
            chart_5: [153, 153, 153],
            sidebar: [18, 18, 18],
            sidebar_foreground: [193, 193, 193],
            sidebar_primary: [231, 138, 83],
            sidebar_primary_foreground: [18, 17, 19],
            sidebar_accent: [51, 51, 51],
            sidebar_accent_foreground: [193, 193, 193],
            sidebar_border: [34, 34, 34],
            sidebar_ring: [231, 138, 83],
            shadow_x: 0.0,
            shadow_y: 1.0,
            shadow_blur: 4.0,
            shadow_spread: 0.0,
            shadow_opacity: 0.05,
        };

        Self {
            light,
            dark,
            css_variables: HashMap::new(),
            font_sans: "Inter".to_string(),
            font_mono: "JetBrains Mono".to_string(),
            radius: 0.5,
            is_dark: false,
        }
    }
}

pub fn load_theme_from_file(path: &str) -> Result<ThemeConfig> {
    ThemeConfig::load_from_file(path)
}

impl ThemeColors {
    pub fn get_color(&self, name: &str) -> Color {
        let rgb = match name {
            "background" => self.background,
            "foreground" => self.foreground,
            "card" => self.card,
            "card_foreground" => self.card_foreground,
            "popover" => self.popover,
            "popover_foreground" => self.popover_foreground,
            "primary" => self.primary,
            "primary_foreground" => self.primary_foreground,
            "secondary" => self.secondary,
            "secondary_foreground" => self.secondary_foreground,
            "muted" => self.muted,
            "muted_foreground" => self.muted_foreground,
            "accent" => self.accent,
            "accent_foreground" => self.accent_foreground,
            "destructive" => self.destructive,
            "destructive_foreground" => self.destructive_foreground,
            "border" => self.border,
            "input" => self.input,
            "ring" => self.ring,
            "sidebar" => self.sidebar,
            "sidebar_foreground" => self.sidebar_foreground,
            "sidebar_primary" => self.sidebar_primary,
            "sidebar_primary_foreground" => self.sidebar_primary_foreground,
            "sidebar_accent" => self.sidebar_accent,
            "sidebar_accent_foreground" => self.sidebar_accent_foreground,
            "sidebar_border" => self.sidebar_border,
            "sidebar_ring" => self.sidebar_ring,
            "chart_1" => self.chart_1,
            "chart_2" => self.chart_2,
            "chart_3" => self.chart_3,
            "chart_4" => self.chart_4,
            "chart_5" => self.chart_5,
            _ => self.foreground,
        };
        Color::rgb(rgb[0], rgb[1], rgb[2])
    }
}
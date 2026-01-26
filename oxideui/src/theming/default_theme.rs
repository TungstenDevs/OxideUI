#[derive(Debug, Clone, Copy)]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: ColorRGB,
    pub foreground: ColorRGB,
    pub card: ColorRGB,
    pub card_foreground: ColorRGB,
    pub popover: ColorRGB,
    pub popover_foreground: ColorRGB,

    pub primary: ColorRGB,
    pub primary_foreground: ColorRGB,
    pub secondary: ColorRGB,
    pub secondary_foreground: ColorRGB,

    pub muted: ColorRGB,
    pub muted_foreground: ColorRGB,
    pub accent: ColorRGB,
    pub accent_foreground: ColorRGB,

    pub destructive: ColorRGB,
    pub destructive_foreground: ColorRGB,

    pub border: ColorRGB,
    pub input: ColorRGB,
    pub ring: ColorRGB,

    pub chart_1: ColorRGB,
    pub chart_2: ColorRGB,
    pub chart_3: ColorRGB,
    pub chart_4: ColorRGB,
    pub chart_5: ColorRGB,

    pub sidebar: ColorRGB,
    pub sidebar_foreground: ColorRGB,
    pub sidebar_primary: ColorRGB,
    pub sidebar_primary_foreground: ColorRGB,
    pub sidebar_accent: ColorRGB,
    pub sidebar_accent_foreground: ColorRGB,
    pub sidebar_border: ColorRGB,
    pub sidebar_ring: ColorRGB,

    pub font_sans: &'static str,
    pub font_mono: &'static str,
    pub font_serif: &'static str,

    pub radius: f32,

    pub shadow_x: f32,
    pub shadow_y: f32,
    pub shadow_blur: f32,
    pub shadow_spread: f32,
    pub shadow_opacity: f32,
}

pub const LIGHT_THEME: Theme = Theme {
    background: ColorRGB::new(255, 255, 255),
    foreground: ColorRGB::new(17, 24, 39),
    card: ColorRGB::new(255, 255, 255),
    card_foreground: ColorRGB::new(17, 24, 39),
    popover: ColorRGB::new(255, 255, 255),
    popover_foreground: ColorRGB::new(17, 24, 39),

    primary: ColorRGB::new(216, 121, 67),
    primary_foreground: ColorRGB::new(255, 255, 255),
    secondary: ColorRGB::new(82, 117, 117),
    secondary_foreground: ColorRGB::new(255, 255, 255),

    muted: ColorRGB::new(243, 244, 246),
    muted_foreground: ColorRGB::new(107, 114, 128),
    accent: ColorRGB::new(238, 238, 238),
    accent_foreground: ColorRGB::new(17, 24, 39),

    destructive: ColorRGB::new(239, 68, 68),
    destructive_foreground: ColorRGB::new(250, 250, 250),

    border: ColorRGB::new(229, 231, 235),
    input: ColorRGB::new(229, 231, 235),
    ring: ColorRGB::new(216, 121, 67),

    chart_1: ColorRGB::new(95, 135, 135),
    chart_2: ColorRGB::new(231, 138, 83),
    chart_3: ColorRGB::new(251, 203, 151),
    chart_4: ColorRGB::new(136, 136, 136),
    chart_5: ColorRGB::new(153, 153, 153),

    sidebar: ColorRGB::new(243, 244, 246),
    sidebar_foreground: ColorRGB::new(17, 24, 39),
    sidebar_primary: ColorRGB::new(216, 121, 67),
    sidebar_primary_foreground: ColorRGB::new(255, 255, 255),
    sidebar_accent: ColorRGB::new(255, 255, 255),
    sidebar_accent_foreground: ColorRGB::new(17, 24, 39),
    sidebar_border: ColorRGB::new(229, 231, 235),
    sidebar_ring: ColorRGB::new(216, 121, 67),

    font_sans: "Inter",
    font_mono: "JetBrains Mono",
    font_serif: "serif",

    radius: 0.75,

    shadow_x: 0.0,
    shadow_y: 1.0,
    shadow_blur: 4.0,
    shadow_spread: 0.0,
    shadow_opacity: 0.05,
};

pub const DARK_THEME: Theme = Theme {
    background: ColorRGB::new(18, 17, 19),
    foreground: ColorRGB::new(193, 193, 193),
    card: ColorRGB::new(18, 18, 18),
    card_foreground: ColorRGB::new(193, 193, 193),
    popover: ColorRGB::new(18, 17, 19),
    popover_foreground: ColorRGB::new(193, 193, 193),

    primary: ColorRGB::new(231, 138, 83),
    primary_foreground: ColorRGB::new(18, 17, 19),
    secondary: ColorRGB::new(95, 135, 135),
    secondary_foreground: ColorRGB::new(18, 17, 19),

    muted: ColorRGB::new(34, 34, 34),
    muted_foreground: ColorRGB::new(136, 136, 136),
    accent: ColorRGB::new(51, 51, 51),
    accent_foreground: ColorRGB::new(193, 193, 193),

    destructive: ColorRGB::new(95, 135, 135),
    destructive_foreground: ColorRGB::new(18, 17, 19),

    border: ColorRGB::new(34, 34, 34),
    input: ColorRGB::new(34, 34, 34),
    ring: ColorRGB::new(231, 138, 83),

    chart_1: ColorRGB::new(95, 135, 135),
    chart_2: ColorRGB::new(231, 138, 83),
    chart_3: ColorRGB::new(251, 203, 151),
    chart_4: ColorRGB::new(136, 136, 136),
    chart_5: ColorRGB::new(153, 153, 153),

    sidebar: ColorRGB::new(18, 18, 18),
    sidebar_foreground: ColorRGB::new(193, 193, 193),
    sidebar_primary: ColorRGB::new(231, 138, 83),
    sidebar_primary_foreground: ColorRGB::new(18, 17, 19),
    sidebar_accent: ColorRGB::new(51, 51, 51),
    sidebar_accent_foreground: ColorRGB::new(193, 193, 193),
    sidebar_border: ColorRGB::new(34, 34, 34),
    sidebar_ring: ColorRGB::new(231, 138, 83),

    font_sans: "Inter",
    font_mono: "JetBrains Mono",
    font_serif: "serif",

    radius: 0.75,

    shadow_x: 0.0,
    shadow_y: 1.0,
    shadow_blur: 4.0,
    shadow_spread: 0.0,
    shadow_opacity: 0.05,
};

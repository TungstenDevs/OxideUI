mod default_theme;
mod theme_loader;

pub use default_theme::{LIGHT_THEME, DARK_THEME, ColorRGB, Theme as DefaultTheme};
pub use theme_loader::{ThemeConfig, ThemeColors, load_theme_from_file};

pub struct ThemeManager {
    config: ThemeConfig,
    is_dark: bool,
}

impl ThemeManager {
    pub fn new(config: ThemeConfig, is_dark: bool) -> Self {
        Self { config, is_dark }
    }

    pub fn toggle_dark_mode(&mut self) {
        self.is_dark = !self.is_dark;
    }

    pub fn set_dark_mode(&mut self, dark: bool) {
        self.is_dark = dark;
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark
    }

    pub fn get_css_variables(&self) -> String {
        self.config.to_css_variables(self.is_dark)
    }

    pub fn get_current_colors(&self) -> &ThemeColors {
        if self.is_dark {
            &self.config.dark
        } else {
            &self.config.light
        }
    }
}
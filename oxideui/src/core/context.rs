//! Build context - safe access to element tree during widget building

use std::any::TypeId;
use std::sync::Arc;
use crate::Color;
use crate::core::element::{ElementId, SharedElementTree};
use crate::layout::constraints::Constraints;
use crate::theming::ThemeConfig;

/// Theme data with Radix UI inspired colors
#[derive(Clone, Debug)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub card: Color,
    pub card_foreground: Color,
    pub popover: Color,
    pub popover_foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub destructive: Color,
    pub destructive_foreground: Color,
    pub border: Color,
    pub input: Color,
    pub ring: Color,
    pub sidebar: Color,
    pub sidebar_foreground: Color,
    pub sidebar_primary: Color,
    pub sidebar_primary_foreground: Color,
    pub sidebar_accent: Color,
    pub sidebar_accent_foreground: Color,
    pub sidebar_border: Color,
    pub sidebar_ring: Color,
    pub font_sans: String,
    pub font_mono: String,
    pub radius: f32,
    pub is_dark: bool,
    pub shadow_x: f32,
    pub shadow_y: f32,
    pub shadow_blur: f32,
    pub shadow_spread: f32,
    pub shadow_opacity: f32,
    pub chart_1: Color,
    pub chart_2: Color,
    pub chart_3: Color,
    pub chart_4: Color,
    pub chart_5: Color,
}

impl Theme {
    pub fn from_config(config: &ThemeConfig, is_dark: bool) -> Self {
        let colors = if is_dark { &config.dark } else { &config.light };

        Self {
            background: colors.get_color("background"),
            foreground: colors.get_color("foreground"),
            card: colors.get_color("card"),
            card_foreground: colors.get_color("card_foreground"),
            popover: colors.get_color("popover"),
            popover_foreground: colors.get_color("popover_foreground"),
            primary: colors.get_color("primary"),
            primary_foreground: colors.get_color("primary_foreground"),
            secondary: colors.get_color("secondary"),
            secondary_foreground: colors.get_color("secondary_foreground"),
            muted: colors.get_color("muted"),
            muted_foreground: colors.get_color("muted_foreground"),
            accent: colors.get_color("accent"),
            accent_foreground: colors.get_color("accent_foreground"),
            destructive: colors.get_color("destructive"),
            destructive_foreground: colors.get_color("destructive_foreground"),
            border: colors.get_color("border"),
            input: colors.get_color("input"),
            ring: colors.get_color("ring"),
            sidebar: colors.get_color("sidebar"),
            sidebar_foreground: colors.get_color("sidebar_foreground"),
            sidebar_primary: colors.get_color("sidebar_primary"),
            sidebar_primary_foreground: colors.get_color("sidebar_primary_foreground"),
            sidebar_accent: colors.get_color("sidebar_accent"),
            sidebar_accent_foreground: colors.get_color("sidebar_accent_foreground"),
            sidebar_border: colors.get_color("sidebar_border"),
            sidebar_ring: colors.get_color("sidebar_ring"),
            font_sans: config.font_sans.clone(),
            font_mono: config.font_mono.clone(),
            radius: config.radius,
            is_dark,
            shadow_x: colors.shadow_x,
            shadow_y: colors.shadow_y,
            shadow_blur: colors.shadow_blur,
            shadow_spread: colors.shadow_spread,
            shadow_opacity: colors.shadow_opacity,
            chart_1: colors.get_color("chart_1"),
            chart_2: colors.get_color("chart_2"),
            chart_3: colors.get_color("chart_3"),
            chart_4: colors.get_color("chart_4"),
            chart_5: colors.get_color("chart_5"),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        let config = ThemeConfig::default();
        Theme::from_config(&config, false)
    }
}

/// Theme provider trait for widgets
pub trait ThemeProvider {
    fn theme(&self) -> &Theme;
    fn is_dark(&self) -> bool {
        self.theme().is_dark
    }
}

/// Build context provides safe access to the element tree during widget building
pub struct BuildContext {
    /// The element being built
    pub element_id: ElementId,

    /// Reference to the element tree
    pub element_tree: SharedElementTree,

    /// Layout constraints for this element
    pub constraints: Constraints,

    /// Current theme
    pub theme: Arc<Theme>,
}

impl BuildContext {
    /// Create a new build context
    pub fn new(
        element_id: ElementId,
        element_tree: SharedElementTree,
        constraints: Constraints,
        theme: Arc<Theme>,
    ) -> Self {
        Self {
            element_id,
            element_tree,
            constraints,
            theme,
        }
    }

    /// Get the parent element ID
    pub fn parent(&self) -> Option<ElementId> {
        self.element_tree.read().get_parent(self.element_id)
    }

    /// Get the children of this element
    pub fn children(&self) -> Vec<ElementId> {
        self.element_tree.read().get_children(self.element_id)
    }

    /// Find an ancestor element of a specific widget type
    pub fn find_ancestor<W: 'static>(&self) -> Option<ElementId> {
        self.element_tree
            .read()
            .find_ancestor(self.element_id, TypeId::of::<W>())
    }

    /// Mark the current element as dirty (needs rebuilding)
    pub fn mark_dirty(&self) {
        self.element_tree.write().mark_dirty(self.element_id);
    }

    /// Create a child context
    pub fn child_context(&self, child_id: ElementId, constraints: Constraints) -> BuildContext {
        BuildContext {
            element_id: child_id,
            element_tree: self.element_tree.clone(),
            constraints,
            theme: self.theme.clone(),
        }
    }
}

impl ThemeProvider for BuildContext {
    fn theme(&self) -> &Theme {
        &self.theme
    }
}

impl Clone for BuildContext {
    fn clone(&self) -> Self {
        Self {
            element_id: self.element_id,
            element_tree: self.element_tree.clone(),
            constraints: self.constraints,
            theme: self.theme.clone(),
        }
    }
}
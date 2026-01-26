use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Combobox {
    pub options: Vec<String>,
    pub selected: Option<usize>,
    pub placeholder: String,
    pub searchable: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub disabled: bool,
    pub open: bool,
    pub on_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub on_search: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

impl Combobox {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected: None,
            placeholder: "Select an option...".to_string(),
            searchable: false,
            width: None,
            height: None,
            disabled: false,
            open: false,
            on_change: None,
            on_search: None,
            tooltip: None,
            key: None,
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        self.on_change = Some(Arc::new(callback));
        self
    }

    pub fn with_on_search<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.on_search = Some(Arc::new(callback));
        self
    }

    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Combobox {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(200.0);
        let height = self.height.unwrap_or(40.0);
        let item_height = 32.0;

        let bg_color = if self.disabled {
            theme.muted
        } else {
            theme.input
        };

        let border_color = if self.disabled {
            theme.border.with_alpha(128)
        } else {
            theme.border
        };

        let text_color = if self.disabled {
            theme.muted_foreground
        } else {
            theme.foreground
        };

        let mut render_objects = Vec::new();

        // Main combobox box
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            bg_color,
        ));

        // Border
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, 1.0),
            border_color,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(width - 1.0, 0.0, 1.0, height),
            border_color,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, height - 1.0, width, 1.0),
            border_color,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, 1.0, height),
            border_color,
        ));

        // Selected value or placeholder
        let display_text = if let Some(selected) = self.selected {
            &self.options[selected]
        } else {
            &self.placeholder
        };

        let display_color = if self.selected.is_none() && !self.disabled {
            theme.muted_foreground
        } else {
            text_color
        };

        render_objects.push(RenderObject::text(
            display_text.to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 14.0,
                color: display_color,
                bold: false,
                italic: false,
            },
            Point::new(12.0, height / 2.0 + 5.0),
        ));

        // Combobox arrow
        render_objects.push(RenderObject::text(
            "▼".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 12.0,
                color: theme.muted_foreground,
                bold: false,
                italic: false,
            },
            Point::new(width - 24.0, height / 2.0 + 5.0),
        ));

        // Dropdown menu (if open)
        if self.open && !self.disabled {
            let menu_height = ((self.options.len() as f32 + 0.5) * item_height).min(250.0);

            // Menu background
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height, width, menu_height),
                theme.popover,
            ));

            // Menu border
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height, width, 1.0),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(width - 1.0, height, 1.0, menu_height),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height + menu_height - 1.0, width, 1.0),
                theme.border,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, height, 1.0, menu_height),
                theme.border,
            ));

            // Search input (if searchable)
            let mut current_y = height;
            if self.searchable {
                let search_height = item_height;

                // Search background
                render_objects.push(RenderObject::rect(
                    Rect::new(0.0, current_y, width, search_height),
                    theme.background,
                ));

                // Search placeholder
                render_objects.push(RenderObject::text(
                    "Search...".to_string(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 14.0,
                        color: theme.muted_foreground,
                        bold: false,
                        italic: false,
                    },
                    Point::new(12.0, current_y + search_height / 2.0 + 5.0),
                ));

                current_y += search_height;
            }

            // Menu items
            for (i, option) in self.options.iter().enumerate() {
                let item_y = current_y + (i as f32 * item_height);
                let is_selected = self.selected == Some(i);

                // Item background (hover/selected effect)
                if is_selected {
                    render_objects.push(RenderObject::rect(
                        Rect::new(0.0, item_y, width, item_height),
                        theme.accent,
                    ));
                }

                // Item text
                render_objects.push(RenderObject::text(
                    option.clone(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 14.0,
                        color: if is_selected { theme.accent_foreground } else { theme.popover_foreground },
                        bold: false,
                        italic: false,
                    },
                    Point::new(12.0, item_y + item_height / 2.0 + 5.0),
                ));
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Combobox {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn key(&self) -> Option<WidgetKey> {
        self.key.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Widget> {
        Box::new(self.clone())
    }
}
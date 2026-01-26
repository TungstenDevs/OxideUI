use std::any::Any;
use std::sync::Arc;
use crate::core::context::BuildContext;
use crate::core::render_object::{Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Table {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub width: Option<f32>,
    pub striped: bool,
    pub hoverable: bool,
    pub bordered: bool,
    pub compact: bool,
    pub sortable: bool,
    pub on_row_click: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub on_sort: Option<Arc<dyn Fn(usize, SortDirection) + Send + Sync>>,
    key: Option<WidgetKey>,
}

#[derive(Clone)]
pub struct TableColumn {
    pub label: String,
    pub width: ColumnWidth,
    pub align: TableAlign,
    pub sortable: bool,
}

#[derive(Clone)]
pub struct TableRow {
    pub cells: Vec<String>,
    pub selectable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColumnWidth {
    Fixed(f32),
    Flex(f32),
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TableAlign {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    None,
}

impl Table {
    pub fn new(columns: Vec<TableColumn>) -> Self {
        Self {
            columns,
            rows: Vec::new(),
            width: None,
            striped: false,
            hoverable: true,
            bordered: true,
            compact: false,
            sortable: false,
            on_row_click: None,
            on_sort: None,
            key: None,
        }
    }

    pub fn with_rows(mut self, rows: Vec<TableRow>) -> Self {
        self.rows = rows;
        self
    }

    pub fn add_row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_on_row_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        self.on_row_click = Some(Arc::new(callback));
        self
    }

    pub fn with_on_sort<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, SortDirection) + Send + Sync + 'static,
    {
        self.on_sort = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }

    fn calculate_column_widths(&self, total_width: f32) -> Vec<f32> {
        let mut widths = Vec::new();
        let mut flex_sum = 0.0;
        let mut fixed_total = 0.0;

        // Calculate fixed and flex totals
        for col in &self.columns {
            match col.width {
                ColumnWidth::Fixed(w) => fixed_total += w,
                ColumnWidth::Flex(flex) => flex_sum += flex,
                ColumnWidth::Auto => flex_sum += 1.0,
            }
        }

        let available_flex = (total_width - fixed_total).max(0.0);

        // Calculate actual widths
        for col in &self.columns {
            let width = match col.width {
                ColumnWidth::Fixed(w) => w,
                ColumnWidth::Flex(flex) => {
                    if flex_sum > 0.0 {
                        (flex / flex_sum) * available_flex
                    } else {
                        available_flex / self.columns.len() as f32
                    }
                }
                ColumnWidth::Auto => {
                    if flex_sum > 0.0 {
                        (1.0 / flex_sum) * available_flex
                    } else {
                        available_flex / self.columns.len() as f32
                    }
                }
            };
            widths.push(width);
        }

        widths
    }
}

impl StatelessWidget for Table {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(ctx.constraints.max_width);
        let row_height = if self.compact { 32.0 } else { 48.0 };
        let header_height = if self.compact { 40.0 } else { 56.0 };

        let column_widths = self.calculate_column_widths(width);
        let mut render_objects = Vec::new();

        // Table background
        let total_height = header_height + (self.rows.len() as f32 * row_height);
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, total_height),
            theme.card,
        ));

        // Table border
        if self.bordered {
            let border_color = theme.border;
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, 0.0, width, 1.0),
                border_color,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(width - 1.0, 0.0, 1.0, total_height),
                border_color,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, total_height - 1.0, width, 1.0),
                border_color,
            ));
            render_objects.push(RenderObject::rect(
                Rect::new(0.0, 0.0, 1.0, total_height),
                border_color,
            ));
        }

        // Header background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, header_height),
            theme.muted,
        ));

        // Header separator
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, header_height - 1.0, width, 1.0),
            theme.border,
        ));

        // Header cells
        let mut current_x = 8.0;
        for (i, col) in self.columns.iter().enumerate() {
            let col_width = column_widths[i];

            // Column text
            let x_offset = match col.align {
                TableAlign::Left => current_x,
                TableAlign::Center => current_x + (col_width - col.label.len() as f32 * 7.0) / 2.0,
                TableAlign::Right => current_x + col_width - col.label.len() as f32 * 7.0 - 8.0,
            };

            render_objects.push(RenderObject::text(
                col.label.clone(),
                TextStyle {
                    font_family: theme.font_sans.clone(),
                    font_size: 14.0,
                    color: theme.foreground,
                    bold: true,
                    italic: false,
                },
                Point::new(x_offset.max(current_x), header_height / 2.0 + 5.0),
            ));

            // Sort indicator if sortable
            if self.sortable && col.sortable {
                render_objects.push(RenderObject::text(
                    "⇅".to_string(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 12.0,
                        color: theme.muted_foreground,
                        bold: false,
                        italic: false,
                    },
                    Point::new(current_x + col_width - 20.0, header_height / 2.0 + 5.0),
                ));
            }

            // Vertical separator
            if self.bordered && i < self.columns.len() - 1 {
                render_objects.push(RenderObject::rect(
                    Rect::new(current_x + col_width, 0.0, 1.0, total_height),
                    theme.border,
                ));
            }

            current_x += col_width;
        }

        // Data rows
        let mut current_y = header_height;
        for (row_idx, row) in self.rows.iter().enumerate() {
            // Striped background
            if self.striped && row_idx % 2 == 1 {
                render_objects.push(RenderObject::rect(
                    Rect::new(0.0, current_y, width, row_height),
                    theme.muted.with_alpha(50),
                ));
            }

            // Row separator
            if self.bordered {
                render_objects.push(RenderObject::rect(
                    Rect::new(0.0, current_y + row_height - 1.0, width, 1.0),
                    theme.border,
                ));
            }

            // Row cells
            current_x = 8.0;
            for (col_idx, cell) in row.cells.iter().enumerate() {
                if col_idx >= self.columns.len() {
                    break;
                }

                let col = &self.columns[col_idx];
                let col_width = column_widths[col_idx];

                let x_offset = match col.align {
                    TableAlign::Left => current_x,
                    TableAlign::Center => current_x + (col_width - cell.len() as f32 * 7.0) / 2.0,
                    TableAlign::Right => current_x + col_width - cell.len() as f32 * 7.0 - 8.0,
                };

                render_objects.push(RenderObject::text(
                    cell.clone(),
                    TextStyle {
                        font_family: theme.font_sans.clone(),
                        font_size: 13.0,
                        color: theme.foreground,
                        bold: false,
                        italic: false,
                    },
                    Point::new(x_offset.max(current_x), current_y + row_height / 2.0 + 5.0),
                ));

                current_x += col_width;
            }

            current_y += row_height;
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Table {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        self.build_stateless(ctx)
    }

    fn handle_event(&self, event: &crate::core::event::UiEvent, context: &mut crate::core::event::EventContext) -> crate::core::event::EventResult {
        use crate::core::event::{UiEvent, MouseButton, EventResult};

        match event {
            UiEvent::PointerUp { position, button: MouseButton::Left, .. } if context.is_at_target() => {
                let row_height = if self.compact { 32.0 } else { 48.0 };
                let header_height = if self.compact { 40.0 } else { 56.0 };

                // Check if clicked on header (for sorting)
                if position.y <= header_height && self.sortable {
                    let width = self.width.unwrap_or(800.0);
                    let column_widths = self.calculate_column_widths(width);

                    let mut current_x = 0.0;
                    for (i, col_width) in column_widths.iter().enumerate() {
                        if position.x >= current_x && position.x < current_x + col_width {
                            if self.columns[i].sortable {
                                if let Some(on_sort) = &self.on_sort {
                                    on_sort(i, SortDirection::Ascending);
                                }
                                return EventResult::Stopped;
                            }
                        }
                        current_x += col_width;
                    }
                } else if position.y > header_height {
                    // Check if clicked on row
                    let row_index = ((position.y - header_height) / row_height) as usize;
                    if row_index < self.rows.len() && self.rows[row_index].selectable {
                        if let Some(on_row_click) = &self.on_row_click {
                            on_row_click(row_index);
                            return EventResult::Stopped;
                        }
                    }
                }

                EventResult::Unhandled
            }
            _ => EventResult::Unhandled,
        }
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

impl TableColumn {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            width: ColumnWidth::Auto,
            align: TableAlign::Left,
            sortable: false,
        }
    }

    pub fn with_width(mut self, width: ColumnWidth) -> Self {
        self.width = width;
        self
    }

    pub fn align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }
}

impl TableRow {
    pub fn new(cells: Vec<String>) -> Self {
        Self {
            cells,
            selectable: true,
        }
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }
}
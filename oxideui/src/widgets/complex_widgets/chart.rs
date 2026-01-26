use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use crate::ThemeProvider;

#[derive(Clone)]
pub struct Chart {
    pub data: Vec<f32>,
    pub labels: Vec<String>,
    pub chart_type: ChartType,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub show_grid: bool,
    pub show_labels: bool,
    pub colors: Vec<Color>,
    pub tooltip: Option<String>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Area,
}

impl Chart {
    pub fn new(data: Vec<f32>) -> Self {
        Self {
            data,
            labels: Vec::new(),
            chart_type: ChartType::Bar,
            width: None,
            height: None,
            show_grid: true,
            show_labels: true,
            colors: Vec::new(),
            tooltip: None,
            key: None,
        }
    }

    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    pub fn with_chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = chart_type;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn with_colors(mut self, colors: Vec<Color>) -> Self {
        self.colors = colors;
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

impl StatelessWidget for Chart {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(400.0);
        let height = self.height.unwrap_or(300.0);

        let padding = 40.0;
        let chart_width = width - (padding * 2.0);
        let chart_height = height - (padding * 2.0);

        let mut render_objects = Vec::new();

        // Chart background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            theme.card,
        ));

        // Chart border
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(width - 1.0, 0.0, 1.0, height),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, height - 1.0, width, 1.0),
            theme.border,
        ));
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, 1.0, height),
            theme.border,
        ));

        // Grid lines
        if self.show_grid {
            let grid_color = theme.border.with_alpha(50);

            // Vertical grid lines
            for i in 0..=10 {
                let x = padding + (i as f32 * chart_width / 10.0);
                render_objects.push(RenderObject::rect(
                    Rect::new(x, padding, 1.0, chart_height),
                    grid_color,
                ));
            }

            // Horizontal grid lines
            for i in 0..=5 {
                let y = padding + (i as f32 * chart_height / 5.0);
                render_objects.push(RenderObject::rect(
                    Rect::new(padding, y, chart_width, 1.0),
                    grid_color,
                ));
            }
        }

        if !self.data.is_empty() {
            let max_value = self.data.iter().cloned().fold(0.0, f32::max).max(1.0);
            let item_count = self.data.len();
            let item_width = chart_width / item_count as f32;

            let default_colors = vec![
                theme.chart_1,
                theme.chart_2,
                theme.chart_3,
                theme.chart_4,
                theme.chart_5,
            ];
            let colors = if self.colors.is_empty() { &default_colors } else { &self.colors };

            match self.chart_type {
                ChartType::Bar => {
                    // Draw bars
                    for (i, &value) in self.data.iter().enumerate() {
                        let bar_height = (value / max_value) * chart_height;
                        let x = padding + (i as f32 * item_width) + 4.0;
                        let y = padding + chart_height - bar_height;
                        let bar_width = item_width - 8.0;

                        let color_index = i % colors.len();
                        render_objects.push(RenderObject::rect(
                            Rect::new(x, y, bar_width, bar_height),
                            colors[color_index],
                        ));

                        // Value label
                        if self.show_labels && bar_height > 20.0 {
                            render_objects.push(RenderObject::text(
                                format!("{:.1}", value),
                                TextStyle {
                                    font_family: theme.font_sans.clone(),
                                    font_size: 10.0,
                                    color: theme.foreground,
                                    bold: false,
                                    italic: false,
                                },
                                Point::new(x + bar_width / 2.0 - 10.0, y - 15.0),
                            ));
                        }
                    }
                }
                ChartType::Line => {
                    // Draw line chart
                    let points: Vec<Point> = self.data.iter().enumerate().map(|(i, &value)| {
                        let x = padding + (i as f32 * item_width) + (item_width / 2.0);
                        let y = padding + chart_height - ((value / max_value) * chart_height);
                        Point::new(x, y)
                    }).collect();

                    // Draw line
                    for i in 0..points.len() - 1 {
                        let start = points[i];
                        let end = points[i + 1];

                        // Simple line drawing (would need proper line rendering)
                        let line_color = colors[0];
                        // For simplicity, draw a rectangle representing the line
                        let dx = end.x - start.x;
                        let dy = end.y - start.y;
                        let length = (dx * dx + dy * dy).sqrt();
                        let _angle = dy.atan2(dx);

                        // Note: This is a simplification. Real line drawing would need proper rendering.
                        render_objects.push(RenderObject::rect(
                            Rect::new(start.x, start.y, length, 2.0),
                            line_color,
                        ));
                    }
                }
                ChartType::Pie => {
                    // Draw pie chart (simplified as donut chart)
                    let center_x = padding + chart_width / 2.0;
                    let center_y = padding + chart_height / 2.0;
                    let radius = chart_height.min(chart_width) / 3.0;

                    let total: f32 = self.data.iter().sum();
                    let mut _current_angle = 0.0;

                    for (i, &value) in self.data.iter().enumerate() {
                        let slice_angle = (value / total) * 360.0;
                        let color_index = i % colors.len();

                        // Draw slice (simplified as circle segment)
                        // In a real implementation, we'd draw proper arcs
                        render_objects.push(RenderObject::rect(
                            Rect::new(center_x - radius, center_y - radius, radius * 2.0, radius * 2.0),
                            colors[color_index].with_alpha(150),
                        ));

                        _current_angle += slice_angle;
                    }
                }
                ChartType::Area => {
                    // Draw area chart (simplified as filled polygon)
                    let points: Vec<Point> = self.data.iter().enumerate().map(|(i, &value)| {
                        let x = padding + (i as f32 * item_width);
                        let y = padding + chart_height - ((value / max_value) * chart_height);
                        Point::new(x, y)
                    }).collect();

                    // Draw area (simplified as series of rectangles)
                    for i in 0..points.len() - 1 {
                        let start = points[i];
                        let end = points[i + 1];

                        let area_color = colors[0].with_alpha(100);
                        let area_height = chart_height - start.y.min(end.y);

                        render_objects.push(RenderObject::rect(
                            Rect::new(start.x, start.y.min(end.y), end.x - start.x, area_height),
                            area_color,
                        ));
                    }
                }
            }
        }

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Chart {
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
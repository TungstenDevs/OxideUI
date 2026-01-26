use crate::core::context::{BuildContext, ThemeProvider};
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct Calendar {
    pub selected_date: Option<String>,
    pub month: u32,
    pub year: i32,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub show_header: bool,
    pub show_navigation: bool,
    pub on_date_select: Option<Arc<dyn Fn(String) + Send + Sync>>,
    key: Option<WidgetKey>,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            selected_date: None,
            month: 1,
            year: 2025,
            width: None,
            height: None,
            show_header: true,
            show_navigation: true,
            on_date_select: None,
            key: None,
        }
    }

    pub fn with_date(mut self, date: impl Into<String>) -> Self {
        self.selected_date = Some(date.into());
        self
    }

    pub fn with_month_year(mut self, month: u32, year: i32) -> Self {
        self.month = month;
        self.year = year;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Calendar {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(300.0);
        let height = self.height.unwrap_or(300.0);

        let mut render_objects = Vec::new();

        // Calendar background
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            theme.card,
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Calendar {
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

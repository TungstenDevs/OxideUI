use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct Grid {
    pub columns: usize,
    pub rows: usize,
    pub column_gap: f32,
    pub row_gap: f32,
    pub children: Vec<Box<dyn Widget>>,
    key: Option<WidgetKey>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            columns: 1,
            rows: 1,
            column_gap: 0.0,
            row_gap: 0.0,
            children: Vec::new(),
            key: None,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            columns: self.columns,
            rows: self.rows,
            column_gap: self.column_gap,
            row_gap: self.row_gap,
            children: self.children.iter().map(|c| c.clone_box()).collect(),
            key: self.key.clone(),
        }
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(1);
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self.row_gap = gap;
        self
    }

    pub fn column_gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self
    }

    pub fn row_gap(mut self, gap: f32) -> Self {
        self.row_gap = gap;
        self
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Widget>>) -> Self {
        self.children = children;
        self
    }

    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Grid {
    fn build_stateless(&self, _ctx: &BuildContext) -> WidgetNode {
        // For now, just return the children as a container
        // In a real implementation, we would calculate grid layout
        WidgetNode::Container {
            children: self.children.iter().map(|c| c.clone_box()).collect(),
        }
    }
}

impl Widget for Grid {
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
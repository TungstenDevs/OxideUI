use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct Flexbox {
    pub direction: FlexDirection,
    pub justify: JustifyContent,
    pub align: AlignItems,
    pub wrap: FlexWrap,
    pub gap: f32,
    pub children: Vec<Box<dyn Widget>>,
    key: Option<WidgetKey>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Flexbox {
    pub fn new() -> Self {
        Self {
            direction: FlexDirection::Row,
            justify: JustifyContent::FlexStart,
            align: AlignItems::Stretch,
            wrap: FlexWrap::NoWrap,
            gap: 0.0,
            children: Vec::new(),
            key: None,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            direction: self.direction,
            justify: self.justify,
            align: self.align,
            wrap: self.wrap,
            gap: self.gap,
            children: self.children.iter().map(|c| c.clone_box()).collect(),
            key: self.key.clone(),
        }
    }

    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.justify = justify;
        self
    }

    pub fn align(mut self, align: AlignItems) -> Self {
        self.align = align;
        self
    }

    pub fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
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

impl StatelessWidget for Flexbox {
    fn build_stateless(&self, _ctx: &BuildContext) -> WidgetNode {
        // For now, just return the children as a container
        // In a real implementation, we would calculate flexbox layout
        WidgetNode::Container {
            children: self.children.iter().map(|c| c.clone_box()).collect(),
        }
    }
}

impl Widget for Flexbox {
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
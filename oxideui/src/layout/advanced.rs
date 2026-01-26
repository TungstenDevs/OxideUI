// File: ./oxideui/src/layout/advanced.rs
//! Advanced layout engine with flexbox, grid, and absolute positioning

use crate::layout::constraints::{Constraints, Size};
use std::collections::HashMap;

/// Layout node in the layout tree
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: u64,
    pub constraints: Constraints,
    pub size: Size,
    pub position: (f32, f32),
    pub children: Vec<LayoutNode>,
    pub layout_type: LayoutType,
}

/// Layout algorithm type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutType {
    Flex,
    Grid,
    Absolute,
    Stack,
}

/// Flexbox layout properties
#[derive(Debug, Clone, Copy)]
pub struct FlexLayout {
    pub direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub wrap: FlexWrap,
    pub gap: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

/// Flex item properties
#[derive(Debug, Clone, Copy)]
pub struct FlexItem {
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<f32>,
    pub align_self: Option<AlignItems>,
}

impl Default for FlexItem {
    fn default() -> Self {
        Self {
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            align_self: None,
        }
    }
}

/// Grid layout properties
#[derive(Debug, Clone)]
pub struct GridLayout {
    pub columns: Vec<GridTrack>,
    pub rows: Vec<GridTrack>,
    pub column_gap: f32,
    pub row_gap: f32,
    pub auto_flow: GridAutoFlow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridTrack {
    Fixed(f32),
    Flex(f32),
    Auto,
    MinContent,
    MaxContent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridAutoFlow {
    Row,
    Column,
    RowDense,
    ColumnDense,
}

/// Grid item placement
#[derive(Debug, Clone, Copy)]
pub struct GridItem {
    pub column_start: Option<usize>,
    pub column_end: Option<usize>,
    pub row_start: Option<usize>,
    pub row_end: Option<usize>,
}

/// Layout engine
pub struct LayoutEngine {
    cache: HashMap<u64, LayoutNode>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Layout a node and its children
    pub fn layout(&mut self, node: &mut LayoutNode) {
        match node.layout_type {
            LayoutType::Flex => self.layout_flex(node),
            LayoutType::Grid => self.layout_grid(node),
            LayoutType::Absolute => self.layout_absolute(node),
            LayoutType::Stack => self.layout_stack(node),
        }
    }

    /// Flexbox layout algorithm
    fn layout_flex(&self, node: &mut LayoutNode) {
        let is_row = true;
        let mut position = 0.0;

        for child in &mut node.children {
            let child_size = child.constraints.biggest();

            if is_row {
                child.position = (position, 0.0);
                position += child_size.width;
            } else {
                child.position = (0.0, position);
                position += child_size.height;
            }

            child.size = child_size;
        }

        node.size = if is_row {
            Size::new(position, node.constraints.max_height)
        } else {
            Size::new(node.constraints.max_width, position)
        };
    }

    /// Grid layout algorithm - FIXED TYPE ANNOTATIONS
    fn layout_grid(&self, node: &mut LayoutNode) {
        let columns = 3;
        let gap = 10.0;

        let available_width = node.constraints.max_width - (gap * (columns - 1) as f32);
        let cell_width = available_width / columns as f32;
        let cell_height = 100.0;

        for (i, child) in node.children.iter_mut().enumerate() {
            let col = i % columns;
            let row = i / columns;

            child.position = (
                col as f32 * (cell_width + gap),
                row as f32 * (cell_height + gap),
            );
            child.size = Size::new(cell_width, cell_height);
        }

        let rows = (node.children.len() + columns - 1) / columns;
        node.size = Size::new(
            node.constraints.max_width,
            rows as f32 * cell_height + (rows - 1) as f32 * gap,
        );
    }

    fn layout_absolute(&self, node: &mut LayoutNode) {
        for child in &mut node.children {
            child.size = child.constraints.biggest();
        }
        node.size = node.constraints.biggest();
    }

    fn layout_stack(&self, node: &mut LayoutNode) {
        let mut max_width: f32 = 0.0;  // FIX: Explicit type annotation
        let mut max_height: f32 = 0.0; // FIX: Explicit type annotation

        for child in &mut node.children {
            child.position = (0.0, 0.0);
            child.size = child.constraints.biggest();

            max_width = max_width.max(child.size.width);
            max_height = max_height.max(child.size.height);
        }

        node.size = Size::new(max_width, max_height);
    }

    pub fn measure_intrinsic(&self, node: &LayoutNode) -> Size {
        match node.layout_type {
            LayoutType::Flex => self.measure_flex_intrinsic(node),
            LayoutType::Grid => self.measure_grid_intrinsic(node),
            _ => node.constraints.smallest(),
        }
    }

    fn measure_flex_intrinsic(&self, node: &LayoutNode) -> Size {
        let is_row = true;
        let mut total_width = 0.0;
        let mut total_height: f32 = 0.0; // FIX: Explicit type annotation

        for child in &node.children {
            let child_size = self.measure_intrinsic(child);

            if is_row {
                total_width += child_size.width;
                total_height = total_height.max(child_size.height);
            } else {
                total_width = total_width.max(child_size.width);
                total_height += child_size.height;
            }
        }

        Size::new(total_width, total_height)
    }

    fn measure_grid_intrinsic(&self, node: &LayoutNode) -> Size {
        let columns = 3;
        let rows = (node.children.len() + columns - 1) / columns;

        Size::new(
            300.0 * columns as f32,
            100.0 * rows as f32,
        )
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LayoutSolver {
    variables: HashMap<String, f32>,
}

impl LayoutSolver {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn solve(&mut self, constraints: &[LayoutConstraint]) -> bool {
        for constraint in constraints {
            match constraint {
                LayoutConstraint::Equal(var, value) => {
                    self.variables.insert(var.clone(), *value);
                }
                LayoutConstraint::GreaterThan(var, value) => {
                    let current = self.variables.get(var).copied().unwrap_or(0.0);
                    if current < *value {
                        self.variables.insert(var.clone(), *value);
                    }
                }
                LayoutConstraint::LessThan(var, value) => {
                    let current = self.variables.get(var).copied().unwrap_or(f32::INFINITY);
                    if current > *value {
                        self.variables.insert(var.clone(), *value);
                    }
                }
            }
        }
        true
    }

    pub fn get_value(&self, var: &str) -> Option<f32> {
        self.variables.get(var).copied()
    }
}

#[derive(Debug, Clone)]
pub enum LayoutConstraint {
    Equal(String, f32),
    GreaterThan(String, f32),
    LessThan(String, f32),
}

impl Default for LayoutSolver {
    fn default() -> Self {
        Self::new()
    }
}
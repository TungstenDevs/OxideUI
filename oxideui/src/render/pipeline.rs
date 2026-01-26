use std::collections::HashMap;
use crate::core::{ElementId, Rect, RenderObject};

/// Damage region tracking for efficient partial redraws
#[derive(Debug, Clone)]
pub struct DamageRegion {
    pub rects: Vec<Rect>,
}

impl DamageRegion {
    pub fn new() -> Self {
        Self { rects: Vec::new() }
    }

    pub fn add(&mut self, rect: Rect) {
        self.rects.push(rect);
    }

    pub fn merge(&mut self) -> Option<Rect> {
        if self.rects.is_empty() {
            return None;
        }

        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for rect in &self.rects {
            min_x = min_x.min(rect.x);
            min_y = min_y.min(rect.y);
            max_x = max_x.max(rect.x + rect.width);
            max_y = max_y.max(rect.y + rect.height);
        }

        Some(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
    }

    pub fn clear(&mut self) {
        self.rects.clear();
    }
}

/// Display list for batched rendering
#[derive(Debug, Clone)]
pub struct DisplayList {
    pub items: Vec<DisplayItem>,
}

#[derive(Debug, Clone)]
pub struct DisplayItem {
    pub render_object: RenderObject,
    pub bounds: Rect,
    pub transform: crate::core::render_object::Matrix,
    pub opacity: f32,
    pub clip: Option<Rect>,
}

impl DisplayList {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: DisplayItem) {
        self.items.push(item);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Cull items outside viewport
    pub fn cull(&mut self, viewport: Rect) {
        self.items.retain(|item| {
            item.bounds.x < viewport.x + viewport.width &&
                item.bounds.x + item.bounds.width > viewport.x &&
                item.bounds.y < viewport.y + viewport.height &&
                item.bounds.y + item.bounds.height > viewport.y
        });
    }
}

/// Rendering pipeline coordinator
pub struct RenderPipeline {
    pub damage: DamageRegion,
    pub display_list: DisplayList,
    pub layer_cache: HashMap<ElementId, RenderObject>,
    pub viewport: Rect,
}

impl RenderPipeline {
    pub fn new(viewport: Rect) -> Self {
        Self {
            damage: DamageRegion::new(),
            display_list: DisplayList::new(),
            layer_cache: HashMap::new(),
            viewport,
        }
    }

    /// Mark element as dirty and add to damage region
    pub fn mark_dirty(&mut self, element_id: ElementId, bounds: Rect) {
        self.damage.add(bounds);
        self.layer_cache.remove(&element_id);
    }

    /// Build display list from render tree
    pub fn build_display_list(&mut self, root: &RenderObject) {
        self.display_list.clear();
        self.build_display_list_recursive(
            root,
            crate::core::render_object::Matrix::identity(),
            1.0,
            None,
        );
        self.display_list.cull(self.viewport);
    }

    fn build_display_list_recursive(
        &mut self,
        obj: &RenderObject,
        transform: crate::core::render_object::Matrix,
        opacity: f32,
        clip: Option<Rect>,
    ) {
        match obj {
            RenderObject::Group { children } => {
                for child in children {
                    self.build_display_list_recursive(child, transform, opacity, clip);
                }
            }
            RenderObject::Transform { matrix, child } => {
                // Multiply transforms
                let new_transform = self.multiply_matrices(&transform, matrix);
                self.build_display_list_recursive(child, new_transform, opacity, clip);
            }
            RenderObject::Clip { rect, child } => {
                let new_clip = Some(self.transform_rect(*rect, &transform));
                self.build_display_list_recursive(child, transform, opacity, new_clip);
            }
            _ => {
                // Add to display list
                let bounds = self.calculate_bounds(obj, &transform);
                self.display_list.add(DisplayItem {
                    render_object: obj.clone(),
                    bounds,
                    transform,
                    opacity,
                    clip,
                });
            }
        }
    }

    fn multiply_matrices(
        &self,
        a: &crate::core::render_object::Matrix,
        b: &crate::core::render_object::Matrix,
    ) -> crate::core::render_object::Matrix {
        let mut result = crate::core::render_object::Matrix::identity();
        for i in 0..3 {
            for j in 0..3 {
                result.values[i][j] = 0.0;
                for k in 0..3 {
                    result.values[i][j] += a.values[i][k] * b.values[k][j];
                }
            }
        }
        result
    }

    fn transform_rect(&self, rect: Rect, matrix: &crate::core::render_object::Matrix) -> Rect {
        // Transform rect corners
        let x1 = rect.x * matrix.values[0][0] + rect.y * matrix.values[0][1] + matrix.values[0][2];
        let y1 = rect.x * matrix.values[1][0] + rect.y * matrix.values[1][1] + matrix.values[1][2];

        let x2 = (rect.x + rect.width) * matrix.values[0][0] + (rect.y + rect.height) * matrix.values[0][1] + matrix.values[0][2];
        let y2 = (rect.x + rect.width) * matrix.values[1][0] + (rect.y + rect.height) * matrix.values[1][1] + matrix.values[1][2];

        Rect::new(
            x1.min(x2),
            y1.min(y2),
            (x2 - x1).abs(),
            (y2 - y1).abs(),
        )
    }

    fn calculate_bounds(&self, obj: &RenderObject, transform: &crate::core::render_object::Matrix) -> Rect {
        match obj {
            RenderObject::Rect { rect, .. } => self.transform_rect(*rect, transform),
            RenderObject::Text { position, .. } => {
                // Approximate text bounds
                self.transform_rect(Rect::new(position.x, position.y, 100.0, 20.0), transform)
            }
            RenderObject::Image { size } => {
                self.transform_rect(Rect::from_size(*size), transform)
            }
            _ => Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn update_viewport(&mut self, viewport: Rect) {
        self.viewport = viewport;
    }

    pub fn has_damage(&self) -> bool {
        !self.damage.rects.is_empty()
    }

    pub fn clear_damage(&mut self) {
        self.damage.clear();
    }
}
use crate::core::render_object::{Color as OxColor, Matrix, Point, Rect, RenderObject, TextStyle};
use skia_safe::{Canvas, Color as SkColor, FontMgr, FontStyle, Paint, PaintStyle, Typeface};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle as SkTextStyle};

pub struct SkiaRenderer {
    font_cache: std::collections::HashMap<String, Typeface>,
    font_mgr: FontMgr,
    font_collection: FontCollection,
}

impl SkiaRenderer {
    pub fn new() -> Self {
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::new(), None);

        Self {
            font_cache: std::collections::HashMap::new(),
            font_mgr: FontMgr::new(),
            font_collection,
        }
    }

    pub fn render(&mut self, canvas: &Canvas, render_obj: &RenderObject) {
        match render_obj {
            RenderObject::Rect { rect, paint } => {
                self.draw_rect(canvas, rect, &paint.color);
            }
            RenderObject::Text { content, style, position } => {
                self.draw_text(canvas, content, style, position);
            }
            RenderObject::Image { size } => {
                self.draw_image_placeholder(canvas, *size);
            }
            RenderObject::Clip { rect, child } => {
                canvas.save();
                canvas.clip_rect(rect.to_skia_rect(), None, None);
                self.render(canvas, child);
                canvas.restore();
            }
            RenderObject::Transform { matrix, child } => {
                canvas.save();
                canvas.concat(&self.matrix_to_skia(matrix));
                self.render(canvas, child);
                canvas.restore();
            }
            RenderObject::Group { children } => {
                for child in children {
                    self.render(canvas, child);
                }
            }
            RenderObject::None => {}
        }
    }

    fn draw_rect(&self, canvas: &Canvas, rect: &Rect, color: &OxColor) {
        let mut paint = Paint::default();
        paint.set_color(SkColor::from_argb(color.a, color.r, color.g, color.b));
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        canvas.draw_rect(rect.to_skia_rect(), &paint);
    }

    fn draw_text(&mut self, canvas: &Canvas, content: &str, style: &TextStyle, position: &Point) {
        let paragraph_style = ParagraphStyle::new();
        let mut text_style = SkTextStyle::new();

        let typeface = self.get_or_create_typeface(&style.font_family, style.bold, style.italic);
        text_style.set_typeface(Some(typeface));
        text_style.set_font_size(style.font_size);
        text_style.set_color(SkColor::from_argb(
            style.color.a,
            style.color.r,
            style.color.g,
            style.color.b,
        ));

        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, self.font_collection.clone());
        paragraph_builder.push_style(&text_style);
        paragraph_builder.add_text(content);

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(f32::INFINITY);

        // Draw text using the paragraph's draw method instead of text_blob
        canvas.save();
        canvas.translate((position.x, position.y));
        paragraph.paint(canvas, (0.0, 0.0));
        canvas.restore();
    }

    fn get_or_create_typeface(&mut self, family: &str, bold: bool, italic: bool) -> Typeface {
        let cache_key = format!(
            "{}_{}{}",
            family,
            if bold { "b" } else { "" },
            if italic { "i" } else { "" }
        );

        if let Some(typeface) = self.font_cache.get(&cache_key) {
            return typeface.clone();
        }

        let font_style = match (bold, italic) {
            (true, true) => FontStyle::bold_italic(),
            (true, false) => FontStyle::bold(),
            (false, true) => FontStyle::italic(),
            (false, false) => FontStyle::normal(),
        };

        let typeface = self
            .font_mgr
            .match_family_style(family, font_style)
            .or_else(|| self.font_mgr.match_family_style("sans-serif", font_style))
            .or_else(|| self.font_mgr.match_family_style("", font_style))
            .unwrap_or_else(|| {
                self.font_mgr
                    .legacy_make_typeface("", font_style)
                    .expect("Failed to create any typeface")
            });

        self.font_cache.insert(cache_key.clone(), typeface.clone());
        typeface
    }

    fn draw_image_placeholder(&self, canvas: &Canvas, size: crate::layout::Size) {
        let mut paint = Paint::default();
        paint.set_color(SkColor::from_rgb(200, 200, 200));
        paint.set_anti_alias(true);
        let rect = skia_safe::Rect::from_xywh(0.0, 0.0, size.width, size.height);
        canvas.draw_rect(rect, &paint);

        paint.set_color(SkColor::from_rgb(150, 150, 150));
        paint.set_stroke_width(2.0);
        paint.set_style(PaintStyle::Stroke);
        canvas.draw_line((0.0, 0.0), (size.width, size.height), &paint);
        canvas.draw_line((size.width, 0.0), (0.0, size.height), &paint);
    }

    fn matrix_to_skia(&self, matrix: &Matrix) -> skia_safe::Matrix {
        skia_safe::Matrix::new_all(
            matrix.values[0][0],
            matrix.values[0][1],
            matrix.values[0][2],
            matrix.values[1][0],
            matrix.values[1][1],
            matrix.values[1][2],
            matrix.values[2][0],
            matrix.values[2][1],
            matrix.values[2][2],
        )
    }

    pub fn clear(&mut self, canvas: &Canvas, color: OxColor) {
        canvas.clear(SkColor::from_argb(color.a, color.r, color.g, color.b));
    }
}

impl Default for SkiaRenderer {
    fn default() -> Self {
        Self::new()
    }
}
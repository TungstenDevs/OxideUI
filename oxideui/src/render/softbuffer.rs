use anyhow::{anyhow, Result};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::window::Window;
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use super::RenderBackend;

pub struct SoftbufferRenderer {
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    context: Option<Context<Arc<Window>>>,
    width: u32,
    height: u32,
    window: Arc<Window>,
}

impl SoftbufferRenderer {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        println!("[Softbuffer] Initializing renderer...");

        let context = Context::new(window.clone())
            .map_err(|e| anyhow!("Failed to create softbuffer context: {}", e))?;

        println!("[Softbuffer] Renderer initialized successfully!");

        Ok(Self {
            surface: None,
            context: Some(context),
            width: 0,
            height: 0,
            window,
        })
    }

    fn ensure_surface(&mut self) -> Result<&mut Surface<Arc<Window>, Arc<Window>>> {
        if self.surface.is_none() {
            let size = self.window.inner_size();
            let width = size.width.max(1);
            let height = size.height.max(1);

            let context = self.context.as_ref().unwrap();
            self.surface = Some(Surface::new(context, self.window.clone())
                .map_err(|e| anyhow!("Softbuffer error: {}", e))?);

            self.width = width;
            self.height = height;

            self.surface.as_mut().unwrap().resize(
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            ).map_err(|e| anyhow!("Failed to resize surface: {}", e))?;
        }

        Ok(self.surface.as_mut().unwrap())
    }

    fn render_object_to_buffer(
        buffer: &mut [u32],
        obj: &RenderObject,
        width: u32,
        height: u32,
    ) {
        match obj {
            RenderObject::Rect { rect, paint } => {
                Self::draw_rect_to_buffer(buffer, rect, paint.color, width, height);
            }
            RenderObject::Text { content, style, position } => {
                Self::draw_text_to_buffer(buffer, content, style, position, width, height);
            }
            RenderObject::Group { children } => {
                for child in children {
                    Self::render_object_to_buffer(buffer, child, width, height);
                }
            }
            RenderObject::Transform { child, .. } => {
                Self::render_object_to_buffer(buffer, child, width, height);
            }
            RenderObject::Clip { child, .. } => {
                Self::render_object_to_buffer(buffer, child, width, height);
            }
            _ => {}
        }
    }

    fn draw_rect_to_buffer(
        buffer: &mut [u32],
        rect: &Rect,
        color: Color,
        width: u32,
        height: u32,
    ) {
        let x1 = rect.x.max(0.0).min(width as f32) as u32;
        let y1 = rect.y.max(0.0).min(height as f32) as u32;
        let x2 = ((rect.x + rect.width).max(0.0).min(width as f32)) as u32;
        let y2 = ((rect.y + rect.height).max(0.0).min(height as f32)) as u32;

        let color_u32 = ((color.a as u32) << 24)
            | ((color.r as u32) << 16)
            | ((color.g as u32) << 8)
            | (color.b as u32);

        for y in y1..y2 {
            for x in x1..x2 {
                let idx = (y * width + x) as usize;
                if idx < buffer.len() {
                    buffer[idx] = color_u32;
                }
            }
        }
    }

    fn draw_text_to_buffer(
        buffer: &mut [u32],
        text: &str,
        style: &TextStyle,
        position: &Point,
        width: u32,
        height: u32,
    ) {
        let x = position.x.max(0.0) as u32;
        let y = position.y.max(0.0) as u32;
        let char_width = (style.font_size * 0.6) as u32;
        let char_height = (style.font_size * 1.2) as u32;
        let color_u32 = ((style.color.a as u32) << 24)
            | ((style.color.r as u32) << 16)
            | ((style.color.g as u32) << 8)
            | (style.color.b as u32);

        for (i, ch) in text.chars().enumerate() {
            let char_x = x + (i as u32 * char_width);
            if char_x >= width || y >= height {
                break;
            }

            if ch.is_whitespace() {
                continue;
            }

            for dy in 0..char_height.min(height - y) {
                for dx in 0..(char_width - 2).min(width - char_x) {
                    let px = char_x + dx;
                    let py = y + dy;
                    let idx = (py * width + px) as usize;
                    if idx < buffer.len() {
                        buffer[idx] = color_u32;
                    }
                }
            }
        }
    }
}

impl RenderBackend for SoftbufferRenderer {
    fn draw(&mut self, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        let surface = self.ensure_surface()?;
        let mut buffer = surface.buffer_mut()
            .map_err(|e| anyhow!("Failed to get buffer: {}", e))?;

        // Clear with white background
        for pixel in buffer.iter_mut() {
            *pixel = 0xFFFFFFFF;
        }

        buffer.present()
            .map_err(|e| anyhow!("Failed to present buffer: {}", e))?;

        Ok(())
    }

    fn draw_render_object(&mut self, render_obj: &RenderObject, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        let surface = self.ensure_surface()?;
        let mut buffer = surface.buffer_mut()
            .map_err(|e| anyhow!("Failed to get buffer: {}", e))?;

        // Clear with white background
        for pixel in buffer.iter_mut() {
            *pixel = 0xFFFFFFFF;
        }

        Self::render_object_to_buffer(&mut buffer, render_obj, width, height);

        buffer.present()
            .map_err(|e| anyhow!("Failed to present buffer: {}", e))?;

        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        let width = width.max(1);
        let height = height.max(1);

        self.width = width;
        self.height = height;

        if let Some(surface) = &mut self.surface {
            surface.resize(
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            ).map_err(|e| anyhow!("Failed to resize surface: {}", e))?;
        }

        Ok(())
    }

    fn cleanup(&mut self) {
        println!("[Softbuffer] Cleaning up renderer");
        self.surface = None;
        self.context = None;
    }

    fn name(&self) -> &str {
        "Softbuffer"
    }
}
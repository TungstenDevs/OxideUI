use anyhow::{Result, anyhow};
use skia_safe::{ColorType, ISize, ImageInfo, Surface, AlphaType};
use skia_safe::image::CachingHint;
use softbuffer::Context;
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::window::Window;

use super::RenderBackend;
use crate::core::RenderObject;
use crate::render::rendering_impl::SkiaRenderer;

pub struct SkiaCPURenderer {
    surface: Surface,
    width: u32,
    height: u32,
    softbuffer_surface: softbuffer::Surface<Arc<Window>, Arc<Window>>,
    window: Arc<Window>,
    skia_renderer: SkiaRenderer,
}

unsafe impl Send for SkiaCPURenderer {}

impl SkiaCPURenderer {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        println!("[Skia CPU] Initializing renderer...");

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let info = ImageInfo::new(
            ISize::new(width as i32, height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        let surface = skia_safe::surfaces::raster(&info, None, None)
            .ok_or_else(|| anyhow!("Failed to create CPU surface"))?;

        let context = Context::new(window.clone())
            .map_err(|e| anyhow!("Failed to create softbuffer context: {}", e))?;

        let softbuffer_surface = softbuffer::Surface::new(&context, window.clone())
            .map_err(|e| anyhow!("Failed to create softbuffer surface: {}", e))?;

        println!("[Skia CPU] Renderer initialized successfully!");

        Ok(Self {
            surface,
            width,
            height,
            softbuffer_surface,
            window,
            skia_renderer: SkiaRenderer::new(),
        })
    }
}

impl RenderBackend for SkiaCPURenderer {
    fn draw(&mut self, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        let canvas = self.surface.canvas();

        // Clear with background
        self.skia_renderer.clear(canvas, crate::core::Color::from_hex(0xFFFFFF));

        Ok(())
    }
    

    fn draw_render_object(&mut self, render_obj: &RenderObject, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        let canvas = self.surface.canvas();

        // Clear canvas
        self.skia_renderer.clear(canvas, crate::core::Color::from_hex(0xFFFFFF));

        // Actually render the widget tree!
        self.skia_renderer.render(canvas, render_obj);

        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        // Copy Skia surface pixels to softbuffer
        let image = self.surface.image_snapshot();
        let info = ImageInfo::new(
            (self.width as i32, self.height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        let row_bytes = (self.width * 4) as usize;
        let mut pixel_data = vec![0u8; (self.width * self.height * 4) as usize];

        if !image.read_pixels(&info, &mut pixel_data, row_bytes, (0, 0), CachingHint::Disallow) {
            return Err(anyhow!("Failed to read pixels from Skia surface"));
        }

        let _width_nz = NonZeroU32::new(self.width).ok_or_else(|| anyhow!("Width must be > 0"))?;
        let _height_nz = NonZeroU32::new(self.height).ok_or_else(|| anyhow!("Height must be > 0"))?;

        let mut buffer = self
            .softbuffer_surface
            .buffer_mut()
            .map_err(|e| anyhow!("Failed to get buffer: {}", e))?;

        // Convert RGBA to ARGB for softbuffer
        for (i, chunk) in pixel_data.chunks_exact(4).enumerate() {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3] as u32;

            buffer[i] = (a << 24) | (r << 16) | (g << 8) | b;
        }

        buffer
            .present()
            .map_err(|e| anyhow!("Failed to present buffer: {}", e))?;

        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.width = width.max(1);
        self.height = height.max(1);

        let info = ImageInfo::new(
            ISize::new(self.width as i32, self.height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        self.surface = skia_safe::surfaces::raster(&info, None, None)
            .ok_or_else(|| anyhow!("Failed to resize CPU surface"))?;

        self.softbuffer_surface
            .resize(
                NonZeroU32::new(self.width).unwrap(),
                NonZeroU32::new(self.height).unwrap(),
            )
            .map_err(|e| anyhow!("Failed to resize softbuffer: {}", e))?;

        Ok(())
    }

    fn cleanup(&mut self) {
        println!("[Skia CPU] Cleaning up renderer");
    }

    fn name(&self) -> &str {
        "Skia CPU"
    }
}
//! Rendering backend abstractions for OxideUI

pub mod skia_opengl;
pub mod skia_cpu;
pub mod softbuffer;
pub mod rendering_impl;
mod pipeline;
pub mod text;

pub use crate::render::text::{FontManager, TextLayout, TextCache, FontDescriptor, FontWeight, FontStyle};

use anyhow::Result;
use crate::core::RenderObject;

/// Core trait for all rendering backends
pub trait RenderBackend: Send {
    /// Draw a frame (fallback when no render object provided)
    fn draw(&mut self, width: u32, height: u32) -> Result<()>;

    /// Draw a complete render object tree - THIS IS THE REAL RENDERING
    fn draw_render_object(&mut self, _render_obj: &RenderObject, width: u32, height: u32) -> Result<()> {
        // Default implementation falls back to basic draw
        self.draw(width, height)
    }

    /// Present the rendered frame to screen
    fn present(&mut self) -> Result<()>;

    /// Resize the rendering surface
    fn resize(&mut self, width: u32, height: u32) -> Result<()>;

    /// Clean up rendering resources
    fn cleanup(&mut self);

    /// Get backend name for debugging
    fn name(&self) -> &str {
        "Unknown"
    }
}

/// Available renderer backends
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackendType {
    /// Skia with OpenGL acceleration
    SkiaOpenGL,
    /// Skia with CPU rasterization
    SkiaCPU,
    /// Pure software rendering
    Softbuffer,
}

/// Select the best available rendering backend
pub fn select_backend() -> BackendType {
    match std::env::var("OXIDEUI_RENDERER") {
        Ok(val) => match val.to_lowercase().as_str() {
            "skia" | "skia-opengl" | "opengl" | "gpu" => {
                println!("[Backend] User requested: Skia OpenGL");
                BackendType::SkiaOpenGL
            }
            "skia-cpu" | "cpu-skia" | "skia-cpu-fallback" => {
                println!("[Backend] User requested: Skia CPU");
                BackendType::SkiaCPU
            }
            "softbuffer" | "cpu" | "software" => {
                println!("[Backend] User requested: Softbuffer");
                BackendType::Softbuffer
            }
            _ => {
                eprintln!("[Backend] Unknown renderer '{}', defaulting to Softbuffer", val);
                BackendType::Softbuffer
            }
        },
        Err(_) => {
            // Auto-select based on available features
            #[cfg(feature = "skia-opengl")]
            {
                println!("[Backend] Auto-selected: Skia OpenGL (GPU accelerated)");
                BackendType::SkiaOpenGL
            }
            #[cfg(all(feature = "skia-cpu", not(feature = "skia-opengl")))]
            {
                println!("[Backend] Auto-selected: Skia CPU");
                BackendType::SkiaCPU
            }
            #[cfg(not(any(feature = "skia-opengl", feature = "skia-cpu")))]
            {
                println!("[Backend] Auto-selected: Softbuffer (fallback)");
                BackendType::Softbuffer
            }
        }
    }
}
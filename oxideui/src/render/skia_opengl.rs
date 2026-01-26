use anyhow::{Context, Result};
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use skia_safe::{gpu, ColorType, Surface};
use std::ffi::CString;
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use super::RenderBackend;
use crate::core::render_object::RenderObject;
use crate::render::rendering_impl::SkiaRenderer;
use winit::dpi::PhysicalSize;

pub struct SkiaOpenGLRenderer {
    gl_context: PossiblyCurrentContext,
    gl_surface: GlutinSurface<WindowSurface>,
    skia_context: gpu::DirectContext,
    skia_surface: Option<Surface>,
    skia_renderer: SkiaRenderer,
    width: u32,
    height: u32,
    window: Arc<Window>,
}

unsafe impl Send for SkiaOpenGLRenderer {}

impl SkiaOpenGLRenderer {
    pub fn new(window: Arc<Window>, event_loop: &ActiveEventLoop) -> Result<Self> {
        println!("[Skia OpenGL] Initializing renderer...");
        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        println!("[Skia OpenGL] Window size: {}x{}", width, height);

        // WAYLAND COMPATIBLE CONFIG
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_depth_size(24)
            .with_stencil_size(8)
            .with_transparency(false)
            .prefer_hardware_accelerated(Some(true));

        println!("[Skia OpenGL] Creating display...");
        let display_builder = DisplayBuilder::new().with_window_attributes(None);

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs: Box<dyn Iterator<Item = glutin::config::Config>>| {
                configs
                    .reduce(|accum: glutin::config::Config, config: glutin::config::Config| {
                        let transparency = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);
                        if transparency || config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .expect("No suitable GL config found")
            })
            .map_err(|e| anyhow::anyhow!("Failed to build display: {}", e))?;

        if let Some(_w) = &window {
            // Drop the old window reference if needed
            println!("[Skia OpenGL] Using existing window");
        }

        println!("[Skia OpenGL] Display created");
        println!("[Skia OpenGL] Config: samples={}, stencil={}, depth={}",
                 gl_config.num_samples(),
                 gl_config.stencil_size(),
                 gl_config.depth_size());

        let raw_window_handle = match &window {
            Some(w) => w.window_handle()
                .context("Failed to get window handle")?
                .as_raw(),
            None => {
                return Err(anyhow::anyhow!("Window is required"));
            }
        };

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::Gles(Some(
                glutin::context::Version::new(3, 0),
            )))
            .build(Some(raw_window_handle));

        println!("[Skia OpenGL] Creating GL context...");
        let gl_display = gl_config.display();
        let gl_context = unsafe {
            gl_display.create_context(&gl_config, &context_attributes)
        }.or_else(|_| {
            println!("[Skia OpenGL] GLES failed, trying OpenGL 3.3...");
            let attrs = ContextAttributesBuilder::new()
                .with_context_api(glutin::context::ContextApi::OpenGl(Some(
                    glutin::context::Version::new(3, 3),
                )))
                .build(Some(raw_window_handle));
            unsafe { gl_display.create_context(&gl_config, &attrs) }
        })
            .context("Failed to create GL context")?;

        let size = window.as_ref().unwrap().inner_size();
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new()
            .build(
                raw_window_handle,
                NonZeroU32::new(size.width.max(1)).unwrap(),
                NonZeroU32::new(size.height.max(1)).unwrap(),
            );

        println!("[Skia OpenGL] Creating window surface...");
        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &surface_attributes)
                .context("Failed to create window surface")?
        };

        let gl_context = gl_context
            .make_current(&gl_surface)
            .context("Failed to make context current")?;

        println!("[Skia OpenGL] Loading GL functions...");
        gl::load_with(|symbol| {
            let cstr = CString::new(symbol).unwrap();
            gl_display.get_proc_address(cstr.as_c_str()) as *const _
        });

        println!("[Skia OpenGL] Creating Skia GL interface...");
        let interface = gpu::gl::Interface::new_load_with(|name| {
            let cstr = CString::new(name).unwrap();
            gl_display.get_proc_address(cstr.as_c_str())
        })
            .context("Failed to create Skia GL interface")?;

        println!("[Skia OpenGL] Creating Skia DirectContext...");
        let skia_context = gpu::direct_contexts::make_gl(interface, None)
            .context("Failed to create Skia DirectContext")?;

        // Initialize Skia surface later after resize
        println!("[Skia OpenGL] Renderer initialized successfully!");

        // Since the window is passed separately to the function, we need to handle this differently
        // Let's create a new approach - we'll need to pass the window separately
        // For now, let's fix this by using the original window parameter
        let actual_window = match window {
            Some(w) => w,
            None => return Err(anyhow::anyhow!("Window is required")),
        };

        Ok(Self {
            gl_context,
            gl_surface,
            skia_context,
            skia_surface: None,
            skia_renderer: SkiaRenderer::new(),
            width: width as u32,
            height: height as u32,
            window: actual_window.into(), // Convert Window to Arc<Window>
        })
    }

    fn recreate_skia_surface(&mut self) -> Result<()> {
        let size = self.window.inner_size();
        let width = size.width.max(1) as i32;
        let height = size.height.max(1) as i32;

        let mut fboid: i32 = 0;
        unsafe {
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid);
        }

        let fb_info = gpu::gl::FramebufferInfo {
            fboid: fboid as u32,
            format: gpu::gl::Format::RGBA8.into(),
            ..Default::default()
        };

        let samples = 0; // No MSAA for now
        let stencil = 8;

        let backend_render_target = gpu::backend_render_targets::make_gl(
            (width, height),
            samples,
            stencil,
            fb_info,
        );

        self.skia_surface = Some(gpu::surfaces::wrap_backend_render_target(
            &mut self.skia_context,
            &backend_render_target,
            gpu::SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        ).context("Failed to create Skia surface")?);

        Ok(())
    }
}

impl RenderBackend for SkiaOpenGLRenderer {
    fn draw(&mut self, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        if self.skia_surface.is_none() {
            self.recreate_skia_surface()?;
        }

        if let Some(ref mut surface) = self.skia_surface {
            let canvas = surface.canvas();
            self.skia_renderer.clear(canvas, crate::core::Color::from_hex(0xFFFFFF));
            self.skia_context.flush_and_submit();
        }

        Ok(())
    }

    fn draw_render_object(&mut self, render_obj: &RenderObject, width: u32, height: u32) -> Result<()> {
        if width != self.width || height != self.height {
            self.resize(width, height)?;
        }

        if self.skia_surface.is_none() {
            self.recreate_skia_surface()?;
        }

        if let Some(ref mut surface) = self.skia_surface {
            let canvas = surface.canvas();
            self.skia_renderer.clear(canvas, crate::core::Color::from_hex(0xFFFFFF));
            self.skia_renderer.render(canvas, render_obj);
            self.skia_context.flush_and_submit();
        }

        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        self.gl_surface
            .swap_buffers(&self.gl_context)
            .context("Failed to swap buffers")?;
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        let width = width.max(1);
        let height = height.max(1);
        println!("[Skia OpenGL] Resizing to {}x{}", width, height);

        self.width = width;
        self.height = height;

        let _size = PhysicalSize::new(width, height);
        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        // Recreate Skia surface with new dimensions
        self.recreate_skia_surface()?;

        Ok(())
    }

    fn cleanup(&mut self) {
        println!("[Skia OpenGL] Cleaning up renderer");
        self.skia_surface = None;
    }

    fn name(&self) -> &str {
        "Skia OpenGL"
    }
}
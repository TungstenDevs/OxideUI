mod widget_builder;
use anyhow::{Context, Result};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent, ElementState, MouseButton};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use winit_input_helper::WinitInputHelper;
use crate::core::element::SharedElementTree;
use crate::core::widget::Widget;
use crate::core::{EventDispatcher, Theme};
use crate::layout::Constraints;
use crate::render::{select_backend, BackendType, RenderBackend};
use crate::theming::ThemeConfig;
use widget_builder::WidgetBuilder;
use std::time::Instant;
use oneshot;
pub struct Runtime {
    event_loop: Option<EventLoop<()>>,
    root_widget: Option<Box<dyn Widget>>,
    title: String,
    width: u32,
    height: u32,
    theme_config: Option<ThemeConfig>,
}

impl Runtime {
    pub fn new(root_widget: Box<dyn Widget>) -> Self {
        Self {
            event_loop: Some(EventLoop::new().unwrap()),
            root_widget: Some(root_widget),
            title: "OxideUI Application".to_string(),
            width: 800,
            height: 600,
            theme_config: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_theme(mut self, theme: ThemeConfig) -> Self {
        self.theme_config = Some(theme);
        self
    }

    pub async fn run(self) -> Result<()> {
        let event_loop = self.event_loop.context("Event loop was taken")?;
        let root_widget = self.root_widget.context("Root widget was taken")?;
        let (tx, rx) = oneshot::channel::<()>();

        let mut app = OxideApp {
            window: None,
            renderer: None,
            backend_type: select_backend(),
            input: WinitInputHelper::new(),
            event_dispatcher: EventDispatcher::new(),
            element_tree: crate::core::element::new_shared_element_tree(),
            exit_tx: Some(tx),
            root_widget,
            theme_config: self.theme_config,
            title: self.title,
            width: self.width,
            height: self.height,
            theme: Arc::new(Theme::default()),
            last_frame_time: Instant::now(),
            frame_count: 0,
        };

        println!("🎨 OxideUI Framework Starting...");
        println!("📦 Selected renderer: {:?}", app.backend_type);
        println!(
            "🪟 Window: \"{}\" ({}x{})",
            app.title, app.width, app.height
        );

        event_loop
            .run_app(&mut app)
            .context("Failed to run application event loop")?;

        rx.await.context("Event loop shutdown channel failed")?;
        Ok(())
    }
}

struct OxideApp {
    window: Option<Arc<Window>>,
    renderer: Option<Box<dyn RenderBackend>>,
    backend_type: BackendType,
    input: WinitInputHelper,
    event_dispatcher: EventDispatcher,
    element_tree: SharedElementTree,
    exit_tx: Option<oneshot::Sender<()>>,
    root_widget: Box<dyn Widget>,
    theme_config: Option<ThemeConfig>,
    title: String,
    width: u32,
    height: u32,
    theme: Arc<Theme>,
    last_frame_time: Instant,
    frame_count: u64,
}

impl ApplicationHandler for OxideApp {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        self.input.step();
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            println!(
                "🪟 Creating window: \"{}\" ({}x{})",
                self.title, self.width, self.height
            );

            let window_attributes = WindowAttributes::default()
                .with_title(&self.title)
                .with_inner_size(LogicalSize::new(self.width, self.height))
                .with_visible(true)
                .with_resizable(true)
                .with_decorations(true)
                .with_transparent(false);

            match event_loop.create_window(window_attributes) {
                Ok(window) => {
                    println!("✅ Window created successfully");
                    if let Some(config) = &self.theme_config {
                        self.theme = Arc::new(Theme::from_config(config, false));
                        println!("🎨 Theme loaded: {}", config.font_sans);
                    }

                    let window_arc = Arc::new(window);
                    self.window = Some(window_arc.clone());

                    // Create renderer based on backend type
                    let renderer = match self.backend_type {
                        BackendType::SkiaOpenGL => {
                            #[cfg(feature = "skia-opengl")]
                            {
                                use crate::render::skia_opengl::SkiaOpenGLRenderer;
                                match SkiaOpenGLRenderer::new(window_arc.clone(), event_loop) {
                                    Ok(r) => Ok(Box::new(r) as Box<dyn RenderBackend>),
                                    Err(e) => Err(e),
                                }
                            }
                            #[cfg(not(feature = "skia-opengl"))]
                            {
                                // Fallback when skia-opengl feature is not enabled
                                Err(anyhow::anyhow!("SkiaOpenGL renderer not available - skia-opengl feature not enabled"))
                            }
                        }
                        BackendType::SkiaCPU => {
                            use crate::render::skia_cpu::SkiaCPURenderer;
                            match SkiaCPURenderer::new(window_arc) {
                                Ok(r) => Ok(Box::new(r) as Box<dyn RenderBackend>),
                                Err(e) => Err(e),
                            }
                        }
                        BackendType::Softbuffer => {
                            use crate::render::softbuffer::SoftbufferRenderer;
                            match SoftbufferRenderer::new(window_arc.clone()) {
                                Ok(r) => Ok(Box::new(r) as Box<dyn RenderBackend>),
                                Err(e) => Err(e),
                            }
                        }
                    };

                    match renderer {
                        Ok(renderer) => {
                            println!("✅ Renderer ({}) initialized", renderer.name());
                            self.renderer = Some(renderer);
                            if let Some(window) = &self.window {
                                window.request_redraw();
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to create renderer: {}", e);
                            event_loop.exit();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to create window: {}", e);
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.input.process_window_event(&event) {
            self.process_input_events();
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("🛑 Close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.rebuild_and_render();
            }
            WindowEvent::Resized(size) => {
                println!("📐 Window resized to: {}x{}", size.width, size.height);
                if let Some(renderer) = &mut self.renderer {
                    if let Err(e) = renderer.resize(size.width, size.height) {
                        eprintln!("❌ Resize error: {}", e);
                    }
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state: ElementState::Pressed,
                ..
            } => {
                self.process_mouse_click();
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        self.input.process_device_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.input.end_step();
        if self.input.close_requested() || self.input.destroyed() {
            println!("🛑 Application exit requested");
            event_loop.exit();
            return;
        }

        // Request redraw for animation frames
        if let Some(window) = &self.window {
            // Check for any key press using the correct method
            use winit::keyboard::KeyCode;
            if self.input.key_pressed(KeyCode::Space) ||
               self.input.key_pressed(KeyCode::Enter) ||
               self.input.key_pressed(KeyCode::ArrowUp) ||
               self.input.key_pressed(KeyCode::ArrowDown) ||
               self.input.key_pressed(KeyCode::ArrowLeft) ||
               self.input.key_pressed(KeyCode::ArrowRight) {
                window.request_redraw();
            }
        }

        // Calculate and display FPS every 60 frames
        self.frame_count += 1;
        if self.frame_count % 60 == 0 {
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_frame_time);
            let fps = 60.0 / elapsed.as_secs_f32();
            println!("📊 FPS: {:.1}", fps);
            self.last_frame_time = now;
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("👋 Application exiting...");
        if let Some(mut renderer) = self.renderer.take() {
            renderer.cleanup();
        }
        if let Some(tx) = self.exit_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl OxideApp {
    fn process_input_events(&mut self) {
        // Process keyboard events - checking for specific keys instead of generic key_pressed
        use winit::keyboard::KeyCode;
        if self.input.key_pressed(KeyCode::Space) {
            println!("⌨️ Space key pressed");
        }
        if self.input.key_pressed(KeyCode::Enter) {
            println!("⌨️ Enter key pressed");
        }

        // Process mouse events
        let (x, y) = self.input.mouse_diff();
        if self.input.mouse_pressed(winit::event::MouseButton::Left) {
            println!("🖱️ Mouse pressed at: ({}, {})", x, y);
        }
    }

    fn process_mouse_click(&mut self) {
        let (x, y) = self.input.mouse_diff();
        println!("🖱️ Click detected at: ({}, {})", x, y);

        // This is where you'd trigger widget interactions
        // For now, just force a rebuild to show we're responding
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn rebuild_and_render(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            let size = if let Some(window) = &self.window {
                window.inner_size()
            } else {
                return;
            };

            let constraints = Constraints::new(
                0.0, size.width as f32,
                0.0, size.height as f32
            );

            let builder = WidgetBuilder::new(self.theme.clone());
            let root_render_obj = builder.build_widget_tree(&self.root_widget, constraints);

            println!("🎨 Rendering frame with constraints: {:?}", constraints);

            if let Err(e) = renderer.draw_render_object(&root_render_obj, size.width, size.height) {
                eprintln!("❌ Draw error: {}", e);
                return;
            }

            if let Err(e) = renderer.present() {
                eprintln!("❌ Present error: {}", e);
                return;
            }

            if let Some(window) = &self.window {
                window.pre_present_notify();
            }
        }
    }
}
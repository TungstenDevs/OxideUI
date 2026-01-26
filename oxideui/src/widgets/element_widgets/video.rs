use crate::core::context::{BuildContext, ThemeProvider};
use crate::core::render_object::{Color, Point, Rect, RenderObject, TextStyle};
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct Video {
    pub source: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub autoplay: bool,
    pub controls: bool,
    pub loop_playback: bool,
    pub muted: bool,
    pub on_play: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_pause: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_ended: Option<Arc<dyn Fn() + Send + Sync>>,
    key: Option<WidgetKey>,
}

impl Video {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            width: None,
            height: None,
            autoplay: false,
            controls: true,
            loop_playback: false,
            muted: false,
            on_play: None,
            on_pause: None,
            on_ended: None,
            key: None,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn autoplay(mut self, autoplay: bool) -> Self {
        self.autoplay = autoplay;
        self
    }

    pub fn controls(mut self, controls: bool) -> Self {
        self.controls = controls;
        self
    }

    pub fn loop_playback(mut self, loop_playback: bool) -> Self {
        self.loop_playback = loop_playback;
        self
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn with_on_play<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_play = Some(Arc::new(callback));
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Video {
    fn build_stateless(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();
        let width = self.width.unwrap_or(640.0);
        let height = self.height.unwrap_or(360.0);

        let mut render_objects = Vec::new();

        // Video placeholder (actual video rendering would need platform integration)
        render_objects.push(RenderObject::rect(
            Rect::new(0.0, 0.0, width, height),
            Color::from_hex(0x000000),
        ));

        // Play icon overlay
        render_objects.push(RenderObject::text(
            "▶".to_string(),
            TextStyle {
                font_family: theme.font_sans.clone(),
                font_size: 48.0,
                color: Color::WHITE,
                bold: false,
                italic: false,
            },
            Point::new(width / 2.0 - 24.0, height / 2.0 + 16.0),
        ));

        WidgetNode::Leaf(RenderObject::group(render_objects))
    }
}

impl Widget for Video {
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

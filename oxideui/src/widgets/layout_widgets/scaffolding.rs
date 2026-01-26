use std::any::Any;
use crate::core::context::BuildContext;
use crate::core::widget::{StatelessWidget, Widget, WidgetKey, WidgetNode};

pub struct Scaffolding {
    pub app_bar: Option<Box<dyn Widget>>,
    pub sidebar: Option<Box<dyn Widget>>,
    pub content: Box<dyn Widget>,
    pub footer: Option<Box<dyn Widget>>,
    pub drawer: Option<Box<dyn Widget>>,
    key: Option<WidgetKey>,
}

impl Scaffolding {
    pub fn new(content: Box<dyn Widget>) -> Self {
        Self {
            app_bar: None,
            sidebar: None,
            content,
            footer: None,
            drawer: None,
            key: None,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            app_bar: self.app_bar.as_ref().map(|w| w.clone_box()),
            sidebar: self.sidebar.as_ref().map(|w| w.clone_box()),
            content: self.content.clone_box(),
            footer: self.footer.as_ref().map(|w| w.clone_box()),
            drawer: self.drawer.as_ref().map(|w| w.clone_box()),
            key: self.key.clone(),
        }
    }

    pub fn with_app_bar(mut self, app_bar: Box<dyn Widget>) -> Self {
        self.app_bar = Some(app_bar);
        self
    }

    pub fn with_sidebar(mut self, sidebar: Box<dyn Widget>) -> Self {
        self.sidebar = Some(sidebar);
        self
    }

    pub fn with_footer(mut self, footer: Box<dyn Widget>) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn with_drawer(mut self, drawer: Box<dyn Widget>) -> Self {
        self.drawer = Some(drawer);
        self
    }

    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }
}

impl StatelessWidget for Scaffolding {
    fn build_stateless(&self, _ctx: &BuildContext) -> WidgetNode {
        // This is a layout widget that arranges app bar, sidebar, content, and footer
        // In a real implementation, we would calculate the layout

        let mut children = Vec::new();

        // Add app bar if present
        if let Some(app_bar) = &self.app_bar {
            children.push(app_bar.clone_box());
        }

        // Add sidebar if present
        if let Some(sidebar) = &self.sidebar {
            children.push(sidebar.clone_box());
        }

        // Add content
        children.push(self.content.clone_box());

        // Add footer if present
        if let Some(footer) = &self.footer {
            children.push(footer.clone_box());
        }

        // Add drawer if present (drawn on top)
        if let Some(drawer) = &self.drawer {
            children.push(drawer.clone_box());
        }

        WidgetNode::Container { children }
    }
}

impl Widget for Scaffolding {
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
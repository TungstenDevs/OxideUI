use oxideui::*;
use oxideui::widgets::basic::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

struct ComprehensiveDemo {
    click_count: Arc<AtomicU32>,
}

impl ComprehensiveDemo {
    fn new() -> Self {
        Self {
            click_count: Arc::new(AtomicU32::new(0)),
        }
    }
}

impl Widget for ComprehensiveDemo {
    fn build(&self, ctx: &BuildContext) -> WidgetNode {
        let theme = ctx.theme();

        WidgetNode::Container {
            children: vec![
                Box::new(
                    Scaffolding::new(
                        Box::new(
                            Container::new()
                                .with_color(theme.background)
                                .with_child(
                                    ScrollArea::new(
                                        Box::new(
                                            Container::new()
                                                .with_padding(24.0)
                                                .with_child(
                                                    Column::new().with_children(vec![
                                                        // Header
                                                        Box::new(h1("🎨 OxideUI Widget Showcase").with_color(theme.primary)),
                                                        spacer(16.0),
                                                        Box::new(Text::new("A comprehensive demonstration of all available widgets").with_color(theme.muted_foreground)),
                                                        spacer(8.0),
                                                        Box::new(Text::new(format!("Click count: {}", self.click_count.load(Ordering::SeqCst))).with_color(theme.foreground)),
                                                        spacer(32.0),

                                                        // Basic Widgets Section
                                                        self.build_basic_section(ctx),
                                                        spacer(32.0),

                                                        // Interactive Section
                                                        self.build_interactive_section(ctx),
                                                        spacer(32.0),

                                                        // Layout Section
                                                        self.build_layout_section(ctx),
                                                    ])
                                                )
                                        )
                                    )
                                )
                        )
                    )
                )
            ]
        }
    }

    fn key(&self) -> Option<WidgetKey> {
        None
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Widget> {
        Box::new(Self {
            click_count: self.click_count.clone(),
        })
    }
}

impl ComprehensiveDemo {
    fn build_basic_section(&self, ctx: &BuildContext) -> Box<dyn Widget> {
        let theme = ctx.theme();
        Box::new(
            Card::new()
                .with_title("📝 Basic Widgets")
                .with_padding(24.0)
                .with_children(vec![
                    Box::new(Column::new().with_children(vec![
                        Box::new(h2("Headings")),
                        spacer(8.0),
                        Box::new(h1("This is H1")),
                        Box::new(h2("This is H2")),
                        Box::new(h3("This is H3")),
                        spacer(16.0),

                        Box::new(Label::new("Labels and Text:").bold()),
                        spacer(4.0),
                        Box::new(Text::new("Regular text with default styling")),
                        Box::new(Label::new("Bold label").bold()),
                        spacer(16.0),

                        Box::new(Label::new("Container with border:").bold()),
                        spacer(8.0),
                        Box::new(
                            Container::new()
                                .with_color(theme.muted)
                                .with_size(300.0, 80.0)
                                .with_border_radius(8.0)
                                .with_border(2.0, theme.border)
                                .with_child(
                                    Center::new()
                                        .with_child(
                                            Text::new("Centered in container")
                                                .with_color(theme.foreground)
                                        )
                                )
                        ),
                    ]))
                ])
        )
    }

    fn build_interactive_section(&self, ctx: &BuildContext) -> Box<dyn Widget> {
        let theme = ctx.theme();
        let click_count = self.click_count.clone();

        Box::new(
            Card::new()
                .with_title("🎮 Interactive Controls")
                .with_padding(24.0)
                .with_children(vec![
                    Box::new(Column::new().with_children(vec![
                        Box::new(Label::new("Buttons:").bold()),
                        spacer(12.0),
                        Box::new(Row::new().with_children(vec![
                            Box::new(
                                Button::new("Click Me!")
                                    .with_color(theme.primary)
                                    .with_text_color(theme.primary_foreground)
                                    .with_size(120.0, 40.0)
                                    .with_on_click(move || {
                                        click_count.fetch_add(1, Ordering::SeqCst);
                                        println!("Button clicked!");
                                    })
                            ),
                            spacer_horizontal(12.0),
                            Box::new(
                                Button::new("Secondary")
                                    .with_color(theme.secondary)
                                    .with_text_color(theme.secondary_foreground)
                                    .with_size(120.0, 40.0)
                            ),
                            spacer_horizontal(12.0),
                            Box::new(
                                Button::new("Accent")
                                    .with_color(theme.accent)
                                    .with_text_color(theme.accent_foreground)
                                    .with_size(120.0, 40.0)
                            ),
                        ])),
                        spacer(24.0),

                        Box::new(Label::new("Progress Bar:").bold()),
                        spacer(8.0),
                        Box::new(
                            ProgressBar::new(65.0, 100.0)
                                .with_size(400.0, 12.0)
                                .show_value(true)
                        ),
                        spacer(16.0),

                        Box::new(Label::new("Checkboxes:").bold()),
                        spacer(8.0),
                        Box::new(Column::new().with_children(vec![
                            Box::new(
                                Checkbox::new()
                                    .with_label("Option 1")
                                    .checked(true)
                            ),
                            spacer(6.0),
                            Box::new(
                                Checkbox::new()
                                    .with_label("Option 2")
                                    .checked(false)
                            ),
                            spacer(6.0),
                            Box::new(
                                Checkbox::new()
                                    .with_label("Option 3")
                                    .checked(true)
                            ),
                        ])),
                    ]))
                ])
        )
    }

    fn build_layout_section(&self, ctx: &BuildContext) -> Box<dyn Widget> {
        let theme = ctx.theme();

        Box::new(
            Card::new()
                .with_title("📐 Layout Examples")
                .with_padding(24.0)
                .with_children(vec![
                    Box::new(Column::new().with_children(vec![
                        Box::new(Label::new("Flexbox Row:").bold()),
                        spacer(12.0),
                        Box::new(
                            Flexbox::new()
                                .direction(FlexDirection::Row)
                                .justify(JustifyContent::SpaceBetween)
                                .gap(12.0)
                                .with_children(vec![
                                    demo_box(ctx, "Box 1", 100.0, 60.0, theme.chart_1),
                                    demo_box(ctx, "Box 2", 100.0, 60.0, theme.chart_2),
                                    demo_box(ctx, "Box 3", 100.0, 60.0, theme.chart_3),
                                ])
                        ),
                        spacer(24.0),

                        Box::new(Label::new("Grid Layout:").bold()),
                        spacer(12.0),
                        Box::new(
                            Grid::new()
                                .columns(3)
                                .gap(12.0)
                                .with_children(vec![
                                    demo_box(ctx, "1", 90.0, 60.0, theme.chart_1),
                                    demo_box(ctx, "2", 90.0, 60.0, theme.chart_2),
                                    demo_box(ctx, "3", 90.0, 60.0, theme.chart_3),
                                    demo_box(ctx, "4", 90.0, 60.0, theme.chart_4),
                                    demo_box(ctx, "5", 90.0, 60.0, theme.chart_5),
                                    demo_box(ctx, "6", 90.0, 60.0, theme.chart_1),
                                ])
                        ),
                    ]))
                ])
        )
    }
}

// Helper functions
fn spacer(height: f32) -> Box<dyn Widget> {
    Box::new(Container::new().with_size(1.0, height))
}

fn spacer_horizontal(width: f32) -> Box<dyn Widget> {
    Box::new(Container::new().with_size(width, 1.0))
}

fn demo_box(_ctx: &BuildContext, label: &str, width: f32, height: f32, color: Color) -> Box<dyn Widget> {
    Box::new(
        Container::new()
            .with_color(color)
            .with_size(width, height)
            .with_border_radius(8.0)
            .with_child(
                Center::new()
                    .with_child(
                        Text::new(label)
                            .with_color(Color::WHITE)
                    )
            )
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🎨 OxideUI Widget Showcase");
    println!("==========================");
    println!();
    println!("Demonstrating core widgets:");
    println!("- Basic: Text, Labels, Containers, Headings");
    println!("- Interactive: Buttons, Progress, Checkboxes");
    println!("- Layout: Flexbox, Grid, Cards");
    println!();
    println!("Backend: Auto-selected (Skia CPU or Softbuffer)");
    println!("Click the button to increment the counter!");
    println!();

    let app = ComprehensiveDemo::new();

    oxideui::new(app)
        .with_title("OxideUI - Widget Showcase")
        .with_size(1000, 800)
        .run()
        .await
}
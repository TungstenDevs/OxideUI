_* Desktop-first mainstream adoption
* **GPU-accelerated first**, **CPU fallback**
* **winit** windowing (Wayland / X11 / Windows / macOS)
* **skia-safe** rendering (GL / Metal / CPU raster)
* **softbuffer** final fallback
* **async + multithreaded** runtime (Tokio)
* Declarative UI model (stateful / stateless widgets)
* Retained-mode rendering (not immediate mode)
* Rust-idiomatic (no fake OOP inheritance)

This is a **serious systems architecture**, not a toy framework.

---

# System Objectives

| Requirement    | Decision                         |
|----------------|----------------------------------|
| Declarative UI | Widget tree + diffing            |
| Retained mode  | Element tree                     |
| GPU rendering  | Skia (GL / Metal)                |
| CPU fallback   | Skia CPU → Softbuffer            |
| Async runtime  | Tokio (non-blocking UI thread)   |
| Windowing      | winit                            |
| Input          | winit + raw-input                |
| State model    | Externalized state               |
| Multithreading | Work stealing scheduler          |
| Platform       | Desktop first, mobile extensible |

---

# Global Architecture

```text
┌────────────────────────────────────────────────────────────┐
│                        Application                         │
│          (User Widgets, State, Async Tasks, Logic)          │
└───────────────▲──────────────────────────▲────────────────┘
                │                          │
        Declarative Widgets           Async Runtime
                │                          │
┌───────────────┴──────────────────────────┴────────────────┐
│                    Framework Core                          │
│                                                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │ Widget API   │  │  State Mgmt  │  │  Event Sys   │    │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘    │
│         │                 │                 │            │
│  ┌──────▼──────────┐  ┌───▼───────────┐  ┌──▼─────────┐ │
│  │  Element Tree   │  │  Reconciler    │  │ Hit Testing │ │
│  │ (Retained Mode) │  │  (Diff Engine) │  │ + Routing   │ │
│  └──────┬──────────┘  └──────┬────────┘  └────┬────────┘ │
│         │                    │                 │          │
│  ┌──────▼────────────────────▼─────────────────▼────────┐│
│  │                    Layout Engine                      ││
│  │        (Constraints → Measure → Position)              ││
│  └───────────────┬──────────────────────────────────────┘│
│                  │
│  ┌───────────────▼──────────────────────────────────────┐│
│  │                    Render Engine                      ││
│  │     Scene Graph → Display List → Backend Renderer      ││
│  └───────────────┬──────────────────────────────────────┘│
└──────────────────┼────────────────────────────────────────┘
                   │
┌──────────────────▼────────────────────────────────────────┐
│                Platform / System Layer                     │
│                                                            │
│   winit (Wayland / X11 / Windows / macOS)                   │
│   raw-input / clipboard / IME                               │
│                                                            │
│   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐     │
│   │ Skia GL       │ │ Skia CPU     │ │ Softbuffer   │     │
│   │ Skia Metal    │ │ Rasterizer   │ │ Fallback     │     │
│   └──────────────┘ └──────────────┘ └──────────────┘     │
└────────────────────────────────────────────────────────────┘
```

---

# Core Data Model (Three-Tree Model)

This is **mandatory** for correctness and scalability.

| Tree             | Role                    | Mutability        |
|------------------|-------------------------|-------------------|
| **Widget Tree**  | Declarative user intent | Immutable         |
| **Element Tree** | Live runtime instances  | Mutable           |
| **Render Tree**  | GPU primitives          | Rebuilt per frame |

```text
Widget  →  Element  →  RenderObject
```

---

# Widget System Design

## Widget Trait (Declarative Layer)

```rust
pub trait Widget: Send + Sync {
    fn build(&self, ctx: &BuildContext) -> WidgetNode;
}
```

Widgets are:

* Immutable
* Cheap
* Rebuildable
* Stateless by default

---

## Stateless Widget

```rust
pub struct Text {
    pub value: String,
}

impl Widget for Text {
    fn build(&self, _: &BuildContext) -> WidgetNode {
        WidgetNode::Leaf(RenderObject::Text {
            content: self.value.clone(),
        })
    }
}
```

---

## Stateful Widget (Rust-Correct Model)

State is **external**, owned by the element.

```rust
pub trait StatefulWidget {
    type State: WidgetState;

    fn create_state(&self) -> Self::State;
    fn build(&self, state: &Self::State, ctx: &BuildContext) -> WidgetNode;
}
```

---

# Element Tree (Retained Mode)

```rust
pub struct Element {
    pub id: ElementId,
    pub widget_type: TypeId,
    pub state: Option<Box<dyn Any + Send>>,
    pub children: Vec<ElementId>,
    pub render_object: RenderObject,
    pub dirty: bool,
}
```

Responsibilities:

* Holds state
* Lifecycle
* Dirty flags
* Reconciliation target
* Event routing node

---

# Reconciliation Engine (Diffing)

```text
Old Widget Tree
        +
New Widget Tree
        ↓
   Reconciler
        ↓
 Element Tree Mutations
```

Rules:

1. Same type + same key → reuse element
2. Different type → destroy subtree
3. Children diffed by order or key
4. State preserved on reuse

This is **React diffing**, not DOM diffing.

---

# Layout Engine

## Constraint Model

```rust
pub struct Constraints {
    pub min_w: f32,
    pub max_w: f32,
    pub min_h: f32,
    pub max_h: f32,
}

pub struct Size {
    pub w: f32,
    pub h: f32,
}
```

## Layout Protocol

```text
Parent passes Constraints
        ↓
Child measures
        ↓
Child returns Size
        ↓
Parent positions child
```

This enables:

* Flex
* Stack
* Grid
* Absolute layout
* Responsive UI

---

# Rendering Pipeline

```text
Element Tree
    ↓
Render Tree
    ↓
Scene Graph
    ↓
Display List
    ↓
Backend Renderer
```

---

## RenderObject

```rust
pub enum RenderObject {
    Rect { size: Size, paint: Paint },
    Text { glyphs: Vec<Glyph> },
    Image { handle: ImageHandle },
    Clip { child: Box<RenderObject> },
    Transform { matrix: Matrix, child: Box<RenderObject> },
}
```

Backend-agnostic primitives.

---

# Backend Selection Strategy

```text
Startup:
    try Skia GL
        ↓ fail
    try Skia Metal (macOS)
        ↓ fail
    try Skia CPU
        ↓ fail
    Softbuffer fallback
```

---

## Backend Trait

```rust
pub trait RendererBackend {
    fn init(&mut self);
    fn begin_frame(&mut self);
    fn render(&mut self, scene: &Scene);
    fn end_frame(&mut self);
}
```

Implementations:

* `SkiaGlBackend`
* `SkiaMetalBackend`
* `SkiaCpuBackend`
* `SoftbufferBackend`

---

# Event System

## Flow

```text
winit event
    ↓
normalize
    ↓
hit test
    ↓
dispatch
    ↓
widget callback
```

---

## Event Types

```rust
pub enum UiEvent {
    PointerDown,
    PointerUp,
    PointerMove,
    Scroll,
    KeyDown,
    KeyUp,
    Focus,
    Blur,
}
```

---

# Async Integration Model

**Critical rule:**

> UI thread is never blocked by async.

## Model

```text
UI Thread
    |
    | schedules
    ↓
Tokio Runtime Threads
    |
    | wakes state
    ↓
Dirty Element Flag
    ↓
Next Frame Rebuild
```

---

## Hook Example

```rust
let data = use_future( | | async {
fetch_api().await
});
```

Flow:

1. Task runs in Tokio
2. Result stored in state
3. Element marked dirty
4. Rebuild on next frame

---

# Frame Loop (winit)

```text
EventLoop
    ↓
Input collection
    ↓
State updates
    ↓
Reconciliation
    ↓
Layout
    ↓
Render
    ↓
Present
```

---

# Multithreading Model

| Thread        | Role                  |
|---------------|-----------------------|
| UI thread     | winit + render submit |
| Render thread | GPU submission        |
| Worker pool   | layout / diff / async |
| Tokio threads | async IO              |

---

# Crate Layout

```text
oxideui/
├── core/
│   ├── widget.rs
│   ├── element.rs
│   ├── state.rs
│   ├── reconcile.rs
│   ├── diff.rs
│   └── context.rs
│
├── layout/
│   ├── constraints.rs
│   ├── flex.rs
│   ├── stack.rs
│   └── grid.rs
│
├── render/
│   ├── scene.rs
│   ├── display_list.rs
│   ├── paint.rs
│   ├── image.rs
│   └── backend/
│       ├── mod.rs
│       ├── skia_gl.rs
│       ├── skia_metal.rs
│       ├── skia_cpu.rs
│       └── softbuffer.rs
│
├── platform/
│   ├── winit.rs
│   ├── input.rs
│   ├── clipboard.rs
│   └── ime.rs
│
├── runtime/
│   ├── scheduler.rs
│   ├── hooks.rs
│   └── async_bridge.rs
│
└── widgets/
    ├── text.rs
    ├── button.rs
    ├── row.rs
    ├── column.rs
    ├── stack.rs
    ├── image.rs
    └── scroll.rs
```

---

# Why This Architecture Works

| Problem             | Solved by                    |
|---------------------|------------------------------|
| GPU fallback        | Multi-backend renderer       |
| Rust OOP limits     | Trait + data-oriented design |
| Async UI corruption | Dirty-flag + scheduler       |
| Performance         | Retained mode                |
| Scalability         | Diffing + layout separation  |
| Portability         | winit + skia                 |
| Future mobile       | Scene graph abstraction      |

---

# Roadmap Order (Correct Build Sequence)

1. **winit + skia minimal render loop**
2. Backend abstraction
3. RenderObject model
4. Element tree
5. Widget tree
6. Reconciler
7. Layout engine
8. Event system
9. State system
10. Async hooks
11. Scheduler
12. Damage regions
13. Accessibility
14. IME
15. Text shaping (HarfBuzz)

---

## Next Step Options (pick one)

I will implement **real Rust code skeletons**, not pseudocode:

1. winit + skia backend bootstrap
2. Widget / Element / RenderObject core
3. Reconciler engine
4. Layout engine
5. Async scheduler + hooks
6. Full minimal example app

State the number._

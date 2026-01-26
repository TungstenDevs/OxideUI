# OxideUI Framework

A modern, reactive UI framework for Rust inspired by Flutter and Jetpack Compose, bringing declarative UI development to the Rust ecosystem with professional-grade architecture.

## Overview

OxideUI is a serious UI framework designed from the ground up with Rust's principles in mind. Unlike toy frameworks, OxideUI implements a proven three-tree architecture (Widget/Element/Render) with GPU-accelerated rendering, multi-platform support, and async capabilities that scales from simple applications to complex enterprise systems.

Built with system programming discipline, OxideUI delivers native performance while maintaining the developer ergonomics of modern declarative UI frameworks.

(was experimenting with bunch of things I will rewrite it for nightly release)

## Architecture Highlights

- **Three-Tree System**: Widget tree (declarative), Element tree (runtime state), Render tree (GPU primitives)
- **GPU-First Rendering**: Skia-based rendering with OpenGL/Metal acceleration and intelligent CPU fallbacks
- **Retained Mode**: Efficient scene management and partial updates instead of costly full redraws
- **Async-Native**: Tokio-powered async runtime with non-blocking UI thread and safe state management
- **Declarative Yet Rusty**: No OOP inheritance hierarchies - pure Rust idioms with traits and composition
- **Cross-Platform**: winit-powered windowing supporting Wayland, X11, Windows, and macOS
- **Layout Engine**: Constraint-based layout system supporting flexbox, grids, and absolute positioning

## Features

- **Professional Architecture**: Production-ready design with separation of concerns
- **Adaptive Rendering**: Automatic backend selection (Skia GL → Skia CPU → Softbuffer)
- **Reactive State**: Fine-grained state management with minimal redraws
- **Component-Based**: Build reusable, composable UI components
- **Async Integration**: Safe async/await patterns that never block the UI thread
- **Input Handling**: Comprehensive pointer, keyboard, and focus management
- **Theming System**: Dynamic theming and responsive design capabilities
- **Performance Focused**: Work-stealing scheduler and damage regions for minimal redraws



## Design Philosophy

OxideUI follows core principles that make it suitable for production applications:

1. **Correctness First**: The three-tree model ensures state consistency and proper lifecycle management
2. **Performance by Design**: Retained mode rendering with partial updates minimizes GPU work
3. **Rust-Native Patterns**: No forced OOP hierarchies; composition over inheritance
4. **Practical Fallbacks**: Graceful degradation through multiple rendering backends
5. **Async-Safe**: UI thread never blocked by async operations
6. **Extensible Architecture**: Clear separation between platform, core, and application layers

## Contributing

Contributions are welcome! This project follows a disciplined architecture and development process:

- Report issues and suggest features with specific use cases
- Submit pull requests with comprehensive tests
- Improve documentation with real-world examples
- Optimize performance with benchmarks and profiling data
- Expand platform support with proper abstraction layers

## Maintainer

**xOphiuchus** - [GitHub](https://github.com/xOphiuchus)

## License

Apache-2.0

## Roadmap

- [x] Multi-backend renderer system (Skia GL/CPU, Softbuffer)
- [x] Windowing and platform integration (winit)
- [x] Core framework architecture and runtime
- [ ] Complete widget library implementation
- [ ] Advanced layout engine (flex, grid, constraints)
- [ ] State management system
- [ ] Event handling and routing
- [ ] Async integration hooks
- [ ] Text shaping and typography system
- [ ] Accessibility support
- [ ] Mobile platform extensions
- [ ] Hot reload development workflow
- [ ] Comprehensive documentation and examples

## Inspiration

- [Flutter](https://flutter.dev) - Google's UI framework
- [Jetpack Compose](https://developer.android.com/jetpack/compose) - Android's modern toolkit
- Rust ecosystem best practices

---

### Will be available soon!!

OxideUI represents a serious attempt to bring modern UI development paradigms to Rust while respecting the language's systems programming nature. The framework is actively developed with a focus on correctness, performance, and developer experience. Join us in building the future of Rust UI development!
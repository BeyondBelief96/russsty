# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build the project (requires SDL2 library path on macOS)
LIBRARY_PATH="/opt/homebrew/opt/sdl2/lib:$LIBRARY_PATH" cargo build

# Run the project
LIBRARY_PATH="/opt/homebrew/opt/sdl2/lib:$LIBRARY_PATH" cargo run

# Run tests
cargo test

# Check for compilation errors without building
cargo check
```

## Dependencies

- **SDL2**: Install via `brew install sdl2` on macOS. The `sdl2` Rust crate (v0.38.0) provides bindings.

## Architecture

This is a software-rendered graphics engine using SDL2 for window management and display.

### Core Components

- **Engine struct**: Manages a CPU-side color buffer (`Vec<u32>` in ARGB8888 format) and provides primitive drawing operations (pixels, grid, rectangles).
- **Main loop**: Standard game loop pattern with input processing → update → render phases.
- **Rendering pipeline**: The engine draws to its internal buffer, which is then uploaded to an SDL texture and copied to the canvas each frame.

### Key Design Decisions

- All rendering is CPU-based via direct pixel manipulation in the color buffer
- Window is resizable; resizing recreates the engine and texture at the new dimensions
- Pixel format is ARGB8888 (4 bytes per pixel)

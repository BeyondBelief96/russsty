# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

### macOS

```bash
# Build the project (requires SDL2 library path on macOS)
LIBRARY_PATH="/opt/homebrew/opt/sdl2/lib:$LIBRARY_PATH" cargo build

# Run the project
LIBRARY_PATH="/opt/homebrew/opt/sdl2/lib:$LIBRARY_PATH" cargo run

# Run tests
cargo test

# Run a single test
cargo test test_name

# Check for compilation errors without building
cargo check
```

### Windows

1. Download SDL2 development libraries from https://github.com/libsdl-org/SDL/releases
   - Get the `SDL2-devel-X.XX.X-VC.zip` file (for MSVC) or `SDL2-devel-X.XX.X-mingw.zip` (for GNU toolchain)

2. For MSVC toolchain:
   - Extract the zip file
   - Copy `SDL2.lib` from `lib\x64\` to your Rust lib directory (e.g., `C:\Users\<username>\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\`)
   - Copy `SDL2.dll` from `lib\x64\` to the project root (or add it to your PATH)

3. For GNU toolchain (MinGW):
   - Extract the zip file
   - Copy all `.a` files from `lib\x64\` to your Rust lib directory
   - Copy `SDL2.dll` to the project root

4. Build and run:
   ```cmd
   cargo build
   cargo run
   ```

**Alternative (bundled):** Add `features = ["bundled"]` to the sdl2 dependency in `Cargo.toml`. This compiles SDL2 from source but requires CMake installed.

## Dependencies

- **SDL2**:
  - macOS: Install via `brew install sdl2`
  - Windows: Download from https://github.com/libsdl-org/SDL/releases (see build instructions above)
  - The `sdl2` Rust crate (v0.38.0) provides bindings.
- **tobj**: OBJ file loader for mesh import.
- **approx**: Floating-point comparison utilities.

## Architecture

This is a CPU-based software-rendered 3D graphics engine using SDL2 only for window management and display.

### Rendering Pipeline

1. **Mesh Loading** (`mesh.rs`): Loads OBJ files via `tobj` or uses built-in cube mesh. Faces use 1-based vertex indices.

2. **Transform & Projection** (`engine.rs:update()`):
   - Model → World: Applies rotation transforms (X, Y, Z axes)
   - World → View: Translates by camera position
   - Backface culling via cross product normal and dot product with camera ray
   - Perspective projection using FOV factor division

3. **Rasterization** (`rasterizer/scanline.rs`): Scanline algorithm using flat-top/flat-bottom triangle decomposition. Fills triangles into FrameBuffer.

4. **Display** (`window.rs`): FrameBuffer bytes are uploaded to an SDL streaming texture (ARGB8888) and copied to canvas.

### Module Visibility

- **Public API** (`lib.rs`): `engine`, `math`, `window` modules
- **Internal** (`pub(crate)`): `framebuffer`, `mesh`, `rasterizer`, `renderer`

### Key Types

- **Engine**: Main facade coordinating rendering. Holds Renderer, Rasterizer, Mesh, camera state.
- **Renderer**: Owns the color buffer (`Vec<u32>`), provides primitive drawing (pixels, lines, rectangles, grid).
- **FrameBuffer**: Borrowed view into Renderer's buffer for rasterization with bounds-checked pixel access.
- **Window**: SDL2 wrapper handling events, texture management, and frame presentation.

### Render Modes (keys 1-5)

Controlled via `RenderMode` enum: Wireframe, WireframeVertices, FilledWireframe (default), FilledWireframeVertices, Filled.

### Line Drawing

Uses Bresenham's algorithm (`renderer.rs:draw_line_bresenham`). DDA algorithm also available but unused.

//! Color constants used throughout the renderer.
//!
//! All colors are in ARGB8888 format (0xAARRGGBB).

/// Background color for the frame buffer (dark gray).
pub const BACKGROUND: u32 = 0xFF1E1E1E;

/// Grid line color (medium gray).
pub const GRID: u32 = 0xFF333333;

/// Default triangle fill color (gray).
pub const FILL: u32 = 0xFF444444;

/// Wireframe edge color (green).
pub const WIREFRAME: u32 = 0xFF00FF00;

/// Vertex marker color (red).
pub const VERTEX: u32 = 0xFFFF0000;

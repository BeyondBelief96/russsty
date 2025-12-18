//! Color constants used throughout the renderer.
//!
//! All colors are in ARGB8888 format (0xAARRGGBB).

/// Background color for the frame buffer (dark gray).
pub const BACKGROUND: u32 = 0xFF1E1E1E;

/// Grid line color (medium gray).
pub const GRID: u32 = 0xFF333333;

/// Default triangle fill color (gray).
pub const FILL: u32 = 0xFF888888;

/// Wireframe edge color (green).
pub const WIREFRAME: u32 = 0xFF00FF00;

/// Vertex marker color (red).
pub const VERTEX: u32 = 0xFFFF0000;

/// Modulate a color by an intensity factor (0.0 to 1.0).
///
/// Preserves the alpha channel while scaling the RGB channels.
/// Useful for applying lighting intensity to a base color.
pub fn modulate(color: u32, intensity: f32) -> u32 {
    let a = (color >> 24) & 0xFF;
    let r = ((((color >> 16) & 0xFF) as f32 * intensity) as u32).min(255);
    let g = ((((color >> 8) & 0xFF) as f32 * intensity) as u32).min(255);
    let b = (((color & 0xFF) as f32 * intensity) as u32).min(255);
    (a << 24) | (r << 16) | (g << 8) | b
}

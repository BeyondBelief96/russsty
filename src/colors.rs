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

/// Unpack an ARGB8888 color into its constituent RGB components constrained to the range [0.0, 1.0].
///
/// Returns a tuple of floats representing the red, green, and blue components.
pub fn unpack_color(color: u32) -> (f32, f32, f32) {
    let r = ((color >> 16) & 0xFF) as f32 / 255.0;
    let g = ((color >> 8) & 0xFF) as f32 / 255.0;
    let b = (color & 0xFF) as f32 / 255.0;
    (r, g, b)
}

/// Pack RGB components and an alpha value into an ARGB8888 color.
/// Assumes the input r,g,b values are in the range [0.0, 1.0].
pub fn pack_color(r: f32, g: f32, b: f32, a: f32) -> u32 {
    let a = (a * 255.0).round() as u32;
    let r = (r * 255.0).round() as u32;
    let g = (g * 255.0).round() as u32;
    let b = (b * 255.0).round() as u32;
    (a << 24) | (r << 16) | (g << 8) | b
}

/// Linearly interpolates between two RGB colors.
///
/// # Formula
///
/// ```text
/// result = c1 + (c2 - c1) * t = c1 * (1 - t) + c2 * t
/// ```
///
/// # Arguments
///
/// * `c1` - Start color (when t = 0)
/// * `c2` - End color (when t = 1)
/// * `t` - Interpolation factor in range [0, 1]
///
/// # Returns
///
/// Interpolated RGB color as (r, g, b) tuple with components in [0, 1]
#[inline]
pub fn lerp_color(c1: (f32, f32, f32), c2: (f32, f32, f32), t: f32) -> (f32, f32, f32) {
    (
        c1.0 + (c2.0 - c1.0) * t,
        c1.1 + (c2.1 - c1.1) * t,
        c1.2 + (c2.2 - c1.2) * t,
    )
}

//! Scanline-based triangle rasterization.

use super::{Rasterizer, Triangle};
use crate::colors::{pack_color, unpack_color};
use crate::math::vec3::Vec3;
use crate::render::framebuffer::FrameBuffer;
use crate::ShadingMode;

/// Scanline-based triangle rasterizer.
///
/// Uses the flat-top/flat-bottom triangle decomposition approach:
/// 1. Sort vertices by Y coordinate
/// 2. Split triangle into flat-top and/or flat-bottom triangles
/// 3. Rasterize each scanline from left to right
pub struct ScanlineRasterizer;

impl ScanlineRasterizer {
    pub fn new() -> Self {
        Self
    }

    /// Sort vertices by Y coordinate.
    fn sort_vertices(v0: &mut Vec3, v1: &mut Vec3, v2: &mut Vec3) {
        if v1.y < v0.y {
            std::mem::swap(v0, v1);
        }
        if v2.y < v1.y {
            std::mem::swap(v1, v2);
        }
        if v1.y < v0.y {
            std::mem::swap(v0, v1);
        }
    }

    /// Sort vertices by Y coordinate, also reordering colors to match.
    fn sort_vertices_with_colors(
        v0: &mut Vec3,
        v1: &mut Vec3,
        v2: &mut Vec3,
        c0: &mut (f32, f32, f32),
        c1: &mut (f32, f32, f32),
        c2: &mut (f32, f32, f32),
    ) {
        if v1.y < v0.y {
            std::mem::swap(v0, v1);
            std::mem::swap(c0, c1);
        }
        if v2.y < v1.y {
            std::mem::swap(v1, v2);
            std::mem::swap(c1, c2);
        }
        if v1.y < v0.y {
            std::mem::swap(v0, v1);
            std::mem::swap(c0, c1);
        }
    }

    /// Linearly interpolate between two colors.
    #[inline]
    fn lerp_color(c1: (f32, f32, f32), c2: (f32, f32, f32), t: f32) -> (f32, f32, f32) {
        (
            c1.0 + (c2.0 - c1.0) * t,
            c1.1 + (c2.1 - c1.1) * t,
            c1.2 + (c2.2 - c1.2) * t,
        )
    }

    /// Fill a flat-bottom triangle with Gouraud shading.
    /// v0 is the top vertex, v1 and v2 are at the bottom (same y).
    fn fill_flat_bottom_gouraud(
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        c0: (f32, f32, f32),
        c1: (f32, f32, f32),
        c2: (f32, f32, f32),
        buffer: &mut FrameBuffer,
    ) {
        let height = v1.y - v0.y;
        if height.abs() < f32::EPSILON {
            return;
        }

        let inv_slope_1 = (v1.x - v0.x) / height;
        let inv_slope_2 = (v2.x - v0.x) / height;

        let y_start = v0.y.ceil() as i32;
        let y_end = v1.y.floor() as i32;

        for y in y_start..=y_end {
            let dy = y as f32 - v0.y;
            let t = dy / height;

            let x1 = v0.x + inv_slope_1 * dy;
            let x2 = v0.x + inv_slope_2 * dy;

            // Interpolate colors along edges
            let color1 = Self::lerp_color(c0, c1, t);
            let color2 = Self::lerp_color(c0, c2, t);

            // Determine left/right
            let (x_left, x_right, c_left, c_right) = if x1 < x2 {
                (x1, x2, color1, color2)
            } else {
                (x2, x1, color2, color1)
            };

            let x_start = x_left.ceil() as i32;
            let x_end = x_right.floor() as i32;
            let span = x_right - x_left;

            for x in x_start..=x_end {
                let tx = if span.abs() < f32::EPSILON {
                    0.0
                } else {
                    (x as f32 - x_left) / span
                };
                let color = Self::lerp_color(c_left, c_right, tx);
                buffer.set_pixel(x, y, pack_color(color.0, color.1, color.2, 1.0));
            }
        }
    }

    /// Fill a flat-top triangle with Gouraud shading.
    /// v0 and v1 are at the top (same y), v2 is at the bottom.
    fn fill_flat_top_gouraud(
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        c0: (f32, f32, f32),
        c1: (f32, f32, f32),
        c2: (f32, f32, f32),
        buffer: &mut FrameBuffer,
    ) {
        let height = v2.y - v0.y;
        if height.abs() < f32::EPSILON {
            return;
        }

        let inv_slope_1 = (v2.x - v0.x) / height;
        let inv_slope_2 = (v2.x - v1.x) / height;

        let y_start = v0.y.ceil() as i32;
        let y_end = v2.y.floor() as i32;

        for y in y_start..=y_end {
            let dy = y as f32 - v0.y;
            let t = dy / height;

            let x1 = v0.x + inv_slope_1 * dy;
            let x2 = v1.x + inv_slope_2 * dy;

            // Interpolate colors along edges
            let color1 = Self::lerp_color(c0, c2, t);
            let color2 = Self::lerp_color(c1, c2, t);

            // Determine left/right
            let (x_left, x_right, c_left, c_right) = if x1 < x2 {
                (x1, x2, color1, color2)
            } else {
                (x2, x1, color2, color1)
            };

            let x_start = x_left.ceil() as i32;
            let x_end = x_right.floor() as i32;
            let span = x_right - x_left;

            for x in x_start..=x_end {
                let tx = if span.abs() < f32::EPSILON {
                    0.0
                } else {
                    (x as f32 - x_left) / span
                };
                let color = Self::lerp_color(c_left, c_right, tx);
                buffer.set_pixel(x, y, pack_color(color.0, color.1, color.2, 1.0));
            }
        }
    }

    /// Fill a flat-bottom triangle with a solid color.
    fn fill_flat_bottom_solid(
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        buffer: &mut FrameBuffer,
        color: u32,
    ) {
        let height = v1.y - v0.y;
        if height.abs() < f32::EPSILON {
            return;
        }

        let inv_slope_1 = (v1.x - v0.x) / height;
        let inv_slope_2 = (v2.x - v0.x) / height;

        let y_start = v0.y.ceil() as i32;
        let y_end = v1.y.floor() as i32;

        for y in y_start..=y_end {
            let dy = y as f32 - v0.y;
            let x1 = v0.x + inv_slope_1 * dy;
            let x2 = v0.x + inv_slope_2 * dy;
            let x_left = x1.min(x2).ceil() as i32;
            let x_right = x1.max(x2).floor() as i32;
            buffer.fill_scanline(y, x_left, x_right, color);
        }
    }

    /// Fill a flat-top triangle with a solid color.
    fn fill_flat_top_solid(
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        buffer: &mut FrameBuffer,
        color: u32,
    ) {
        let height = v2.y - v0.y;
        if height.abs() < f32::EPSILON {
            return;
        }

        let inv_slope_1 = (v2.x - v0.x) / height;
        let inv_slope_2 = (v2.x - v1.x) / height;

        let y_start = v0.y.ceil() as i32;
        let y_end = v2.y.floor() as i32;

        for y in y_start..=y_end {
            let dy = y as f32 - v0.y;
            let x1 = v0.x + inv_slope_1 * dy;
            let x2 = v1.x + inv_slope_2 * dy;
            let x_left = x1.min(x2).ceil() as i32;
            let x_right = x1.max(x2).floor() as i32;
            buffer.fill_scanline(y, x_left, x_right, color);
        }
    }
}

impl Default for ScanlineRasterizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Rasterizer for ScanlineRasterizer {
    fn fill_triangle(&self, triangle: &Triangle, buffer: &mut FrameBuffer, color: u32) {
        let mut v0 = triangle.points[0];
        let mut v1 = triangle.points[1];
        let mut v2 = triangle.points[2];

        match triangle.shading_mode {
            ShadingMode::Gouraud => {
                // Unpack and sort vertices with their colors
                let mut c0 = unpack_color(triangle.vertex_colors[0]);
                let mut c1 = unpack_color(triangle.vertex_colors[1]);
                let mut c2 = unpack_color(triangle.vertex_colors[2]);

                Self::sort_vertices_with_colors(&mut v0, &mut v1, &mut v2, &mut c0, &mut c1, &mut c2);

                // Check if triangle is flat-bottom (bottom two vertices have same y)
                if (v1.y - v2.y).abs() < f32::EPSILON {
                    Self::fill_flat_bottom_gouraud(v0, v1, v2, c0, c1, c2, buffer);
                    return;
                }

                // Check if triangle is flat-top (top two vertices have same y)
                if (v0.y - v1.y).abs() < f32::EPSILON {
                    Self::fill_flat_top_gouraud(v0, v1, v2, c0, c1, c2, buffer);
                    return;
                }

                // General case: split into flat-bottom and flat-top triangles
                // Calculate split point and interpolated color
                let t = (v1.y - v0.y) / (v2.y - v0.y);
                let split_x = v0.x + (v2.x - v0.x) * t;
                let split_point = Vec3::new(split_x, v1.y, 0.0);
                let split_color = Self::lerp_color(c0, c2, t);

                // Fill flat-bottom triangle (top half): v0 at top, v1 and split at bottom
                Self::fill_flat_bottom_gouraud(v0, v1, split_point, c0, c1, split_color, buffer);
                // Fill flat-top triangle (bottom half): v1 and split at top, v2 at bottom
                Self::fill_flat_top_gouraud(v1, split_point, v2, c1, split_color, c2, buffer);
            }
            ShadingMode::Flat | ShadingMode::None => {
                Self::sort_vertices(&mut v0, &mut v1, &mut v2);

                // Check if triangle is flat-bottom
                if (v1.y - v2.y).abs() < f32::EPSILON {
                    Self::fill_flat_bottom_solid(v0, v1, v2, buffer, color);
                    return;
                }

                // Check if triangle is flat-top
                if (v0.y - v1.y).abs() < f32::EPSILON {
                    Self::fill_flat_top_solid(v0, v1, v2, buffer, color);
                    return;
                }

                // General case: split into flat-bottom and flat-top triangles
                let t = (v1.y - v0.y) / (v2.y - v0.y);
                let split_x = v0.x + (v2.x - v0.x) * t;
                let split_point = Vec3::new(split_x, v1.y, 0.0);

                Self::fill_flat_bottom_solid(v0, v1, split_point, buffer, color);
                Self::fill_flat_top_solid(v1, split_point, v2, buffer, color);
            }
        }
    }
}

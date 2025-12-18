//! Edge function-based triangle rasterization.
//!
//! This module implements triangle rasterization using the edge function algorithm,
//! which is the foundation of modern GPU rasterization. The algorithm tests each
//! pixel against three edge equations to determine triangle coverage.
//!
//! # Algorithm Overview
//!
//! The edge function algorithm works by:
//! 1. Computing a bounding box around the triangle
//! 2. For each pixel in the bounding box, evaluating three edge functions
//! 3. A pixel is inside the triangle if all edge functions have the same sign
//!
//! # Edge Function
//!
//! For an edge from point A to point B, the edge function at point P is:
//!
//! ```text
//! E(P) = (P.x - A.x) * (B.y - A.y) - (P.y - A.y) * (B.x - A.x)
//! ```
//!
//! This is equivalent to the 2D cross product (B - A) × (P - A), which gives:
//! - Positive value: P is to the left of edge AB (counter-clockwise)
//! - Negative value: P is to the right of edge AB (clockwise)
//! - Zero: P is exactly on the edge
//!
//! # Barycentric Coordinates
//!
//! The edge function values are proportional to barycentric coordinates:
//!
//! ```text
//! lambda_i = E_i(P) / (E_0 + E_1 + E_2)
//! ```
//!
//! Where E_i is the edge function for the edge opposite to vertex i.
//! These coordinates are used for attribute interpolation (colors, UVs, etc.).
//!
//! # Winding Order
//!
//! The algorithm handles both clockwise and counter-clockwise triangles by
//! checking the sign of the total signed area. For CW triangles, all edge
//! functions will be negative for interior points; for CCW, all positive.
//!
//! # References
//!
//! - Juan Pineda, "A Parallel Algorithm for Polygon Rasterization" (1988)
//! - Scratchapixel: <https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation>

use super::{Rasterizer, Triangle};
use crate::colors::{pack_color, unpack_color};
use crate::math::vec3::Vec3;
use crate::render::framebuffer::FrameBuffer;
use crate::ShadingMode;

/// Triangle rasterizer using the edge function algorithm.
///
/// This rasterizer iterates over all pixels in the triangle's bounding box
/// and uses edge functions to determine which pixels are inside the triangle.
/// It supports both flat shading (single color) and Gouraud shading (per-vertex
/// color interpolation using barycentric coordinates).
///
/// # Characteristics
///
/// - **Simplicity**: Easy to understand and implement
/// - **Parallelizable**: Each pixel can be evaluated independently (GPU-friendly)
/// - **Accurate**: Handles all triangle orientations and edge cases
/// - **Flexible**: Natural support for attribute interpolation via barycentric coords
///
/// # Performance Considerations
///
/// The bounding box approach means we test many pixels outside the triangle,
/// especially for thin/elongated triangles. More sophisticated implementations
/// use hierarchical testing or tile-based approaches to reduce wasted work.
pub struct EdgeFunctionRasterizer;

impl EdgeFunctionRasterizer {
    /// Creates a new edge function rasterizer instance.
    pub fn new() -> Self {
        EdgeFunctionRasterizer {}
    }

    /// Computes the edge function value for point P relative to edge (A -> B).
    ///
    /// The edge function is the signed area of the parallelogram formed by
    /// vectors (B - A) and (P - A), computed as their 2D cross product:
    ///
    /// ```text
    /// E(P) = (P.x - A.x) * (B.y - A.y) - (P.y - A.y) * (B.x - A.x)
    /// ```
    ///
    /// # Returns
    ///
    /// - Positive: P is to the left of edge AB (counter-clockwise winding)
    /// - Negative: P is to the right of edge AB (clockwise winding)
    /// - Zero: P lies exactly on the edge AB
    ///
    /// # Arguments
    ///
    /// * `a` - Start point of the edge
    /// * `b` - End point of the edge
    /// * `p` - Point to test against the edge
    #[inline]
    fn edge_function(a: Vec3, b: Vec3, p: Vec3) -> f32 {
        (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
    }
}

impl Default for EdgeFunctionRasterizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Rasterizer for EdgeFunctionRasterizer {
    /// Fills a triangle using the edge function algorithm.
    ///
    /// # Algorithm Steps
    ///
    /// 1. **Bounding Box**: Compute axis-aligned bounding box of the triangle,
    ///    clamped to framebuffer bounds to avoid out-of-bounds access.
    ///
    /// 2. **Signed Area**: Calculate the signed area (2x) of the triangle using
    ///    the edge function. This determines winding order and provides the
    ///    denominator for barycentric coordinate normalization.
    ///
    /// 3. **Pixel Iteration**: For each pixel center (x + 0.5, y + 0.5) in the
    ///    bounding box:
    ///    - Compute edge functions w0, w1, w2 for edges opposite to v0, v1, v2
    ///    - Test if pixel is inside by checking all edge functions have same sign
    ///
    /// 4. **Shading**: Based on shading mode:
    ///    - Flat/None: Write solid color directly
    ///    - Gouraud: Compute barycentric coords and interpolate vertex colors
    ///
    /// # Arguments
    ///
    /// * `triangle` - The triangle to rasterize, containing vertices and colors
    /// * `buffer` - The framebuffer to write pixels to
    /// * `color` - The flat color to use (for Flat/None shading modes)
    fn fill_triangle(&self, triangle: &Triangle, buffer: &mut FrameBuffer, color: u32) {
        let v0 = triangle.points[0];
        let v1 = triangle.points[1];
        let v2 = triangle.points[2];

        // ─────────────────────────────────────────────────────────────────────
        // Step 1: Compute bounding box
        // ─────────────────────────────────────────────────────────────────────
        // Find the axis-aligned bounding box containing the triangle.
        // We use floor/ceil to ensure we cover all pixels that might be touched.
        let min_x = v0.x.min(v1.x).min(v2.x).floor() as i32;
        let max_x = v0.x.max(v1.x).max(v2.x).ceil() as i32;
        let min_y = v0.y.min(v1.y).min(v2.y).floor() as i32;
        let max_y = v0.y.max(v1.y).max(v2.y).ceil() as i32;

        // Clamp to framebuffer bounds to prevent out-of-bounds writes
        let min_x = min_x.max(0);
        let max_x = max_x.min(buffer.width() as i32 - 1);
        let min_y = min_y.max(0);
        let max_y = max_y.min(buffer.height() as i32 - 1);

        // ─────────────────────────────────────────────────────────────────────
        // Step 2: Compute signed area (determines winding order)
        // ─────────────────────────────────────────────────────────────────────
        // The edge function of v2 against edge (v0 -> v1) gives twice the
        // signed area of the triangle:
        // - Positive area = counter-clockwise winding
        // - Negative area = clockwise winding
        let area = Self::edge_function(v0, v1, v2);

        // Skip degenerate triangles (collinear vertices = zero area)
        if area.abs() < f32::EPSILON {
            return;
        }

        // ─────────────────────────────────────────────────────────────────────
        // Step 3 & 4: Iterate pixels and shade
        // ─────────────────────────────────────────────────────────────────────
        match triangle.shading_mode {
            ShadingMode::Gouraud => {
                // Precompute inverse area for barycentric normalization
                let inv_area = 1.0 / area;

                // Unpack vertex colors to floating-point for interpolation
                let colors: [(f32, f32, f32); 3] = [
                    unpack_color(triangle.vertex_colors[0]),
                    unpack_color(triangle.vertex_colors[1]),
                    unpack_color(triangle.vertex_colors[2]),
                ];

                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        // Sample at pixel center (add 0.5 to integer coords)
                        let p = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

                        // Compute edge functions for each edge:
                        // w0 = edge opposite to v0 (edge v1->v2)
                        // w1 = edge opposite to v1 (edge v2->v0)
                        // w2 = edge opposite to v2 (edge v0->v1)
                        let w0 = Self::edge_function(v1, v2, p);
                        let w1 = Self::edge_function(v2, v0, p);
                        let w2 = Self::edge_function(v0, v1, p);

                        // Point is inside if all edge functions have same sign as area.
                        // This handles both CW (all negative) and CCW (all positive) winding.
                        let inside = if area > 0.0 {
                            w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                        } else {
                            w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                        };

                        if inside {
                            // Compute barycentric coordinates by normalizing edge functions.
                            // lambda_i represents the "weight" of vertex i at this point.
                            // These sum to 1.0 for any point in the triangle.
                            let lambda0 = w0 * inv_area;
                            let lambda1 = w1 * inv_area;
                            let lambda2 = w2 * inv_area;

                            // Interpolate RGB components using barycentric weights
                            let r = lambda0 * colors[0].0
                                + lambda1 * colors[1].0
                                + lambda2 * colors[2].0;
                            let g = lambda0 * colors[0].1
                                + lambda1 * colors[1].1
                                + lambda2 * colors[2].1;
                            let b = lambda0 * colors[0].2
                                + lambda1 * colors[1].2
                                + lambda2 * colors[2].2;

                            buffer.set_pixel(x, y, pack_color(r, g, b, 1.0));
                        }
                    }
                }
            }
            ShadingMode::Flat | ShadingMode::None => {
                // Flat shading: single color for entire triangle (no interpolation)
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        let p = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

                        let w0 = Self::edge_function(v1, v2, p);
                        let w1 = Self::edge_function(v2, v0, p);
                        let w2 = Self::edge_function(v0, v1, p);

                        let inside = if area > 0.0 {
                            w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                        } else {
                            w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                        };

                        if inside {
                            buffer.set_pixel(x, y, color);
                        }
                    }
                }
            }
        }
    }
}

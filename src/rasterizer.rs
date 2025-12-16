mod scanline;

pub(crate) use scanline::ScanlineRasterizer;

use crate::framebuffer::FrameBuffer;
use crate::math::vec2::Vec2;

/// A triangle ready for rasterization in screen space.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Triangle {
    pub points: [Vec2; 3],
    pub color: u32,
}

impl Triangle {
    pub fn new(points: [Vec2; 3], color: u32) -> Self {
        Self { points, color }
    }
}

/// Trait for triangle rasterization algorithms.
///
/// Implementors define how triangles are filled into a pixel buffer.
/// This allows swapping between different rasterization strategies
/// (scanline, edge functions, etc.) for testing and benchmarking.
pub(crate) trait Rasterizer {
    /// Fill a triangle into the frame buffer.
    ///
    /// # Arguments
    /// * `triangle` - The triangle to rasterize
    /// * `buffer` - The frame buffer to draw into
    /// * `color` - The color to fill the triangle with
    fn fill_triangle(&self, triangle: &Triangle, buffer: &mut FrameBuffer, color: u32);
}

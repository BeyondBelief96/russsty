//! Utility functions for math operations.

use crate::prelude::Vec2;

/// Compute the edge function value for point p relative to edge (a -> b).
///
/// Returns a positive value if p is to the left of the edge (counter-clockwise),
/// negative if to the right (clockwise), and zero if exactly on the edge.
/// This is the 2D cross product: (b - a) × (p - a)
#[inline]
pub fn edge_function(a: Vec2, b: Vec2, p: Vec2) -> f32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}

/// Compute barycentric coordinates for point p within triangle (v0, v1, v2).
///
/// Returns (lambda0, lambda1, lambda2) where each lambda is the weight
/// for the corresponding vertex. For points inside the triangle,
/// all values are in [0, 1] and sum to 1.
#[inline]
pub fn barycentric_coordinates(p: Vec2, v0: Vec2, v1: Vec2, v2: Vec2) -> (f32, f32, f32) {
    let w0 = edge_function(v1, v2, p);
    let w1 = edge_function(v2, v0, p);
    let w2 = edge_function(v0, v1, p);

    let area = w0 + w1 + w2;
    (w0 / area, w1 / area, w2 / area)
}

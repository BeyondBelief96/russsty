//! Utility functions for math operations.

use crate::prelude::Vec2;

pub enum BarycentricCoordinate {
    /// Barycentric coordinate for vertex 0.
    Lambda0(f32),
    /// Barycentric coordinate for vertex 1.
    Lambda1(f32),
    /// Barycentric coordinate for vertex 2.
    Lambda2(f32),
}

/// Compute the edge function value for point p relative to edge (a -> b).
///
/// Returns a positive value if p is to the left of the edge (counter-clockwise),
/// negative if to the right (clockwise), and zero if exactly on the edge.
/// This is the 2D cross product: (b - a) × (p - a)
#[inline]
fn edge_function(a: Vec2, b: Vec2, p: Vec2) -> f32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}

pub fn barycentric_coordinates(
    p: Vec2,
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
) -> (
    BarycentricCoordinate,
    BarycentricCoordinate,
    BarycentricCoordinate,
) {
    let a = edge_function(v1, v2, p);
    let b = edge_function(v2, v0, p);
    let c = edge_function(v0, v1, p);

    let total = a + b + c;
    (
        BarycentricCoordinate::Lambda0(a / total),
        BarycentricCoordinate::Lambda1(b / total),
        BarycentricCoordinate::Lambda2(c / total),
    )
}

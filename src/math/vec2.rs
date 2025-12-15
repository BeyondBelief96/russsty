use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };
    pub const UP: Self = Self { x: 0.0, y: 1.0 };
    pub const DOWN: Self = Self { x: 0.0, y: -1.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn rotate(&self, angle: f32) -> Self {
        Self {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn subtract(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn scale(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the area of the parallelogram formed by the two vectors.
    /// In 2D, this is equivalent to the magnitude of the cross product.
    pub fn cross(&self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

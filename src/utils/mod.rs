pub mod vector;
pub mod color;
pub mod image;

pub const EPSILON: f32 = 0.001;

#[inline]
pub fn approximately(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a) * t + a
}

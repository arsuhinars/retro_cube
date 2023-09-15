pub mod vector;
pub mod color;
pub mod image;

#[inline]
pub fn approximately(a: f32, b: f32) -> bool {
    (a - b).abs() < f32::EPSILON
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a) * t + a
}

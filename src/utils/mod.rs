use self::vector::Vector3;

pub mod vector;
pub mod matrix;
pub mod transform;
pub mod color;

const EPSILON: f32 = 0.001;

#[inline]
pub fn approximately(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a) * t + a
}

#[inline]
pub fn plane_cast(
    plane_normal: &Vector3, plane_d: f32, ray_origin: &Vector3, ray_direction: &Vector3
) -> Option<Vector3> {
    let t = (
        plane_d - Vector3::dot(plane_normal, ray_origin)
    ) / Vector3::dot(plane_normal, ray_direction);

    return if t.is_nan() || t < 0.0 {
        None
    } else {
        Some(*ray_origin + *ray_direction * t)
    };
}

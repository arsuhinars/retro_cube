use crate::utils::approximately;
use crate::utils::vector::{Vector3, ONE_VECTOR, FORWARD_VECTOR, UP_VECTOR, RIGHT_VECTOR};
use crate::utils::matrix::{
    translate_matrix,
    look_at_rotate_matrix,
    inverse_look_at_rotate_matrix,
    euler_rotation_matrix,
    inverse_euler_rotation_matrix,
    scale_matrix
};

#[test]
fn test_translate_matrix() {
    let v = translate_matrix(
        &Vector3::new(0.5, 3.0, -2.5)
    ) * ONE_VECTOR;
    assert!(v.approximately(&Vector3::new(
        1.5, 4.0, -1.5
    )));
}

#[test]
fn test_look_at_rotate() {
    let dir = Vector3::new(-0.5, 1.5, 1.0);

    let forward = look_at_rotate_matrix(&dir) * FORWARD_VECTOR;
    let up = look_at_rotate_matrix(&dir) * UP_VECTOR;
    let right = look_at_rotate_matrix(&dir) * RIGHT_VECTOR;

    assert!(approximately(
        Vector3::angle(&forward, &dir), 0.0
    ));

    assert!(approximately(
        Vector3::angle(&up, &dir), (90.0_f32).to_radians()
    ));

    assert!(approximately(
        Vector3::angle(&right, &dir), (90.0_f32).to_radians()
    ));

    assert!(approximately(
        Vector3::angle(&forward, &up), (90.0_f32).to_radians()
    ));

    assert!(approximately(
        Vector3::angle(&forward, &right), (90.0_f32).to_radians()
    ));

    assert!(approximately(
        Vector3::angle(&right, &up), (90.0_f32).to_radians()
    ));
}

#[test]
fn test_inverse_look_at_rotate() {
    let dir = Vector3::new(5.0, 4.0, -3.0);
    let v = inverse_look_at_rotate_matrix(&dir) * dir.normalized();
    // let v = Vector3::inverse_look_at_rotate(&dir.normalized(), &dir);

    assert!(FORWARD_VECTOR.approximately(&v));
}

#[test]
fn test_euler_rotate() {
    let v = euler_rotation_matrix(
        &Vector3::new((-90.0_f32).to_radians(), 0.0, 0.0)
    ) * FORWARD_VECTOR;
    assert!(UP_VECTOR.approximately(&v));

    let v = euler_rotation_matrix(
        &Vector3::new(0.0, (90.0_f32).to_radians(), 0.0)
    ) * FORWARD_VECTOR;
    assert!(RIGHT_VECTOR.approximately(&v));

    let v = euler_rotation_matrix(
        &Vector3::new(0.0, 0.0, (90.0_f32).to_radians())
    ) * RIGHT_VECTOR;
    assert!(UP_VECTOR.approximately(&v));
}

#[test]
fn test_inverse_euler_rotate() {
    let v = inverse_euler_rotation_matrix(
        &Vector3::new((-90.0_f32).to_radians(), 0.0, 0.0)
    ) * UP_VECTOR;
    assert!(FORWARD_VECTOR.approximately(&v));

    let v = inverse_euler_rotation_matrix(
        &Vector3::new(0.0, (90.0_f32).to_radians(), 0.0)
    ) * RIGHT_VECTOR;
    assert!(FORWARD_VECTOR.approximately(&v));

    let v = inverse_euler_rotation_matrix(
        &Vector3::new(0.0, 0.0, (90.0_f32).to_radians())
    ) * UP_VECTOR;
    assert!(RIGHT_VECTOR.approximately(&v));
}

#[test]
fn test_scale_matrix() {
    let v = scale_matrix(&Vector3::new(
        -1.0, 2.0, 0.0
    )) * ONE_VECTOR;
    assert!(v.approximately(&Vector3::new(
        -1.0, 2.0, 0.0
    )));
}

use crate::utils::approximately;
use crate::utils::vector::{Vector3, FORWARD_VECTOR, UP_VECTOR, RIGHT_VECTOR};

#[test]
fn test_look_at_rotate() {
    let dir = Vector3::new(-0.5, 1.5, 1.0);

    let forward = Vector3::look_at_rotate(&FORWARD_VECTOR, &dir);
    let up = Vector3::look_at_rotate(&UP_VECTOR, &dir);
    let right = Vector3::look_at_rotate(&RIGHT_VECTOR, &dir);

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
    let v = Vector3::inverse_look_at_rotate(&dir.normalized(), &dir);

    assert!(FORWARD_VECTOR.approximately(&v));
}

#[test]
fn test_euler_rotate() {
    let v = Vector3::euler_rotate(
        &FORWARD_VECTOR, &Vector3::new((90.0_f32).to_radians(), 0.0, 0.0)
    );
    assert!(UP_VECTOR.approximately(&v));

    let v = Vector3::euler_rotate(
        &FORWARD_VECTOR, &Vector3::new(0.0, (-90.0_f32).to_radians(), 0.0)
    );
    assert!(RIGHT_VECTOR.approximately(&v));

    let v = Vector3::euler_rotate(
        &RIGHT_VECTOR, &Vector3::new(0.0, 0.0, (-90.0_f32).to_radians())
    );
    assert!(UP_VECTOR.approximately(&v));
}

#[test]
fn test_inverse_euler_rotate() {
    let v = Vector3::inverse_euler_rotate(
        &UP_VECTOR, &Vector3::new((90.0_f32).to_radians(), 0.0, 0.0)
    );
    assert!(FORWARD_VECTOR.approximately(&v));

    let v = Vector3::inverse_euler_rotate(
        &RIGHT_VECTOR, &Vector3::new(0.0, (-90.0_f32).to_radians(), 0.0)
    );
    assert!(FORWARD_VECTOR.approximately(&v));

    let v = Vector3::inverse_euler_rotate(
        &UP_VECTOR, &Vector3::new(0.0, 0.0, (-90.0_f32).to_radians())
    );
    assert!(RIGHT_VECTOR.approximately(&v));
}

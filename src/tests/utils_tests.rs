use crate::utils::{approximately, lerp, plane_cast};
use crate::utils::vector::Vector3;

#[test]
fn test_approximately_true() {
    assert!(approximately(200.0, 200.0));
    assert!(approximately(-300.0, -300.0));
    assert!(approximately(0.0, 0.0));
    assert!(approximately(1.0, 1.00000001));
}

#[test]
fn test_approximately_false() {
    assert_eq!(approximately(0.0, 1.0), false);
    assert_eq!(approximately(-100.0, 100.0), false);
    assert_eq!(approximately(90.0, 10.0), false);
    assert_eq!(approximately(1.0, 1.1), false);
}

#[test]
fn test_lerp() {
    assert!(approximately(lerp(-10.0, 15.0, 0.0), -10.0));
    assert!(approximately(lerp(-25.0, 30.0, 1.0), 30.0));
    assert!(approximately(lerp(-100.0, 100.0, 0.5), 0.0));
    assert!(approximately(lerp(10.0, 20.0, 0.5), 15.0));
}

#[test]
fn test_plane_cast_non_none() {
    let n = Vector3::new(-1.0, 1.0, 0.5);
    let d = 1.5;

    let origin = Vector3::new(0.0, 1.0, -1.0);
    let direction = Vector3::new(-1.0, 0.0, 1.0);

    let result = plane_cast(&n, d, &origin, &direction);

    match result {
        Some(v) => {
            assert!(approximately(Vector3::dot(&n, &v) - d, 0.0));
        }
        None => {
            assert!(false);
        }
    }
}

#[test]
fn test_plane_cast_none() {
    let n = Vector3::new(1.0, 1.0, -1.0);
    let d = 0.5;

    let origin = Vector3::new(2.0, 3.0, -4.0);
    let direction = Vector3::new(1.0, 0.0, 1.0);

    let result = plane_cast(&n, d, &origin, &direction);

    match result {
        Some(_) => {
            assert!(false);
        },
        _ => {}
    }
}

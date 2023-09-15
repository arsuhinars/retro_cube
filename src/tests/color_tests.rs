use crate::utils::color::Color;

#[test]
fn test_from_pixel_data() {
    assert_eq!(Color::from_pixel_data(0xAABBCCDD), Color::new_with_alpha(170, 187, 204, 221));
    assert_eq!(Color::from_pixel_data(0x10203040), Color::new_with_alpha(16, 32, 48, 64 ));
    assert_eq!(Color::from_pixel_data(0xFFFFFFFF), Color::new_with_alpha(255, 255, 255, 255));
    assert_eq!(Color::from_pixel_data(0x00000000), Color::new_with_alpha(0, 0, 0, 0));
}

#[test]
fn test_to_pixel_data() {
    assert_eq!(Color::new_with_alpha(255, 0, 0, 255).to_pixel_data(), 0xFF0000FF);
    assert_eq!(Color::new_with_alpha(127, 96, 48, 50).to_pixel_data(), 0x7F603032);
    assert_eq!(Color::new_with_alpha(0, 128, 255, 255).to_pixel_data(), 0x0080FFFF);
    assert_eq!(Color::new_with_alpha(50, 50, 50, 50).to_pixel_data(), 0x32323232);
}

#[test]
fn test_color_lerp() {
    let a = Color::new(128, 64, 32);
    let b = Color::new(0, 255, 48);

    assert_eq!(Color::lerp(&a, &b, 0.0), a);
    assert_eq!(Color::lerp(&a, &b, 1.0), b);
    assert_eq!(Color::lerp(&a, &b, 0.5), Color::new(64, 159, 40));
}

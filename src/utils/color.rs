use super::lerp;

pub type PixelData = u32;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn tint(&self, tint: &Color) -> Color {
        Color {
            r: ((self.r as u32) * (tint.r as u32) / 255) as u8,
            g: ((self.g as u32) * (tint.g as u32) / 255) as u8,
            b: ((self.b as u32) * (tint.b as u32) / 255) as u8,
            a: ((self.a as u32) * (tint.a as u32) / 255) as u8,
        }
    }

    pub fn lerp(a: &Color, b: &Color, t: f32) -> Color {
        Color {
            r: lerp(a.r as f32, b.r as f32, t) as u8,
            g: lerp(a.g as f32, b.g as f32, t) as u8,
            b: lerp(a.b as f32, b.b as f32, t) as u8,
            a: lerp(a.a as f32, b.a as f32, t) as u8
        }
    }
}

impl From<PixelData> for Color {
    fn from(value: PixelData) -> Self {
        Color {
            r: (value >> 24) as u8,
            g: ((value >> 16) & 0xFF) as u8,
            b: ((value >> 8) & 0xFF) as u8,
            a: (value & 0xFF) as u8
        }
    }
}

impl From<Color> for PixelData {
    fn from(value: Color) -> Self {
        ((value.r as u32) << 24) |
        ((value.g as u32) << 16) |
        ((value.b as u32) << 8) |
        (value.a as u32)
    }
}

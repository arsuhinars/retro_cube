use crate::utils::{color::Color, vector::Vector3};

pub trait Lightning {
    fn apply_light(&self, base_color: Color, position: &Vector3, normal: &Vector3) -> Color;
}

pub struct UnlitLightning {
    ambient_color: Color
}

impl UnlitLightning {
    pub fn new(ambient_color: &Color) -> UnlitLightning {
        UnlitLightning {
            ambient_color: *ambient_color
        }
    }

    pub fn get_ambient_color(&self) -> &Color {
        &self.ambient_color
    }

    pub fn set_ambient_color(&mut self, color: &Color) {
        self.ambient_color = *color;
    }
}

impl Lightning for UnlitLightning {
    fn apply_light(&self, base_color: Color, _position: &Vector3, _normal: &Vector3) -> Color {
        base_color.tint(&self.ambient_color)
    }
}

pub struct DiffuseDirectLightning {
    direction: Vector3,
    normalized_dir: Vector3,
    color: Color,
    ambient_color: Color,
}

impl DiffuseDirectLightning {
    pub fn new(direction: &Vector3, color: &Color, ambient_color: &Color) -> DiffuseDirectLightning {
        DiffuseDirectLightning {
            direction: *direction,
            normalized_dir: -direction.normalized(),
            color: *color,
            ambient_color: *ambient_color
        }
    }

    pub fn get_direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn set_direction(&mut self, direction: &Vector3) {
        self.direction = *direction;
        self.normalized_dir = -direction.normalized();
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: &Color) {
        self.color = *color;
    }

    pub fn get_ambient_color(&self) -> &Color {
        &self.ambient_color
    }

    pub fn set_ambient_color(&mut self, color: &Color) {
        self.ambient_color = *color;
    }
}

impl Lightning for DiffuseDirectLightning {
    fn apply_light(&self, base_color: Color, _position: &Vector3, normal: &Vector3) -> Color {
        let k = Vector3::dot(&self.normalized_dir, normal).clamp(0.0, 1.0);

        let light_color = Color::lerp(
            &self.ambient_color,
            &self.color,
            k
        );

        base_color.tint(&light_color)
    }
}

// pub struct SpecularDirectLightning {
//     pub direction: Vector3,
//     pub color: Color,
//     pub ambient_color: Color,
//     pub specular_factor: f32
// }

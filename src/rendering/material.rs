use crate::utils::{vector::Vector3, color::Color};

pub trait Material {
    fn compute_surface_color(&self, position: &Vector3, normal: &Vector3) -> Color;
}

pub struct FlatMaterial {
    pub color: Color,
}

impl Material for FlatMaterial {
    fn compute_surface_color(&self, _position: &Vector3, _normal: &Vector3) -> Color {
        self.color
    }
}

pub struct CheckerMaterial {
    pub scale: f32,
    pub first_color: Color,
    pub second_color: Color
}

impl CheckerMaterial {
    fn sample_checker_texture(&self, uv_x: f32, uv_y: f32) -> f32 {
        let x = ((uv_x / self.scale).rem_euclid(1.0) / 0.5) as u32;
        let y = ((uv_y / self.scale).rem_euclid(1.0) / 0.5) as u32;
        return ((x + y) % 2) as f32;
    }

    fn get_triplanar_weights(normal: &Vector3) -> Vector3 {
        let w = normal.abs();
        let s = w.x + w.y + w.z;
        Vector3 {
            x: w.x / s,
            y: w.y / s,
            z: w.z / s
        }
    }
}

impl Material for CheckerMaterial {
    fn compute_surface_color(&self, position: &Vector3, normal: &Vector3) -> Color {
        let albedo_x = self.sample_checker_texture(position.y, position.z);
        let albedo_y = self.sample_checker_texture(position.x, position.z);
        let albedo_z = self.sample_checker_texture(position.x, position.y);

        let weights = CheckerMaterial::get_triplanar_weights(normal);

        Color::lerp(
            &self.first_color,
            &self.second_color,
            albedo_x * weights.x + albedo_y * weights.y + albedo_z * weights.z
        )
    }
}

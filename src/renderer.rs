use crate::raycaster::Raycaster;
use crate::utils::vector::Vector3;
use crate::utils::image::Image;

pub struct Camera {
    position: Vector3,
    direction: Vector3,
    fov: f32,
    aspect_ratio: f32
}

pub struct Renderer {
    screen_image: Image,
    camera: Camera,
    raycaster: Box<dyn Raycaster>
}

impl Renderer {
    fn update(delta_time: f32) {

    }
}

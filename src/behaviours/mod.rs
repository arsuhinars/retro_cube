pub mod camera_behaviour;
pub mod renderer_behaviour;
pub mod raycaster_behaviour;
pub mod material_behaviour;

pub trait Behaviour {
    fn update(&mut self, delta_time: f32);
}

use crate::utils::vector::Vector3;

fn world_ray_to_local(origin: &mut Vector3, dir: &mut Vector3, pos: &Vector3, rotation: &Vector3) {

}

pub struct RaycastHit {
    position: Vector3,
    normal: Vector3
}

pub trait Raycaster {
    fn get_tranform(&self, position: &mut Vector3, rotation: &mut Vector3);

    fn set_transform(&self, position: &Vector3, rotation: &Vector3);

    fn raycast(&self, origin: &Vector3, dir: &Vector3) -> RaycastHit;
}

pub struct BoxRaycaster {
    position: Vector3,
    rotation: Vector3,
    size: Vector3
}

impl Raycaster for BoxRaycaster {
    fn get_tranform(&self, position: &mut Vector3, rotation: &mut Vector3) {
        todo!()
    }

    fn set_transform(&self, position: &Vector3, rotation: &Vector3) {
        todo!()
    }

    fn raycast(&self, origin: &Vector3, dir: &Vector3) -> RaycastHit {
        todo!()
    }
}

pub struct SphereRaycaster {
    position: Vector3,
    rotation: Vector3,
    radius: f32
}

impl Raycaster for SphereRaycaster {
    fn get_tranform(&self, position: &mut Vector3, rotation: &mut Vector3) {
        todo!()
    }

    fn set_transform(&self, position: &Vector3, rotation: &Vector3) {
        todo!()
    }

    fn raycast(&self, origin: &Vector3, dir: &Vector3) -> RaycastHit {
        todo!()
    }
}

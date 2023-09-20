use crate::utils::{vector::Vector3, transform::Transform, plane_cast};

pub struct RaycastHit {
    pub position: Vector3,
    pub local_position: Vector3,
    pub local_normal: Vector3
}

pub trait Raycaster {
    fn get_mut_tranform(&mut self) -> &mut Transform;

    fn raycast(&mut self, origin: &Vector3, direction: &Vector3) -> Option<RaycastHit>;
}

pub struct BoxRaycaster {
    transform: Transform,
    half_size: Vector3
}

impl BoxRaycaster {
    pub fn new(size: &Vector3) -> BoxRaycaster {
        BoxRaycaster { transform: Transform::default(), half_size: *size * 0.5 }
    }
}

impl Raycaster for BoxRaycaster {
    fn get_mut_tranform(&mut self) -> &mut Transform { &mut self.transform }

    fn raycast(&mut self, origin: &Vector3, direction: &Vector3) -> Option<RaycastHit> {
        let mut o = self.transform.inverse_transform_position(origin);
        let mut d = self.transform.inverse_transform_direction(direction);

        let i = (o.x < 0.0, o.y < 0.0, o.z < 0.0);

        fn invert_vector(v: &Vector3, mask: (bool, bool, bool)) -> Vector3 {
            let mut a = *v;

            if mask.0 {
                a.x *= -1.0;
            }
            if mask.1 {
                a.y *= -1.0;
            }
            if mask.2 {
                a.z *= -1.0;
            }

            return a
        }

        o = invert_vector(&o, i);
        d = invert_vector(&d, i);

        let n = Vector3::new(0.0, 0.0, 1.0);
        match plane_cast(&n, self.half_size.z, &o, &d) {
            Some(mut p) => {
                if p.x.abs() < self.half_size.x && p.y.abs() < self.half_size.y {
                    p = invert_vector(&p, i);
                    return Some(RaycastHit {
                        position: self.transform.transform_position(&p),
                        local_position: p,
                        local_normal: n
                    })
                }
            },
            _ => ()
        }

        let n = Vector3::new(0.0, 1.0, 0.0);
        match plane_cast(&n, self.half_size.y, &o, &d) {
            Some(mut p) => {
                if p.x.abs() < self.half_size.x && p.z.abs() < self.half_size.z {
                    p = invert_vector(&p, i);
                    return Some(RaycastHit {
                        position: self.transform.transform_position(&p),
                        local_position: p,
                        local_normal: n
                    })
                }
            },
            _ => ()
        }

        let n = Vector3::new(1.0, 0.0, 0.0);
        match plane_cast(&n, self.half_size.x, &o, &d) {
            Some(mut p) => {
                if p.y.abs() < self.half_size.y && p.z.abs() < self.half_size.z {
                    p = invert_vector(&p, i);
                    return Some(RaycastHit {
                        position: self.transform.transform_position(&p),
                        local_position: p,
                        local_normal: n
                    })
                }
            },
            _ => ()
        }

        return None;
    }
}

pub struct SphereRaycaster {
    transform: Transform,
    radius: f32
}

impl SphereRaycaster {
    pub fn new(radius: f32) -> SphereRaycaster {
        SphereRaycaster { transform: Default::default(), radius }
    }
}

impl Raycaster for SphereRaycaster {
    fn get_mut_tranform(&mut self) -> &mut Transform { &mut self.transform }

    fn raycast(&mut self, origin: &Vector3, direction: &Vector3) -> Option<RaycastHit> {
        let o = self.transform.inverse_transform_position(origin);
        let d = self.transform.inverse_transform_direction(direction).normalized();

        let a = Vector3::dot(&-o, &d);
        let t = a - (self.radius.powi(2) - o.sqr_length() + a.powi(2)).sqrt();

        if t.is_nan() || t < 0.0 {
            return None;
        }

        let p = o + d * t;

        return Some(RaycastHit {
            position: self.transform.transform_position(&p),
            local_position: p,
            local_normal: p.normalized()
        })
    }
}

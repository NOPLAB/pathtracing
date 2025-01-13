use super::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct HitPoint {
    pub distance: f64,
    pub normal: Vec3,
    pub position: Vec3,
}

impl HitPoint {
    pub fn new(distance: f64, normal: Vec3, position: Vec3) -> HitPoint {
        HitPoint {
            distance: distance,
            normal: normal,
            position: position,
        }
    }
}

pub struct Intersection {
    pub hit_point: HitPoint,
    pub object_id: u32,
}

impl Intersection {
    pub fn new(hit_point: HitPoint, object_id: u32) -> Intersection {
        Intersection {
            hit_point,
            object_id,
        }
    }
}

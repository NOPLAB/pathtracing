use super::{intersection::HitPoint, material::RefrectionType, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    radius: f64,
    position: Vec3,
    emission: Vec3,
    color: Vec3,
    reflection_type: RefrectionType,
}

impl Sphere {
    pub fn new(
        radius: f64,
        position: Vec3,
        emission: Vec3,
        color: Vec3,
        reflection_type: RefrectionType,
    ) -> Sphere {
        Sphere {
            radius,
            position,
            emission,
            color,
            reflection_type,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitPoint> {
        let p_o = self.position - ray.origin;
        let b = p_o.dot(ray.direction);
        let d4 = b * b - p_o.dot(p_o) + self.radius * self.radius;

        if d4 < 0.0 {
            return None;
        }

        let sqrt_d4 = d4.sqrt();
        let t1 = b - sqrt_d4;
        let t2 = b + sqrt_d4;

        if t1 < f64::EPSILON && t2 < f64::EPSILON {
            return None;
        }

        let distance = if t1 > f64::EPSILON { t1 } else { t2 };

        let position = ray.origin + ray.direction * distance;

        let normal = (position - self.position).normalize();

        Some(HitPoint::new(distance, normal, position))
    }
}

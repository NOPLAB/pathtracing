use super::{
    intersection::{HitPoint, Intersection},
    material::RefrectionType,
    ray::Ray,
    sphere::Sphere,
    vec3::Vec3,
};

pub struct Scene {
    spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Scene {
        let spheres = [
            Sphere::new(
                1e5,
                Vec3::new(1e5 + 1.0, 40.8, 81.6),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.75, 0.25, 0.25),
                RefrectionType::Diffuse,
            ),
            Sphere::new(
                1e5,
                Vec3::new(-1e5 + 99.0, 40.8, 81.6),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.25, 0.25, 0.75),
                RefrectionType::Diffuse,
            ),
            Sphere::new(
                1e5,
                Vec3::new(50.0, 40.8, 1e5),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.75, 0.75, 0.75),
                RefrectionType::Diffuse,
            ),
            Sphere::new(
                1e5,
                Vec3::new(50.0, 40.8, -1e5 + 250.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                RefrectionType::Diffuse,
            ),
            Sphere::new(
                1e5,
                Vec3::new(50.0, 1e5, 81.6),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.75, 0.75, 0.75),
                RefrectionType::Diffuse,
            ),
            Sphere::new(
                1e5,
                Vec3::new(50.0, -1e5 + 81.6, 81.6),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.75, 0.75, 0.75),
                RefrectionType::Diffuse,
            ),
            Sphere::new(
                20.0,
                Vec3::new(65.0, 20.0, 20.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.25, 0.75, 0.25),
                RefrectionType::Diffuse,
            ),
            Sphere::new(
                16.5,
                Vec3::new(27.0, 16.5, 47.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.99, 0.99, 0.99),
                RefrectionType::Specular,
            ),
            Sphere::new(
                16.5,
                Vec3::new(77.0, 16.5, 78.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.99, 0.99, 0.99),
                RefrectionType::Refraction,
            ),
            Sphere::new(
                15.0,
                Vec3::new(50.0, 90.0, 81.6),
                Vec3::new(36.0, 36.0, 36.0),
                Vec3::new(0.0, 0.0, 0.0),
                RefrectionType::Diffuse,
            ),
        ];

        Scene {
            spheres: spheres.to_vec(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut hit_point: Option<HitPoint> = None;
        let mut object_id: Option<usize> = None;
        let mut distance = f64::MAX;

        for (i, sphere) in self.spheres.iter().enumerate() {
            if let Some(hit) = sphere.intersect(ray) {
                if hit.distance < distance {
                    distance = hit.distance;
                    hit_point = Some(hit);
                    object_id = Some(i);
                }
            }
        }

        if let Some(hit) = hit_point {
            Some(Intersection {
                hit_point: hit,
                object_id: object_id.unwrap() as u32,
            })
        } else {
            None
        }
    }

    pub fn spheres(&self) -> &[Sphere] {
        &self.spheres
    }
}

use std::intrinsics::powf64;

use intersection::Intersection;
use material::Color;
use random::XorShiftRandom;
use ray::Ray;
use scene::Scene;
use vec3::Vec3;

mod intersection;
mod material;
mod ppm;
mod random;
mod ray;
mod scene;
mod sphere;
mod vec3;

const BACKGROUND_COLOR: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const DEPTH: u32 = 5;
const DEPTH_LIMIT: u32 = 64;

pub struct Render {
    scene: Scene,
}

impl Render {
    fn new() -> Render {
        Render {
            scene: Scene::new(),
        }
    }

    fn render(&self, width: u32, height: u32) {
        let mut random = XorShiftRandom::new(0);

        let camera_pos = Vec3::new(50.0, 52.0, 220.0);
        let camera_dir = Vec3::new(0.0, -0.04, -1.0).normalize();
        let camera_up = Vec3::new(0.0, 1.0, 0.0);

        let screen_width = 30.0 * width as f64 / height as f64;
        let screen_height = 30.0;

        let screen_dist = 40.0;

        let screen_x = camera_dir.cross(camera_up).normalize() * screen_width;
        let screen_y = screen_x.cross(camera_dir).normalize() * screen_height;
        let screen_center = camera_pos + camera_dir * screen_dist;

        let mut image = vec![Vec3::new(0.0, 0.0, 0.0); (width * height) as usize];

        for y in 0..height {
            for x in 0..width {
                let mut color = Vec3::new(0.0, 0.0, 0.0);

                for sy in 0..2 {
                    for sx in 0..2 {
                        let mut subpixel_color = Vec3::new(0.0, 0.0, 0.0);

                        for _ in 0..4 {
                            let ray_dir = screen_center
                                + screen_x * ((sx as f64 + 0.25 + 0.5 * random.next()) / 2.0 - 0.5)
                                + screen_y * ((sy as f64 + 0.25 + 0.5 * random.next()) / 2.0 - 0.5)
                                - camera_pos;
                            let ray_dir = ray_dir.normalize();

                            subpixel_color = subpixel_color
                                + self.radiance(&camera_pos, &ray_dir, &mut random, 0);
                        }

                        subpixel_color = subpixel_color * 0.25;
                        color = color + subpixel_color;
                    }
                }

                image[(height - y - 1) as usize * width as usize + x as usize] = color;
            }
        }
    }

    fn radiance(&self, ray: &Ray, rnd: XorShiftRandom, depth: u32) -> Color {
        if let Some(intersection) = self.scene.intersect(ray) {
            let hit_point = intersection.hit_point;
            let object_id = intersection.object_id;

            let sphere = &self.scene.spheres()[object_id as usize];
            let hitpoint = &intersection.hit_point;
            let orienting_normal = if hitpoint.normal.dot(ray.dir) < 0.0 {
                hitpoint.normal
            } else {
                hitpoint.normal * -1.0
            };
            let mut russian_roulette_probability = sphere.color.max();

            if depth > DEPTH_LIMIT {
                russian_roulette_probability *= powf64(0.5, depth as f64 - DEPTH_LIMIT as f64);
            }

            if depth > DEPTH {
                if (rnd.next())
        } else {
            BACKGROUND_COLOR
        }
    }
}

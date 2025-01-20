use std::{
    sync::{Arc, Mutex},
    thread, vec,
};

use material::{Color, IOR};
use random::XorShiftRandom;
use ray::Ray;
use scene::Scene;
use vec3::Vec3;

mod intersection;
mod material;
pub mod ppm;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TaskStatus {
    NotStarted,
    Running,
    Completed,
}

#[derive(Clone, Copy, Debug)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub tasks: u32,
    pub samples: u32,
    pub super_samples: u32,
}

pub struct Render {
    config: RenderConfig,
    scene: Scene,
}

impl Render {
    pub fn new(config: RenderConfig) -> Render {
        Render {
            config,
            scene: Scene::new(),
        }
    }

    pub fn render(&self) -> Vec<Color> {
        let tasks = self.config.tasks;
        let width = self.config.width;
        let height = self.config.height;
        let samples = self.config.samples;
        let super_samples = self.config.super_samples;

        let camera_pos = Vec3::new(50.0, 52.0, 220.0);
        let camera_dir = Vec3::new(0.0, -0.04, -1.0).normalize();
        let camera_up = Vec3::new(0.0, 1.0, 0.0);

        let screen_width = 30.0 * self.config.width as f64 / self.config.height as f64;
        let screen_height = 30.0;

        let screen_dist = 40.0;

        let screen_x = camera_dir.cross(camera_up).normalize() * screen_width;
        let screen_y = screen_x.cross(camera_dir).normalize() * screen_height;
        let screen_center = camera_pos + camera_dir * screen_dist;

        let image = Arc::new(Mutex::new(vec![
            Color::new(0.0, 0.0, 0.0);
            (self.config.width * self.config.height)
                as usize
        ]));

        let tasks_status = Arc::new(Mutex::new(vec![
            TaskStatus::NotStarted;
            self.config.height as usize
        ]));

        let arc_image = image.clone();
        thread::scope(move |s| loop {
            let running_tasks = tasks_status
                .lock()
                .unwrap()
                .iter()
                .filter(|t| **t == TaskStatus::Running)
                .count();

            if running_tasks >= tasks as usize {
                thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }

            if tasks_status
                .lock()
                .unwrap()
                .iter()
                .all(|t| *t == TaskStatus::Completed)
            {
                break;
            }

            let arc_completed_tasks = tasks_status.clone();
            let process_y = {
                let mut process_y = None;

                let mut completed_tasks = arc_completed_tasks.lock().unwrap();

                for (i, completed) in completed_tasks.iter_mut().enumerate() {
                    if *completed == TaskStatus::NotStarted {
                        process_y = Some(i);
                        *completed = TaskStatus::Running;
                        break;
                    }
                }

                process_y
            };

            let arc_tasks_status = tasks_status.clone();
            let arc_arc_image = arc_image.clone();
            if let Some(y) = process_y {
                s.spawn(move || {
                    let y = y as u32;

                    println!("Rendering (y = {} / {})", y, height);

                    let mut rnd = XorShiftRandom::new(y + 1);

                    let mut cache = vec![Color::new(0.0, 0.0, 0.0); width as usize];

                    for x in 0..width {
                        for sy in 0..super_samples {
                            for sx in 0..super_samples {
                                let mut accumulated_radiance = Color::new(0.0, 0.0, 0.0);

                                for _ in 0..samples {
                                    let rate = 1.0 / samples as f64;
                                    let r1 = sx as f64 * rate / 2.0;
                                    let r2 = sy as f64 * rate / 2.0;

                                    let screen_position = screen_center
                                        + screen_x * ((r1 + x as f64) / width as f64 - 0.5)
                                        + screen_y * ((r2 + y as f64) / height as f64 - 0.5);

                                    let dir = (screen_position - camera_pos).normalize();

                                    accumulated_radiance = accumulated_radiance
                                        + self.radiance(
                                            &Ray {
                                                origin: camera_pos,
                                                direction: dir,
                                            },
                                            &mut rnd,
                                            0,
                                        ) / samples as f64
                                            / (super_samples as f64 * super_samples as f64);
                                }
                                cache[x as usize] = cache[x as usize] + accumulated_radiance;
                            }
                        }
                    }

                    {
                        let mut image = arc_arc_image.lock().unwrap();
                        for x in 0..width {
                            image[(y * width + x) as usize] = cache[x as usize];
                        }
                    }

                    {
                        let mut completed_tasks = arc_tasks_status.lock().unwrap();
                        completed_tasks[y as usize] = TaskStatus::Completed;
                    }

                    println!("Completed (y = {} / {})", y, height);
                });
            }
        });

        return image.lock().unwrap().to_vec();
    }

    fn radiance(&self, ray: &Ray, rnd: &mut XorShiftRandom, depth: u32) -> Color {
        if let Some(intersection) = self.scene.intersect(ray) {
            let hit_point = intersection.hit_point;
            let object_id = intersection.object_id;

            let sphere = &self.scene.spheres()[object_id as usize];
            let hitpoint = &intersection.hit_point;
            let orienting_normal = if hitpoint.normal.dot(ray.direction) < 0.0 {
                hitpoint.normal
            } else {
                hitpoint.normal * -1.0
            };
            let mut russian_roulette_probability = sphere.color.max();

            if depth > DEPTH_LIMIT {
                russian_roulette_probability *= 0.5f64.powf(depth as f64 - DEPTH_LIMIT as f64);
            }

            if depth > DEPTH {
                if rnd.next_f64() >= russian_roulette_probability {
                    return sphere.emission;
                }
            } else {
                russian_roulette_probability = 1.0;
            }

            let incoming_radiance;
            let weight;

            match sphere.reflection_type {
                material::RefrectionType::Diffuse => {
                    let w = orienting_normal;
                    let u = if w.x.abs() > f64::EPSILON {
                        Vec3::new(0.0, 1.0, 0.0).cross(w).normalize()
                    } else {
                        Vec3::new(1.0, 0.0, 0.0).cross(w).normalize()
                    };
                    let v = w.cross(u);

                    let r1 = 2.0 * std::f64::consts::PI * rnd.next_f64();
                    let r2 = rnd.next_f64();
                    let r2s = r2.sqrt();
                    let dir = (u * r1.cos() * r2s + v * r1.sin() * r2s + w * (1.0 - r2).sqrt())
                        .normalize();

                    incoming_radiance = self.radiance(
                        &Ray {
                            origin: hitpoint.position,
                            direction: dir,
                        },
                        rnd,
                        depth + 1,
                    );
                    weight = sphere.color / russian_roulette_probability;
                }
                material::RefrectionType::Specular => {
                    incoming_radiance = self.radiance(
                        &Ray {
                            origin: hitpoint.position,
                            direction: ray.direction
                                - hitpoint.normal * 2.0 * hitpoint.normal.dot(ray.direction),
                        },
                        rnd,
                        depth + 1,
                    );
                    weight = sphere.color / russian_roulette_probability;
                }
                material::RefrectionType::Refraction => {
                    let refrection_ray = Ray::new(
                        hit_point.position,
                        ray.direction - hitpoint.normal * 2.0 * hitpoint.normal.dot(ray.direction),
                    );
                    let into = hitpoint.normal.dot(orienting_normal) > 0.0;

                    let nc = 1.0;
                    let nt = IOR;
                    let nnt = if into { nc / nt } else { nt / nc };
                    let ddn = ray.direction.dot(orienting_normal);
                    let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);

                    if cos2t < 0.0 {
                        incoming_radiance = self.radiance(&refrection_ray, rnd, depth + 1);
                        weight = sphere.color / russian_roulette_probability;
                    } else {
                        let refrection_ray = Ray::new(
                            hitpoint.position,
                            (ray.direction * nnt
                                - hitpoint.normal
                                    * (if into { 1.0 } else { -1.0 })
                                    * (ddn * nnt + cos2t.sqrt()))
                            .normalize(),
                        );

                        let a = nt - nc;
                        let b = nt + nc;
                        let r0 = (a * a) / (b * b);

                        let c = 1.0
                            - (if into {
                                -ddn
                            } else {
                                refrection_ray.direction.dot(-1.0 * orienting_normal)
                            });
                        let re = r0 + (1.0 - r0) * c.powi(5);
                        let nnt2 = (if into { nt / nc } else { nc / nt }).powi(2);
                        let tr = (1.0 - re) * nnt2;

                        let probability = 0.25 + 0.5 * re;
                        if depth > 2 {
                            if rnd.next_f64() < probability {
                                incoming_radiance =
                                    self.radiance(&refrection_ray, rnd, depth + 1) * re;
                                weight =
                                    sphere.color / (probability * russian_roulette_probability);
                            } else {
                                incoming_radiance =
                                    self.radiance(&refrection_ray, rnd, depth + 1) * tr;
                                weight = sphere.color
                                    / ((1.0 - probability) * russian_roulette_probability);
                            }
                        } else {
                            incoming_radiance =
                                self.radiance(&refrection_ray, rnd, depth + 1) * (re + tr);
                            weight = sphere.color / russian_roulette_probability;
                        }
                    }
                }
            }

            return sphere.emission + incoming_radiance * weight;
        } else {
            BACKGROUND_COLOR
        }
    }
}

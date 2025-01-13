use super::vec3::Vec3;

pub type Color = Vec3;

#[derive(Debug, Clone, Copy)]
pub enum RefrectionType {
    Diffuse,
    Specular,
    Refraction,
}

pub const IOR: f64 = 1.5;

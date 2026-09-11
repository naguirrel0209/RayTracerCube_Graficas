use crate::{material::Material, ray::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub hit: bool,
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Intersection {
    pub fn miss() -> Self {
        Self {
            hit: false,
            distance: f32::INFINITY,
            point: Vec3::ZERO,
            normal: Vec3::ZERO,
            material: Material::new(crate::color::Color::BLACK),
        }
    }

    pub fn new(distance: f32, point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            hit: true,
            distance,
            point,
            normal,
            material,
        }
    }
}

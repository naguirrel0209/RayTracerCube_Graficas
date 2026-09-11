use crate::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub albedo: Color,
    pub k_a: f32,
    pub k_d: f32,
    pub k_s: f32,
    pub shininess: f32,
}

impl Material {
    pub const fn new(albedo: Color) -> Self {
        Self::phong(albedo, 0.16, 0.82, 0.22, 24.0)
    }

    pub const fn phong(albedo: Color, k_a: f32, k_d: f32, k_s: f32, shininess: f32) -> Self {
        Self {
            albedo,
            k_a,
            k_d,
            k_s,
            shininess,
        }
    }
}

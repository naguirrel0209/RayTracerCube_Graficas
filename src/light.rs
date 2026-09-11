use crate::{color::Color, ray::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Light {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
    pub attenuation: f32,
}

impl Light {
    pub const fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
            attenuation: 0.025,
        }
    }
}

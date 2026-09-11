use crate::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub diffuse: Color,
}

impl Material {
    pub const fn new(diffuse: Color) -> Self {
        Self { diffuse }
    }
}

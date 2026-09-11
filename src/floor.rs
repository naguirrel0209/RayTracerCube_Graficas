use crate::{material::Material, ray::Vec3, triangle::Triangle};

pub fn build_floor(size: f32, y: f32, material: Material) -> Vec<Triangle> {
    let h = size * 0.5;

    let back_left = Vec3::new(-h, y, -h);
    let front_left = Vec3::new(-h, y, h);
    let front_right = Vec3::new(h, y, h);
    let back_right = Vec3::new(h, y, -h);

    vec![
        Triangle::new(back_left, front_left, front_right, material),
        Triangle::new(back_left, front_right, back_right, material),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn floor_has_two_triangles() {
        let material = Material::new(Color::new(0.7, 0.7, 0.7));

        assert_eq!(build_floor(8.0, -1.0, material).len(), 2);
    }
}

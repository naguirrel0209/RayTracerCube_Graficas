use crate::{material::Material, ray::Vec3, triangle::Triangle};

pub fn build_cube(size: f32, material: Material) -> Vec<Triangle> {
    let h = size * 0.5;

    let p000 = Vec3::new(-h, -h, -h);
    let p001 = Vec3::new(-h, -h, h);
    let p010 = Vec3::new(-h, h, -h);
    let p011 = Vec3::new(-h, h, h);
    let p100 = Vec3::new(h, -h, -h);
    let p101 = Vec3::new(h, -h, h);
    let p110 = Vec3::new(h, h, -h);
    let p111 = Vec3::new(h, h, h);

    vec![
        Triangle::new(p001, p101, p111, material),
        Triangle::new(p001, p111, p011, material),
        Triangle::new(p100, p000, p010, material),
        Triangle::new(p100, p010, p110, material),
        Triangle::new(p000, p001, p011, material),
        Triangle::new(p000, p011, p010, material),
        Triangle::new(p101, p100, p110, material),
        Triangle::new(p101, p110, p111, material),
        Triangle::new(p010, p011, p111, material),
        Triangle::new(p010, p111, p110, material),
        Triangle::new(p000, p100, p101, material),
        Triangle::new(p000, p101, p001, material),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn cube_has_twelve_triangles() {
        let material = Material::new(Color::new(0.3, 0.7, 1.0));

        assert_eq!(build_cube(2.0, material).len(), 12);
    }
}

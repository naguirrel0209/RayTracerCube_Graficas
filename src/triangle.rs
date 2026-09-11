use crate::{
    intersect::Intersection,
    material::Material,
    ray::{Ray, Vec3},
};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub material: Material,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: Material) -> Self {
        Self { a, b, c, material }
    }

    pub fn normal(self) -> Vec3 {
        (self.b - self.a).cross(self.c - self.a).normalize()
    }

    pub fn intersect(self, ray: Ray) -> Option<Intersection> {
        const EPSILON: f32 = 0.000_001;

        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = ray.direction.cross(edge2);
        let determinant = edge1.dot(h);

        if determinant.abs() < EPSILON {
            return None;
        }

        let inverse_determinant = 1.0 / determinant;
        let s = ray.origin - self.a;
        let u = inverse_determinant * s.dot(h);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(edge1);
        let v = inverse_determinant * ray.direction.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Moller-Trumbore returns the ray distance directly in parameter t.
        let distance = inverse_determinant * edge2.dot(q);
        if distance <= EPSILON {
            return None;
        }

        Some(Intersection::new(
            distance,
            ray.at(distance),
            self.normal(),
            self.material,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn intersects_triangle_with_moller_trumbore() {
        let material = Material::new(Color::new(1.0, 0.0, 0.0));
        let triangle = Triangle::new(
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            material,
        );
        let ray = Ray::new(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0));

        let hit = triangle.intersect(ray).expect("ray should hit triangle");

        assert!(hit.hit);
        assert!((hit.distance - 2.0).abs() < 0.0001);
    }
}

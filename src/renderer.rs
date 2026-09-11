use crate::{
    color::Color, framebuffer::Framebuffer, intersect::Intersection, light::Light, ray::Ray,
    triangle::Triangle,
};

pub struct Renderer {
    background: Color,
    ambient: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{material::Material, ray::Vec3, triangle::Triangle};

    #[test]
    fn cast_ray_keeps_nearest_intersection() {
        let near = Material::new(Color::new(1.0, 0.0, 0.0));
        let far = Material::new(Color::new(0.0, 0.0, 1.0));
        let triangles = vec![
            Triangle::new(
                Vec3::new(-1.0, -1.0, 3.0),
                Vec3::new(1.0, -1.0, 3.0),
                Vec3::new(0.0, 1.0, 3.0),
                far,
            ),
            Triangle::new(
                Vec3::new(-1.0, -1.0, 2.0),
                Vec3::new(1.0, -1.0, 2.0),
                Vec3::new(0.0, 1.0, 2.0),
                near,
            ),
        ];
        let renderer = Renderer::new(Color::BACKGROUND);
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
        let light = Light::new(Vec3::new(0.0, 0.0, 0.0), 1.0);

        let color = renderer.cast_ray(ray, &triangles, light);

        assert!(color.r > color.b);
    }
}

impl Renderer {
    pub fn new(background: Color) -> Self {
        Self {
            background,
            ambient: 0.14,
        }
    }

    pub fn render<F>(
        &self,
        framebuffer: &mut Framebuffer,
        triangles: &[Triangle],
        ray_at: F,
        light: Light,
    ) where
        F: Fn(usize, usize) -> Ray,
    {
        framebuffer.clear(self.background);

        for y in 0..framebuffer.height() {
            for x in 0..framebuffer.width() {
                let ray = ray_at(x, y);
                let color = self.cast_ray(ray, triangles, light);
                framebuffer.set_pixel(x, y, color);
            }
        }
    }

    pub fn cast_ray(&self, ray: Ray, triangles: &[Triangle], light: Light) -> Color {
        let mut z_buffer = f32::INFINITY;
        let mut closest = Intersection::miss();

        for triangle in triangles {
            if let Some(hit) = triangle.intersect(ray) {
                if hit.distance < z_buffer {
                    z_buffer = hit.distance;
                    closest = hit;
                }
            }
        }

        if !closest.hit {
            return self.background;
        }

        let light_direction = (light.position - closest.point).normalize();
        let diffuse = closest.normal.dot(light_direction).max(0.0) * light.intensity;
        let intensity = (self.ambient + diffuse).clamp(0.0, 1.0);

        closest.material.diffuse.scale(intensity)
    }
}

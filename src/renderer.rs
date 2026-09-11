use crate::{
    color::Color, framebuffer::Framebuffer, intersect::Intersection, light::Light, ray::Ray,
    triangle::Triangle,
};

pub struct Renderer {
    background: Color,
    ambient_light: Color,
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
        let light = Light::new(Vec3::new(0.0, 0.0, 0.0), Color::WHITE, 1.0);

        let color = renderer.cast_ray(ray, &triangles, light);

        assert!(color.r > color.b);
    }

    #[test]
    fn phong_lighting_combines_albedo_and_light_color() {
        let material = Material::phong(Color::new(1.0, 0.2, 0.2), 0.0, 1.0, 0.0, 8.0);
        let triangle = Triangle::new(
            Vec3::new(-1.0, -1.0, 2.0),
            Vec3::new(0.0, 1.0, 2.0),
            Vec3::new(1.0, -1.0, 2.0),
            material,
        );
        let renderer = Renderer::new(Color::BACKGROUND);
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
        let light = Light::new(Vec3::new(0.0, 0.0, 0.0), Color::new(0.2, 0.2, 1.0), 1.0);

        let color = renderer.cast_ray(ray, &[triangle], light);

        assert!(color.b > color.g);
    }
}

impl Renderer {
    pub fn new(background: Color) -> Self {
        Self {
            background,
            ambient_light: Color::new(0.16, 0.16, 0.18),
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
        let mut z_buffer = vec![f32::INFINITY; framebuffer.width() * framebuffer.height()];

        for y in 0..framebuffer.height() {
            for x in 0..framebuffer.width() {
                let ray = ray_at(x, y);
                let closest = self.closest_intersection(ray, triangles);
                let index = y * framebuffer.width() + x;

                if closest.hit && closest.distance < z_buffer[index] {
                    z_buffer[index] = closest.distance;
                    framebuffer.set_pixel(x, y, self.shade(ray, closest, light));
                }
            }
        }
    }

    #[cfg(test)]
    fn cast_ray(&self, ray: Ray, triangles: &[Triangle], light: Light) -> Color {
        let closest = self.closest_intersection(ray, triangles);

        if closest.hit {
            self.shade(ray, closest, light)
        } else {
            self.background
        }
    }

    fn closest_intersection(&self, ray: Ray, triangles: &[Triangle]) -> Intersection {
        let mut closest_distance = f32::INFINITY;
        let mut closest = Intersection::miss();

        for triangle in triangles {
            if let Some(hit) = triangle.intersect(ray) {
                if hit.distance < closest_distance {
                    closest_distance = hit.distance;
                    closest = hit;
                }
            }
        }

        closest
    }

    fn shade(&self, ray: Ray, closest: Intersection, light: Light) -> Color {
        let light_vector = light.position - closest.point;
        let light_distance = light_vector.length();
        let light_direction = light_vector.normalize();
        let view_direction = (-ray.direction).normalize();
        let reflection_direction = (-light_direction).reflect(closest.normal).normalize();

        let attenuation = 1.0 / (1.0 + light.attenuation * light_distance * light_distance);
        let light_energy = light.color.scale(light.intensity * attenuation);

        let ambient = closest
            .material
            .albedo
            .multiply(self.ambient_light)
            .scale(closest.material.k_a);
        let diffuse = closest
            .material
            .albedo
            .multiply(light_energy)
            .scale(closest.normal.dot(light_direction).max(0.0) * closest.material.k_d);
        let specular = light_energy.scale(
            reflection_direction
                .dot(view_direction)
                .max(0.0)
                .powf(closest.material.shininess)
                * closest.material.k_s,
        );

        ambient.add(diffuse).add(specular)
    }
}

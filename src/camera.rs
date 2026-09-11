use crate::ray::{Ray, Vec3};

pub struct Camera {
    target: Vec3,
    radius: f32,
    theta: f32,
    phi: f32,
    fov_y: f32,
}

impl Camera {
    pub fn new(target: Vec3, radius: f32, theta: f32, phi: f32, fov_y_degrees: f32) -> Self {
        Self {
            target,
            radius,
            theta,
            phi,
            fov_y: fov_y_degrees.to_radians(),
        }
    }

    pub fn orbit(&mut self, delta_theta: f32, delta_phi: f32) {
        const VERTICAL_LIMIT: f32 = 1.35;

        self.theta += delta_theta;
        self.phi = (self.phi + delta_phi).clamp(-VERTICAL_LIMIT, VERTICAL_LIMIT);
    }

    pub fn position(&self) -> Vec3 {
        let horizontal_radius = self.radius * self.phi.cos();

        self.target
            + Vec3::new(
                horizontal_radius * self.theta.sin(),
                self.radius * self.phi.sin(),
                horizontal_radius * self.theta.cos(),
            )
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let position = self.position();
        let forward = (self.target - position).normalize();
        let right = forward.cross(Vec3::UP).normalize();
        let up = right.cross(forward).normalize();

        let aspect_ratio = width as f32 / height as f32;
        let half_height = (self.fov_y * 0.5).tan();
        let half_width = aspect_ratio * half_height;
        let ndc_x = ((x as f32 + 0.5) / width as f32) * 2.0 - 1.0;
        let ndc_y = 1.0 - ((y as f32 + 0.5) / height as f32) * 2.0;
        let direction =
            (forward + right * (ndc_x * half_width) + up * (ndc_y * half_height)).normalize();

        Ray::new(position, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_orbit_is_clamped() {
        let mut camera = Camera::new(Vec3::ZERO, 4.0, 0.0, 0.0, 60.0);

        camera.orbit(0.0, 10.0);

        assert!(camera.position().y < 4.0);
    }
}

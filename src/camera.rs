use crate::ray::{Ray, Vec3};

const FULL_TURN: f32 = std::f32::consts::TAU;
const MIN_ORBIT_RADIUS: f32 = 2.5;
const MAX_ORBIT_RADIUS: f32 = 12.0;
const PITCH_LIMIT: f32 = 1.4;

pub struct Camera {
    center: Vec3,
    up_reference: Vec3,
    orbit_radius: f32,
    yaw: f32,
    pitch: f32,
    eye: Vec3,
    fov_y: f32,
}

impl Camera {
    pub fn new(orbit_radius: f32, yaw: f32, pitch: f32, fov_y_degrees: f32) -> Self {
        let mut camera = Self {
            center: Vec3::ZERO,
            up_reference: Vec3::UP,
            orbit_radius: orbit_radius.clamp(MIN_ORBIT_RADIUS, MAX_ORBIT_RADIUS),
            yaw,
            pitch: pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT),
            eye: Vec3::ZERO,
            fov_y: fov_y_degrees.to_radians(),
        };
        camera.update_position();
        camera
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw = (self.yaw + delta_yaw).rem_euclid(FULL_TURN);
        self.pitch = (self.pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    }

    pub fn update_position(&mut self) {
        let camera_position = Vec3::new(
            self.orbit_radius * self.pitch.cos() * self.yaw.cos(),
            self.orbit_radius * self.pitch.sin(),
            self.orbit_radius * self.pitch.cos() * self.yaw.sin(),
        );
        self.eye = self.center + camera_position;
    }

    pub fn position(&self) -> Vec3 {
        self.eye
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let eye = self.position();
        let forward = (self.center - eye).normalize();
        let right = forward.cross(self.up_reference).normalize();
        let up = right.cross(forward).normalize();

        let aspect_ratio = width as f32 / height as f32;
        let half_height = (self.fov_y * 0.5).tan();
        let half_width = aspect_ratio * half_height;
        let ndc_x = ((x as f32 + 0.5) / width as f32) * 2.0 - 1.0;
        let ndc_y = 1.0 - ((y as f32 + 0.5) / height as f32) * 2.0;
        let direction =
            (forward + right * (ndc_x * half_width) + up * (ndc_y * half_height)).normalize();

        Ray::new(eye, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_orbit_is_clamped() {
        let mut camera = Camera::new(4.0, 0.0, 0.0, 60.0);

        camera.orbit(0.0, 10.0);
        camera.update_position();

        assert!(camera.position().y < 4.0);
    }

    #[test]
    fn camera_position_uses_spherical_orbit() {
        let camera = Camera::new(5.0, 0.0, 0.0, 60.0);

        assert!((camera.position().x - 5.0).abs() < 0.0001);
        assert!(camera.position().y.abs() < 0.0001);
        assert!(camera.position().z.abs() < 0.0001);
    }

    #[test]
    fn orbit_radius_is_clamped() {
        let min_camera = Camera::new(1.0, 0.0, 0.0, 60.0);
        let max_camera = Camera::new(50.0, 0.0, 0.0, 60.0);

        assert!((min_camera.position().x - MIN_ORBIT_RADIUS).abs() < 0.0001);
        assert!((max_camera.position().x - MAX_ORBIT_RADIUS).abs() < 0.0001);
    }
}

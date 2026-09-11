mod camera;
mod color;
mod cube;
mod floor;
mod framebuffer;
mod intersect;
mod light;
mod material;
mod ray;
mod renderer;
mod triangle;

use camera::Camera;
use color::Color;
use cube::build_cube;
use floor::build_floor;
use framebuffer::Framebuffer;
use light::Light;
use material::Material;
use minifb::{Key, Window, WindowOptions};
use ray::Vec3;
use renderer::Renderer;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const ORBIT_SPEED: f32 = 0.14;

fn main() -> Result<(), minifb::Error> {
    let mut window = Window::new(
        "Raytracer Cube - arrow keys orbit the camera",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;
    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let cube_material = Material::phong(Color::new(0.25, 0.62, 0.95), 0.18, 0.78, 0.28, 28.0);
    let floor_material = Material::phong(Color::new(0.42, 0.43, 0.39), 0.22, 0.74, 0.08, 12.0);
    let mut triangles = build_cube(2.0, cube_material);
    triangles.extend(build_floor(9.0, -1.02, floor_material));
    let light = Light::new(Vec3::new(-3.0, 4.0, 2.5), Color::WHITE, 1.35);
    let renderer = Renderer::new(Color::BACKGROUND);
    let mut camera = Camera::new(5.0, 0.65, 0.32, 55.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Left) {
            camera.orbit(-ORBIT_SPEED, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(ORBIT_SPEED, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, ORBIT_SPEED);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, -ORBIT_SPEED);
        }
        camera.update_position();

        renderer.render(
            &mut framebuffer,
            &triangles,
            |x, y| camera.ray_for_pixel(x, y, WIDTH, HEIGHT),
            light,
        );

        window.update_with_buffer(framebuffer.buffer(), WIDTH, HEIGHT)?;
    }

    Ok(())
}

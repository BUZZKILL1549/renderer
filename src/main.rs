mod framebuffer;
use framebuffer::*;

use minifb::{Key, MouseMode, Window, WindowOptions};

fn main() {
    const WIDTH: usize = 1200;
    const HEIGHT: usize = 800;
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut window = Window::new("Renderer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear(0x000000);

        // Draw a triangle (vertices as f64 for precise calculations)
        let triangle = [(200.0, 150.0), (400.0, 450.0), (600.0, 150.0)];
        framebuffer.draw_triangle(
            triangle[0],
            triangle[1],
            triangle[2],
            Colors::RED.as_u32() + Colors::GREEN.as_u32(),
        );

        // Get mouse position (as f64 for compatibility)
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Pass) {
            let mouse_x = mouse_x as f64;
            let mouse_y = mouse_y as f64;

            for angle in (0..360).step_by(5) {
                let radian = (angle as f64).to_radians();
                let r_dx = radian.cos();
                let r_dy = radian.sin();

                // Extend the ray far enough to intersect the edges of the viewport
                let end_x = mouse_x + r_dx * 1200.0;
                let end_y = mouse_y + r_dy * 800.0;

                // Check for intersections with the triangle
                if let Some((ix, iy)) =
                    framebuffer.intersect_ray_with_polygon(mouse_x, mouse_y, r_dx, r_dy, &triangle)
                {
                    println!("Intersection: {} {}", ix, iy);

                    // Draw a small dot at the intersection
                    for dot_dx in -2..=2 {
                        for dot_dy in -2..=2 {
                            let pixel_x = (ix + dot_dx as f64).round() as isize;
                            let pixel_y = (iy + dot_dy as f64).round() as isize;

                            framebuffer.set_pixel(pixel_x, pixel_y, Colors::RED.as_u32());
                        }
                    }
                }
            }
        }

        // Update the window with the framebuffer
        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

mod framebuffer;
use framebuffer::*;

use minifb::{Key, MouseMode, Window, WindowOptions};

fn main() {
    const WIDTH: usize = 1200;
    const HEIGHT: usize = 800;
    const LIGHT_DIRECTION: (f64, f64, f64) = (0.0, 1.0, -1.0);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut window = Window::new("Renderer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear(0x000000);

        // Cube vertices
        let cube_coords = vec![
            (-50.0, -50.0, -50.0), // Vertex 0
            (50.0, -50.0, -50.0),  // Vertex 1
            (50.0, 50.0, -50.0),   // Vertex 2
            (-50.0, 50.0, -50.0),  // Vertex 3
            (-50.0, -50.0, 50.0),  // Vertex 4
            (50.0, -50.0, 50.0),   // Vertex 5
            (50.0, 50.0, 50.0),    // Vertex 6
            (-50.0, 50.0, 50.0),   // Vertex 7
        ];

        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) {
            // Calculate rotation angles based on mouse position
            let rotation_x = (mouse_y / HEIGHT as f32) * std::f32::consts::PI * 2.0;
            let rotation_y = (mouse_x / WIDTH as f32) * std::f32::consts::PI * 2.0;
            let rotation_z = 0.0; // Static Z rotation for simplicity

            // Light and colors
            let light_dir = (0.0, 0.0, -1.0); // Light direction for shading

            // Draw the cube with lighting and edge highlighting
            framebuffer.draw_filled_cube_with_lighting(
                cube_coords,
                rotation_x,
                rotation_y,
                rotation_z,
                Colors::RED.as_u32(),
                light_dir,
                Colors::WHITE.as_u32(),
            );
        }

        // Update the window with the framebuffer
        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

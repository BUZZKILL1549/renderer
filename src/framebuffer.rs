pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: u32) {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            self.buffer[y as usize * self.width + x as usize] = color;
        }
    }

    pub fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, color: u32) {
        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1.0 } else { -1.0 };
        let sy = if y0 < y1 { 1.0 } else { -1.0 };
        let mut err = dx + dy;

        loop {
            self.set_pixel(x.round() as isize, y.round() as isize, color);
            if (x - x1).abs() < f64::EPSILON && (y - y1).abs() < f64::EPSILON {
                break;
            }
            let e2 = 2.0 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_triangle(&mut self, v0: (f64, f64), v1: (f64, f64), v2: (f64, f64), color: u32) {
        self.draw_line(v0.0, v0.1, v1.0, v1.1, color);
        self.draw_line(v1.0, v1.1, v2.0, v2.1, color);
        self.draw_line(v2.0, v2.1, v0.0, v0.1, color);
    }

    pub fn intersect_ray_with_polygon(
        &self,
        r_px: f64,
        r_py: f64,
        r_dx: f64,
        r_dy: f64,
        polygon: &[(f64, f64)],
    ) -> Option<(f64, f64)> {
        let mut closest_intersection: Option<(f64, f64)> = None;
        let mut closest_t = f64::INFINITY;

        for i in 0..polygon.len() {
            let (x1, y1) = polygon[i];
            let (x2, y2) = polygon[(i + 1) % polygon.len()];

            let edge_dx = x2 - x1;
            let edge_dy = y2 - y1;

            let denominator = r_dx * edge_dy - r_dy * edge_dx;

            if denominator.abs() < f64::EPSILON {
                // The ray and edge are parallel
                continue;
            }

            // Calculate t1 and t2
            let t1 = ((x1 - r_px) * edge_dy - (y1 - r_py) * edge_dx) / denominator;
            let t2 = ((x1 - r_px) * r_dy - (y1 - r_py) * r_dx) / denominator;

            if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
                if t1 < closest_t {
                    closest_t = t1;
                    closest_intersection = Some((r_px + r_dx * t1, r_py + r_dy * t1));
                }
            }
        }

        closest_intersection
    }

    pub fn project_3d_to_2d(&self, x: isize, y: isize, z: isize) -> (isize, isize) {
        // Define constants for perspective projection
        let fov = 500.0; // Field of view (adjustable)
        let viewer_distance = 400.0; // Distance from the viewer to the object

        // Convert coordinates to f64 for precise calculations
        let x = x as f64;
        let y = y as f64;
        let z = z as f64;

        // Perspective projection formula
        let projected_x = (x * fov) / (z + viewer_distance);
        let projected_y = (y * fov) / (z + viewer_distance);

        // Convert back to isize for pixel coordinates
        (
            (projected_x + self.width as f64 / 2.0).round() as isize,
            (projected_y + self.height as f64 / 2.0).round() as isize,
        )
    }
    pub fn draw_cube(&mut self, vertices: Vec<(isize, isize, isize)>, color: u32) {
        let mut flattened_vector: Vec<(isize, isize)> = Vec::new();

        // Flatten the 3D vertices to 2D
        for vertex in vertices {
            let projected = self.project_3d_to_2d(vertex.0, vertex.1, vertex.2);
            flattened_vector.push(projected);
        }

        // List of edges of the cube based on vertex indices
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0), // Bottom face
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4), // Top face
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7), // Connecting edges
        ];

        // Debug print to check if the vertices are projected correctly
        println!("Flattened vertices:");
        for (x, y) in &flattened_vector {
            println!("({}, {})", x, y);
        }

        // Draw the edges
        for (v0, v1) in &edges {
            let (x0, y0) = flattened_vector[*v0];
            let (x1, y1) = flattened_vector[*v1];

            // Debugging: check if the coordinates are valid
            println!("Drawing line from ({}, {}) to ({}, {})", x0, y0, x1, y1);

            self.draw_line(x0 as f64, y0 as f64, x1 as f64, y1 as f64, color);
        }
    }

    pub fn rotate_cube(
        &self,
        vertices: &[(f32, f32, f32)],
        angle_x: f32,
        angle_y: f32,
        angle_z: f32,
    ) -> Vec<(f32, f32, f32)> {
        let num_vertices = vertices.len() as f32;
        let center = vertices.iter().fold((0.0, 0.0, 0.0), |acc, &(x, y, z)| {
            (acc.0 + x, acc.1 + y, acc.2 + z)
        });

        let center = (
            center.0 / num_vertices,
            center.1 / num_vertices,
            center.2 / num_vertices,
        );

        let cos_x = angle_x.cos();
        let sin_x = angle_x.sin();
        let cos_y = angle_y.cos();
        let sin_y = angle_y.sin();
        let cos_z = angle_z.cos();
        let sin_z = angle_z.sin();

        vertices
            .iter()
            .map(|&(x, y, z)| {
                // Translate to the origin
                let (x, y, z) = (x - center.0, y - center.1, z - center.2);

                // Apply rotation around the X-axis
                let (y, z) = (y * cos_x - z * sin_x, y * sin_x + z * cos_x);

                // Apply rotation around the Y-axis
                let (x, z) = (x * cos_y + z * sin_y, -x * sin_y + z * cos_y);

                // Apply rotation around the Z-axis
                let (x, y) = (x * cos_z - y * sin_z, x * sin_z + y * cos_z);

                // Translate back to the original position
                (x + center.0, y + center.1, z + center.2)
            })
            .collect()
    }

    pub fn draw_filled_cube_with_lighting(
        &mut self,
        vertices: Vec<(f32, f32, f32)>,
        angle_x: f32,
        angle_y: f32,
        angle_z: f32,
        base_color: u32,
        light_dir: (f32, f32, f32),
        edge_color: u32,
    ) {
        // Rotate vertices
        let rotated_vertices = self.rotate_cube(&vertices, angle_x, angle_y, angle_z);

        // Project vertices
        let projected_vertices: Vec<(isize, isize)> = rotated_vertices
            .iter()
            .map(|&(x, y, z)| self.project_3d_to_2d(x as isize, y as isize, z as isize))
            .collect();

        // Define cube faces as triangles
        let faces = vec![
            (0, 1, 2),
            (0, 2, 3), // Bottom face
            (4, 5, 6),
            (4, 6, 7), // Top face
            (0, 1, 5),
            (0, 5, 4), // Front face
            (1, 2, 6),
            (1, 6, 5), // Right face
            (2, 3, 7),
            (2, 7, 6), // Back face
            (3, 0, 4),
            (3, 4, 7), // Left face
        ];

        for &(i1, i2, i3) in &faces {
            let v1 = rotated_vertices[i1];
            let v2 = rotated_vertices[i2];
            let v3 = rotated_vertices[i3];

            // Calculate normal
            let normal = Framebuffer::calculate_normal(v1, v2, v3);

            // Calculate brightness
            let brightness = Framebuffer::calculate_brightness(normal, light_dir);

            // Shade color
            let shaded_color =
                Framebuffer::apply_brightness_to_color(base_color, brightness as f64);

            // Projected vertices for filling
            let p1 = projected_vertices[i1];
            let p2 = projected_vertices[i2];
            let p3 = projected_vertices[i3];

            // Fill the triangle
            self.fill_triangle(p1, p2, p3, shaded_color);
        }

        // Highlight edges
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0), // Bottom face
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4), // Top face
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7), // Connecting edges
        ];

        for &(v0, v1) in &edges {
            let (x0, y0) = projected_vertices[v0];
            let (x1, y1) = projected_vertices[v1];
            self.draw_line(x0 as f64, y0 as f64, x1 as f64, y1 as f64, edge_color);
        }
    }

    pub fn draw_scanline(&mut self, y: isize, x_start: f32, x_end: f32, color: u32) {
        let x_start = x_start as isize;
        let x_end = x_end as isize;
        if y >= 0 && y < self.height as isize {
            let (x_start, x_end) = if x_start < x_end {
                (x_start, x_end)
            } else {
                (x_end, x_start)
            };
            for x in x_start.max(0)..=x_end.min(self.width as isize - 1) {
                self.set_pixel(x as isize, y as isize, color);
            }
        }
    }

    pub fn fill_triangle(
        &mut self,
        v0: (isize, isize),
        v1: (isize, isize),
        v2: (isize, isize),
        color: u32,
    ) {
        // Sort vertices by y-coordinate, so v0 is the top, v2 is the bottom
        let mut vertices = [v0, v1, v2];
        vertices.sort_by(|a, b| a.1.cmp(&b.1));
        let (v0, v1, v2) = (vertices[0], vertices[1], vertices[2]);

        // Calculate slopes for edges
        let slope_0_1 = if v1.1 != v0.1 {
            (v1.0 - v0.0) as f32 / (v1.1 - v0.1) as f32
        } else {
            0.0
        };
        let slope_0_2 = if v2.1 != v0.1 {
            (v2.0 - v0.0) as f32 / (v2.1 - v0.1) as f32
        } else {
            0.0
        };
        let slope_1_2 = if v2.1 != v1.1 {
            (v2.0 - v1.0) as f32 / (v2.1 - v1.1) as f32
        } else {
            0.0
        };

        // Draw upper part of the triangle (from v0 to v1)
        let mut x_start = v0.0 as f32;
        let mut x_end = x_start;
        for y in v0.1..=v1.1 {
            self.draw_scanline(y, x_start, x_end, color);
            x_start += slope_0_1;
            x_end += slope_0_2;
        }

        // Draw lower part of the triangle (from v1 to v2)
        x_start = v1.0 as f32;
        for y in v1.1..=v2.1 {
            self.draw_scanline(y, x_start, x_end, color);
            x_start += slope_1_2;
            x_end += slope_0_2;
        }
    }

    pub fn calculate_normal(
        v0: (f32, f32, f32),
        v1: (f32, f32, f32),
        v2: (f32, f32, f32),
    ) -> (f32, f32, f32) {
        let (x1, y1, z1) = (v1.0 - v0.0, v1.1 - v0.1, v1.2 - v0.2);
        let (x2, y2, z2) = (v2.0 - v0.0, v2.1 - v0.1, v2.2 - v0.2);
        let nx = y1 * z2 - z1 * y2;
        let ny = z1 * x2 - x1 * z2;
        let nz = x1 * y2 - y1 * x2;
        let length = (nx * nx + ny * ny + nz * nz).sqrt();
        (nx / length, ny / length, nz / length)
    }

    pub fn calculate_brightness(normal: (f32, f32, f32), light_direction: (f32, f32, f32)) -> f32 {
        let dot_product = normal.0 * light_direction.0
            + normal.1 * light_direction.1
            + normal.2 * light_direction.2;
        dot_product.max(0.0) // Ensure brightness is non-negative
    }

    pub fn apply_brightness_to_color(color: u32, brightness: f64) -> u32 {
        let r = ((color >> 16) & 0xFF) as f64 * brightness;
        let g = ((color >> 8) & 0xFF) as f64 * brightness;
        let b = (color & 0xFF) as f64 * brightness;
        let r = r.min(255.0).max(0.0) as u32;
        let g = g.min(255.0).max(0.0) as u32;
        let b = b.min(255.0).max(0.0) as u32;
        (r << 16) | (g << 8) | b
    }
}

pub struct Colors {
    value: u32,
}

impl Colors {
    pub const RED: Colors = Colors { value: 0xFF0000 };
    pub const GREEN: Colors = Colors { value: 0x00FF00 };
    pub const BLUE: Colors = Colors { value: 0x0000FF };
    pub const WHITE: Colors = Colors { value: 0xFFFFFF };

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

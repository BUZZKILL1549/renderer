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
}

pub struct Colors {
    value: u32,
}

impl Colors {
    pub const RED: Colors = Colors { value: 0xFF0000 };
    pub const GREEN: Colors = Colors { value: 0x00FF00 };
    pub const BLUE: Colors = Colors { value: 0x0000FF };

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

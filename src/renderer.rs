use crate::triangle::Triangle;

pub const COLOR_BACKGROUND: u32 = 0xFF1E1E1E;
pub const COLOR_GRID: u32 = 0xFF333333;
pub const COLOR_MAGENTA: u32 = 0xFFFF00FF;
pub const COLOR_GREEN: u32 = 0xFF00FF00;

pub struct Renderer {
    color_buffer: Vec<u32>,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            color_buffer: vec![COLOR_BACKGROUND; size],
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let size = (width * height) as usize;
        self.color_buffer = vec![COLOR_BACKGROUND; size];
        self.width = width;
        self.height = height;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn clear(&mut self, color: u32) {
        self.color_buffer.fill(color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y as u32 * self.width + x as u32) as usize;
            self.color_buffer[index] = color;
        }
    }

    pub fn draw_grid(&mut self, spacing: i32, color: u32) {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                if x % spacing == 0 || y % spacing == 0 {
                    self.set_pixel(x, y, color);
                }
            }
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        for dy in 0..height {
            for dx in 0..width {
                self.set_pixel(x + dx, y + dy, color);
            }
        }
    }

    pub fn draw_triangle(&mut self, triangle: &Triangle) {
        if triangle.points.len() != 3 {
            return;
        }

        let p0 = &triangle.points[0];
        let p1 = &triangle.points[1];
        let p2 = &triangle.points[2];

        self.draw_line(
            p0.x as i32,
            p0.y as i32,
            p1.x as i32,
            p1.y as i32,
            triangle.color,
        );
        self.draw_line(
            p1.x as i32,
            p1.y as i32,
            p2.x as i32,
            p2.y as i32,
            triangle.color,
        );
        self.draw_line(
            p2.x as i32,
            p2.y as i32,
            p0.x as i32,
            p0.y as i32,
            triangle.color,
        );
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        // Bresenham's line algorithm
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            self.set_pixel(x, y, color);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    #[allow(dead_code)]
    pub fn draw_line_dda(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let dx = x1 - x0;
        let dy = y1 - y0;

        let mut side_length = dx.abs();
        if dy.abs() > side_length {
            side_length = dy.abs();
        }

        let x_increment = dx as f32 / side_length as f32;
        let y_increment = dy as f32 / side_length as f32;
        let mut current_x = x0 as f32;
        let mut current_y = y0 as f32;

        for _ in 0..side_length {
            self.set_pixel(current_x.round() as i32, current_y.round() as i32, color);
            current_x += x_increment;
            current_y += y_increment;
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.color_buffer.as_ptr() as *const u8,
                self.color_buffer.len() * 4,
            )
        }
    }

    #[cfg(test)]
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<u32> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y as u32 * self.width + x as u32) as usize;
            Some(self.color_buffer[index])
        } else {
            None
        }
    }
}

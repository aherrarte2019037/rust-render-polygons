use nalgebra_glm as glm;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<glm::Vec3>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![glm::vec3(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn draw_polygon(&mut self, points: &[glm::Vec2], fill_color: glm::Vec3, border_color: glm::Vec3) {
        // Engrosar el borde dibujando múltiples líneas desplazadas alrededor de las coordenadas originales
        let offsets = [
            glm::vec2(0.0, 0.0),
            glm::vec2(1.0, 0.0),
            glm::vec2(0.0, 1.0),
            glm::vec2(1.0, 1.0),
            glm::vec2(-1.0, 0.0),
            glm::vec2(0.0, -1.0),
            glm::vec2(-1.0, -1.0),
            glm::vec2(1.0, -1.0),
            glm::vec2(-1.0, 1.0),
        ];

        for offset in &offsets {
            for i in 0..points.len() {
                let start = points[i] + offset;
                let end = points[(i + 1) % points.len()] + offset;
                self.draw_line(start, end, border_color);
            }
        }

        // Rellenar el polígono usando el algoritmo de escaneo
        let ymin = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min) as usize;
        let ymax = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max) as usize;

        for y in ymin..=ymax {
            let mut intersections = Vec::new();
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];

                if (p1.y <= y as f32 && p2.y > y as f32) || (p2.y <= y as f32 && p1.y > y as f32) {
                    let x = p1.x + (y as f32 - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                    intersections.push(x as usize);
                }
            }

            intersections.sort();

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    for x in intersections[i]..=intersections[i + 1] {
                        self.set_pixel(x, y, fill_color);
                    }
                }
            }
        }
    }

    fn draw_line(&mut self, start: glm::Vec2, end: glm::Vec2, color: glm::Vec3) {
        let x0 = start.x as i32;
        let y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        let dx = (x1 - x0).abs();
        let dy = -((y1 - y0).abs());
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        loop {
            self.set_pixel(x as usize, y as usize, color);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
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

    fn set_pixel(&mut self, x: usize, y: usize, color: glm::Vec3) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn get_pixels(&self) -> &[glm::Vec3] {
        &self.pixels
    }
}

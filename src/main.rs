mod framebuffer;
mod bmp;

use framebuffer::FrameBuffer;
use nalgebra_glm as glm;

fn main() {
    // Definir los puntos del polígono
    let points = vec![
        glm::vec2(165.0, 380.0),
        glm::vec2(185.0, 360.0),
        glm::vec2(180.0, 330.0),
        glm::vec2(207.0, 345.0),
        glm::vec2(233.0, 330.0),
        glm::vec2(230.0, 360.0),
        glm::vec2(250.0, 380.0),
        glm::vec2(220.0, 385.0),
        glm::vec2(205.0, 410.0),
        glm::vec2(193.0, 383.0),
    ];

    // Crear el framebuffer
    let mut fb = FrameBuffer::new(500, 500);

    // Dibujar el polígono
    fb.draw_polygon(&points, glm::vec3(1.0, 1.0, 0.0), glm::vec3(1.0, 1.0, 1.0)); // Amarillo con borde blanco

    // Guardar la imagen
    bmp::save_framebuffer_to_bmp(&fb, "out.bmp").expect("Failed to save BMP file");
}

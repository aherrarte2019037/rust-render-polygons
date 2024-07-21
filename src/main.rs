mod bmp;
mod framebuffer;

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

    // Definir colores
    let background_color = glm::vec3(0.0, 0.0, 0.0); // Azul
    let border_color = glm::vec3(1.0, 1.0, 1.0); // Blanco
    let fill_color = glm::vec3(1.0, 1.0, 0.0); // Amarillo

    // Crear el framebuffer
    let mut fb = FrameBuffer::new(500, 500, background_color);

    // Dibujar el polígono
    fb.draw_polygon(&points, fill_color, border_color);

    // Guardar la imagen
    bmp::save_framebuffer_to_bmp(&fb, "out.bmp").expect("Failed to save BMP file");
}

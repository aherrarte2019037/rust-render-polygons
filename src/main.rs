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
    let points2 = vec![
        glm::vec2(321.0, 335.0),
        glm::vec2(288.0, 286.0),
        glm::vec2(339.0, 251.0),
        glm::vec2(374.0, 302.0),
    ];

    // Definir colores
    let background_color = glm::vec3(0.0, 0.0, 0.0);
    let border_color = glm::vec3(1.0, 1.0, 1.0);
    let mut fill_color = glm::vec3(1.0, 1.0, 0.0);

    // Crear el framebuffer
    let mut fb = FrameBuffer::new(500, 500, background_color);

    // Dibujar el polígono
    fb.draw_polygon(&points, fill_color, border_color);

    fill_color = glm::vec3(0.0, 0.0, 1.0);
    fb.draw_polygon(&points2, fill_color, border_color);

    // Guardar la imagen
    bmp::save_framebuffer_to_bmp(&fb, "out.bmp").expect("Failed to save BMP file");
}

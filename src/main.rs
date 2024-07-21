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
    let points3 = vec![
        glm::vec2(411.0, 197.0),
        glm::vec2(436.0, 249.0),
        glm::vec2(401.0, 252.0),
    ];
    let points4 = vec![
        glm::vec2(413.0, 177.0),
        glm::vec2(448.0, 159.0),
        glm::vec2(502.0, 88.0),
        glm::vec2(553.0, 53.0),
        glm::vec2(535.0, 36.0),
        glm::vec2(676.0, 37.0),
        glm::vec2(660.0, 52.0),
        glm::vec2(750.0, 145.0),
        glm::vec2(761.0, 179.0),
        glm::vec2(672.0, 192.0),
        glm::vec2(659.0, 214.0),
        glm::vec2(615.0, 214.0),
        glm::vec2(632.0, 230.0),
        glm::vec2(580.0, 230.0),
        glm::vec2(597.0, 215.0),
        glm::vec2(552.0, 214.0),
        glm::vec2(517.0, 144.0),
        glm::vec2(466.0, 180.0),
    ];
    let points5 = vec![
        glm::vec2(682.0, 175.0),
        glm::vec2(708.0, 120.0),
        glm::vec2(735.0, 148.0),
        glm::vec2(739.0, 170.0),
    ];

    // Definir colores
    let background_color = glm::vec3(0.0, 0.0, 0.0);
    let border_color = glm::vec3(1.0, 1.0, 1.0);
    let mut fill_color = glm::vec3(1.0, 1.0, 0.0);

    // Crear el framebuffer
    let mut fb = FrameBuffer::new(800, 800, background_color);

    // Dibujar el polígono
    fb.draw_polygon(&points, fill_color, border_color);

    fill_color = glm::vec3(0.0, 0.0, 1.0);
    fb.draw_polygon(&points2, fill_color, border_color);

    fill_color = glm::vec3(1.0, 0.0, 0.0);
    fb.draw_polygon(&points3, fill_color, border_color);

    fill_color = glm::vec3(0.0, 1.0, 0.0);
    fb.draw_polygon(&points4, fill_color, border_color);

    fb.draw_polygon(&points5, background_color, background_color);

    // Guardar la imagen
    bmp::save_framebuffer_to_bmp(&fb, "out.bmp").expect("Failed to save BMP file");
}

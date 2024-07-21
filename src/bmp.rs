use std::fs::File;
use std::io::Write;
use crate::framebuffer::FrameBuffer;

pub fn save_framebuffer_to_bmp(fb: &FrameBuffer, filename: &str) -> std::io::Result<()> {
    let width = fb.width as u32;
    let height = fb.height as u32;
    let mut file = File::create(filename)?;

    // Escribir el encabezado BMP
    file.write_all(&[
        0x42, 0x4D, // Signature "BM"
        0, 0, 0, 0, // File size (placeholder)
        0, 0,       // Reserved
        0, 0,       // Reserved
        54, 0, 0, 0 // Offset to pixel data
    ])?;

    // Información del encabezado DIB
    file.write_all(&[
        40, 0, 0, 0,       // DIB header size
        (width & 0xFF) as u8, ((width >> 8) & 0xFF) as u8, ((width >> 16) & 0xFF) as u8, ((width >> 24) & 0xFF) as u8, // Width
        (height & 0xFF) as u8, ((height >> 8) & 0xFF) as u8, ((height >> 16) & 0xFF) as u8, ((height >> 24) & 0xFF) as u8, // Height
        1, 0,             // Color planes
        24, 0,            // Bits per pixel
        0, 0, 0, 0,       // Compression (none)
        0, 0, 0, 0,       // Image size (placeholder)
        0, 0, 0, 0,       // Horizontal resolution (placeholder)
        0, 0, 0, 0,       // Vertical resolution (placeholder)
        0, 0, 0, 0,       // Colors in color table (none)
        0, 0, 0, 0,       // Important color count (all)
    ])?;

    // Escribir los datos de los píxeles
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = fb.get_pixels()[(y * width + x) as usize];
            file.write_all(&[
                (pixel.z * 255.0) as u8,
                (pixel.y * 255.0) as u8,
                (pixel.x * 255.0) as u8,
            ])?;
        }
    }

    Ok(())
}

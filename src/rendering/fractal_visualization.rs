use image::{DynamicImage, ImageBuffer, Rgba};

#[derive(Clone)]
pub struct FractalVisualization;

impl FractalVisualization {
    pub fn new() -> Self { Self }

    pub fn render_3d(&self, field: &[Vec<f64>]) -> (DynamicImage, DynamicImage, DynamicImage) {
        let size = if field.len() > 400_000 { 200 } else { 400 };
        let mut xy_img = ImageBuffer::new(size as u32, size as u32);
        let mut xz_img = ImageBuffer::new(size as u32, size as u32);
        let mut yz_img = ImageBuffer::new(size as u32, size as u32);

        for x in 0..size {
            for y in 0..size {
                let value = field[x * field.len() / size][y * field.len() / size];
                let hue = (value * 255.0) as u8;
                xy_img.put_pixel(x as u32, y as u32, Rgba([hue, 255, 255, 255]));
            }
        }
        for x in 0..size {
            for z in 0..size {
                let value = field[x * field.len() / size][z * field.len() / size];
                let hue = (value * 255.0) as u8;
                xz_img.put_pixel(x as u32, z as u32, Rgba([hue, 255, 204, 255]));
            }
        }
        for y in 0..size {
            for z in 0..size {
                let value = field[y * field.len() / size][z * field.len() / size];
                let hue = (value * 255.0) as u8;
                yz_img.put_pixel(y as u32, z as u32, Rgba([hue, 204, 255, 255]));
            }
        }

        (DynamicImage::ImageRgba8(xy_img), DynamicImage::ImageRgba8(xz_img), DynamicImage::ImageRgba8(yz_img))
    }
}

use image::{ImageBuffer, RgbImage, Rgb};

use crate::color::Color;

#[derive(Debug)]
pub struct Image {
    width: u32,
    height: u32,
    pub pixels: Vec<Color>
}

impl Image {
    pub fn new(width: u32, height: u32, pixels: Vec<Color>) -> Self {
        Image {
            width,
            height,
            pixels
        }
    }

    pub fn export(&self, sample_size: i32) {
        let mut img = RgbImage::new(self.width, self.height);
        for (x,y,pixel) in img.enumerate_pixels_mut() {
            let color = &self.pixels[(y * self.width + x) as usize];
            let sample_size = sample_size as f64;

            let r = (f64::sqrt(color.x() / sample_size) * 255.0) as u8;
            let g = (f64::sqrt(color.y() / sample_size) * 255.0) as u8;
            let b = (f64::sqrt(color.z() / sample_size) * 255.0) as u8;


            *pixel = Rgb([r,g,b]);

        }

        let _ = img.save("output.png");
    }

}
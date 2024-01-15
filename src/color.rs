//utility class, Color is of type Vec3
use std::io::{stdout, Write};
use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color{
    //used to convert float RGB to decimal RGB and print to stdout
    pub fn write_color(&self, samples: i32) {

        let samples: f64 = samples as f64;

        let r: u32 = (f64::sqrt(self.x() / samples) * 255.0) as u32;
        let g: u32 = (f64::sqrt(self.y() / samples) * 255.0) as u32;
        let b: u32 = (f64::sqrt(self.z() / samples) * 255.0) as u32;

        // r = clamp(r, 0, 255);
        // g = clamp(r, 0, 255);
        // b = clamp(r, 0, 255);
        

        writeln!(stdout(), "{} {} {}", r,g,b).ok();
    }
}
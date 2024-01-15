// use std::fmt::Debug;

// use crate::{material::Material, ray::Ray, hittable::Record, color::Color, vec3::Vec3};

// #[derive(Debug, Clone, Copy)]
// pub struct Diffuse {
//     color: Color
// }

// impl Diffuse {
//     pub fn new(color: Color) -> Self {
//         Diffuse {
//             color
//         }
//     }
// }

// impl Material for Diffuse {
//     fn scatter(&self, ray_in: &Ray, curr_record: &Record) -> (bool, Color, Ray) {
//         // todo!()
//         let scatter_dir = Vec3::vec_in_unit_hemisphere(curr_record.normal) + curr_record.normal;
//         let ray_out = Ray::new(curr_record.point, scatter_dir);
//         let color_out = self.color;
//         (true, color_out, ray_out)
//     }
// }
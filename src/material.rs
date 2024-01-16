use crate::{hittable::Record, color::Color, ray::Ray, vec3::{Vec3, WHITE, BLACK}, util::gen_random};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Diffuse {
        color: Color
    }, 
    Metal {
        color: Color,
        roughness: f64,
    },
    Dielectric {
        ior: f64
    },
    Glossy {
        color: Color,
        specularity:f64,
        roughness: f64
    },
    UV,
    Stripes,
    Empty,
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, curr_record: &Record) -> Option<(Color, Ray)> {
        match self {
            Material::Diffuse { color } => {
                let scatter_dir = Vec3::vec_in_unit_hemisphere(curr_record.normal) + curr_record.normal;
                let ray_out = Ray::new(curr_record.point, scatter_dir);
                let color_out = *color;
                Some((color_out, ray_out))
            }
            Material::Metal { color, roughness } => {
                let refractive_ratio: f64 = if curr_record.outside_face {1.0/2.2} else {2.2};

                let cos = f64::min(Vec3::dot(-ray_in.direction.unit(), curr_record.normal.unit()), 1.0);
                let mut r0 = (1.0 - refractive_ratio)/(1.0 + refractive_ratio);
                r0 *= r0;
                let R = r0 + (1.0 - r0)*f64::powi(1.0 - cos, 5);

                let mut scatter_dir = Vec3::reflect(ray_in.direction, curr_record.normal);
                scatter_dir = scatter_dir + (*roughness * Vec3::random_unit_vec());
                let ray_out = Ray::new(curr_record.point, scatter_dir);
                let color_out = (1.0 - R) * *color + R * WHITE;

                Some((color_out, ray_out))
            },
            Material::UV => {
                Some(((Color::new(curr_record.normal.x(), 
                curr_record.normal.y(), 
                curr_record.normal.z()) 
                + Vec3::new(1.0, 1.0, 1.0)) 
                * 0.5, 
                Ray::new(curr_record.point, curr_record.normal)))
            },
            Material::Stripes => {
                let black = Color::default();
                let white = Color::new(1.0,1.0,1.0);
                let f: Color;

               
                
                let a = (f64::sin(curr_record.point.x()) + 1.0) / 2.0 + (f64::sin(curr_record.point.y()) + 1.0) / 2.0 ;

                f = (1.0 - a) * black + a* white;

                Some((f, Ray::new(curr_record.point, curr_record.normal)))
            }
            Material::Empty => None,
            Material::Dielectric { ior } => {
                let refractive_ratio: f64 = if curr_record.outside_face {1.0/ior} else {*ior};

                let cos = f64::min(Vec3::dot(-ray_in.direction.unit(), curr_record.normal.unit()), 1.0);
                let sin = f64::sqrt(1.0 - cos * cos);

                let scatter_dir: Vec3;

                let mut r0 = (1.0 - refractive_ratio)/(1.0 + refractive_ratio);
                r0 *= r0;
                let reflectance = r0 + (1.0 - r0)*f64::powi(1.0 - cos, 5);

                if refractive_ratio * sin > 1.0 || reflectance > gen_random() {
                    scatter_dir = Vec3::reflect(ray_in.direction.unit(), curr_record.normal);
                } else {
                    scatter_dir = Vec3::refract(ray_in.direction.unit(), curr_record.normal, refractive_ratio);
                }

                let color = (1.0 - reflectance) * WHITE + reflectance * WHITE;

                let ray_out : Ray = Ray::new(curr_record.point, scatter_dir);
                Some((color, ray_out))
            },
            Material::Glossy { specularity, roughness, color } => {

                let cos = f64::min(Vec3::dot(-ray_in.direction.unit(), curr_record.normal.unit()), 1.0);
                let reflectance = specularity + (1.0 - specularity)*f64::powi(1.0 - cos, 5);


                let is_specular = if reflectance >= gen_random() {1.0}  else {0.0};

                let diff_dir = (Vec3::vec_in_unit_hemisphere(curr_record.normal) + curr_record.normal).unit();
                let reflection_dir = Vec3::reflect(ray_in.direction, curr_record.normal).unit();

                let mut scatter_dir = reflection_dir * ((is_specular * (1.0 - roughness))) + (1.0 - (is_specular * (1.0 - roughness))) * diff_dir;
                scatter_dir = scatter_dir.unit();


                let ray_out = Ray::new(curr_record.point, scatter_dir);
                let color_out = (1.0 - is_specular) * *color + is_specular * WHITE;

                Some((color_out, ray_out))

            }
        }
    }
}
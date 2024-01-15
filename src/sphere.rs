use crate::{hittable::{ Record, Hittable}, vec3::Vec3, material::Material};

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius:f64, material: Material) -> Sphere {
        Sphere {center, radius, material}
    }
}

impl Hittable for Sphere {
    fn ray_hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<Record> {
        let oc: Vec3 = ray.origin() - self.center;
        let a = Vec3::dot(ray.direction(), ray.direction());
        let b = Vec3::dot(2.0 * ray.direction(), oc);
        let c = Vec3::dot(oc, oc) - (self.radius * self.radius);
        let disc = (b*b)-(4.0*a*c);

        if disc < 0.0 {
            None
        } else {
            let mut t: f64 = (-b - f64::sqrt(disc))/(2.0 * a);
            if (t_min > t) || (t > t_max) {
                t = (-b + f64::sqrt(disc))/(2.0 * a);
                if (t_min > t) || (t > t_max) {
                    return None
                }
            }

            let mut return_record: Record = Record::new();
            
            return_record.t = t;
            return_record.point= ray.ray_at(t);
            return_record.material = self.material;
            let normal = (return_record.point - self.center)/self.radius;
            return_record.calculate_normal(ray, normal);
            return Some(return_record);
        }
    }
}

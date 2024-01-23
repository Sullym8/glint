use crate::{vec3::{Point3, Vec3}, material::Material, hittable::{Hittable, Record}, ray::Ray, aabb::AABB};

pub struct Plane {
    point: Point3,
    normal: Vec3,
    material: Material,
    bounds: AABB
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, material: Material) -> Self {
        Plane {point, normal, material, bounds: AABB::default()}
    }
}

impl Hittable for Plane {
    fn ray_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Record> {
        let qp: Vec3 = self.point - ray.origin();
        let v_dot_n = Vec3::dot(ray.direction(), self.normal);

        if v_dot_n == 0.0 {
            return None
        }

        let qp_dot_n = Vec3::dot(qp, self.normal);
        let t: f64 = qp_dot_n/v_dot_n;

        if t_min > t || t > t_max {
            return None;
        }

        let mut return_record = Record::new();

        return_record.t = t;
        return_record.point = ray.ray_at(t);
        return_record.material = self.material;
        return_record.calculate_normal(ray, self.normal.unit());
        // eprintln!("{:?}", return_record);/
        return Some(return_record)
    }

    fn bounds(&self) -> &crate::aabb::AABB {
        &self.bounds
    }

    fn centroid(&self) -> Vec3 {
        Vec3::default()
    }
}
use crate::{vec3::{Vec3, Point3}, material::Material, hittable::{Hittable, Record}};

pub struct Triangle {
    p1: Point3,
    p2: Point3,
    p3: Point3,
    e1: Vec3,
    e2: Vec3,
    normal: Vec3,
    material: Material
}

impl Triangle {
    pub fn new(p1: Point3, p2: Point3, p3: Point3) -> Self{
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = Vec3::cross(e1, e2);

        println!("{:?} {:?} {:?}", e1, e2, normal);
        Triangle {
            p1, p2, p3, e1, e2, normal, material : Material::UV
        }
    }
}

impl Hittable for Triangle {
    fn ray_hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::Record> {


        let v_dot_n = Vec3::dot(ray.direction(), self.normal);

        if v_dot_n == 0.0 {
            return None
        }

        let d = -Vec3::dot(self.p1, self.normal);
        // println!("{d}");

        let qp: Vec3 = self.p1 - ray.origin();


        let qp_dot_n = Vec3::dot(qp, self.normal);
        // let x = Vec3::dot(ray.origin, self.normal) + d;

        let t: f64 = qp_dot_n/v_dot_n;

        if t_min > t || t > t_max {
            return None;
        }

        let p = ray.ray_at(t);

        //E1

        let v = self.p2 - self.p1;
        let e = p - self.p1;
        let c = Vec3::cross(v, e);
        if (Vec3::dot(self.normal, c) < 0.0) {return None;}

        //E2

        let v = self.p3 - self.p2;
        let e = p - self.p2;
        let c = Vec3::cross(v, e);
        if (Vec3::dot(self.normal, c) < 0.0) {return None;}

        //E3

        let v3 = self.p1 - self.p3;
        let e = p - self.p3;
        let c = Vec3::cross(v3, e);
        if (Vec3::dot(self.normal, c) < 0.0) {return None;}

        let mut return_record = Record::new();

        return_record.t = t;
        return_record.point = ray.ray_at(t);
        return_record.material = self.material;
        return_record.calculate_normal(ray, self.normal.unit());
        // println!("{:?}", return_record);
        return Some(return_record)
    }
}
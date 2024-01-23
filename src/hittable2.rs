use crate::{material::Material, vec3::{Vec3, Point3}, sphere::Sphere, triangle::Triangle, hittable::{Record, Hittable}, ray::Ray, aabb::AABB, trimesh::TriMesh};

#[derive(Clone, Copy)]
pub enum Primitive {
    Sphere(Sphere),
    Triangle(Triangle),
}


impl Primitive {
   pub fn ray_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Record> {
    return match self {
        Primitive::Sphere(s) => s.ray_hit(ray, t_min, t_max),
        Primitive::Triangle(t) => t.ray_hit(ray, t_min, t_max),
    }
   }

   pub fn bounds(&self) -> &AABB {
    return match self {
        Primitive::Sphere(s) => &s.bounds,
        Primitive::Triangle(t) => &t.bounds,
    }
   }

   pub fn centroid(&self) -> Point3 {
    return match self {
        Primitive::Sphere(s) => s.centroid(),
        Primitive::Triangle(t) => t.centroid(),
    };
   }

}
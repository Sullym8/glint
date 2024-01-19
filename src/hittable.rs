// Abstraction of hittable objects
use crate::{ray::Ray, vec3::{Point3, Vec3}, material::Material, aabb::AABB};

//A 'log' of the ray intersections that occured, stores important metadata
#[derive(Debug)]
pub struct Record{
    pub t: f64,
    pub point: Point3,
    pub normal: Vec3,
    pub outside_face: bool,
    pub material: Material,
}

pub struct HittableVec {
    pub list: Vec<Box<dyn Hittable + Send + Sync>>,
    bounds: AABB,
}

impl HittableVec {
    pub fn new() -> Self {
        HittableVec { list: Vec::new(), bounds: AABB::default()}
    }

    pub fn add(&mut self, hittable: impl Hittable + 'static + Send + Sync) -> () {
        self.bounds.join(hittable.bounds());
        self.list.push(Box::new(hittable));
    }
}

impl Hittable for HittableVec {
    fn ray_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Record> {
        // println!("World: {:?}", self.bounds);
        if self.bounds.hit(ray) { 
            let mut curr_record: Record = Record::new();
            let mut curr_hit: bool = false;
            let mut curr_closest: f64 = t_max;

            for object in &self.list {
                // println!("Obj: {:?}", object.bounds());
                let res = object.ray_hit(ray, t_min, curr_closest);
                match res {
                    Some(x) => {
                        curr_record = x;
                        curr_hit = true;
                        curr_closest = curr_record.t;
                    },
                    None => {}
                }
            }
            return if curr_hit {Some(curr_record)} else {None};
        }
        None
    }

    fn bounds(&self) -> &AABB {
        &self.bounds
    }

}

impl Record{
    //default cons
    pub fn new() -> Record {
        Record{
            t: 0.0,
            point: Vec3::new(0.0,0.0,0.0),
            normal: Vec3::new(0.0,0.0,0.0),
            outside_face: true,
            material: Material::Empty
        }
    }

    pub fn calculate_normal(&mut self,ray: &Ray, normal: Vec3) -> () {
        if Vec3::dot(ray.direction(), normal) > 0.0 {
            self.normal = -normal;
            self.outside_face = false;
        } else {
            self.normal = normal;
            self.outside_face = true;
        }
    }
}

// Trait that can be implmented by objects that interact with rays
// Takes in self, ray, a range of valid t, and a record object which it writes to -> bool
pub trait Hittable {
    fn ray_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Record>;
    fn bounds(&self) -> &AABB;
    // fn bounding_box_hit() -> AABB;
}
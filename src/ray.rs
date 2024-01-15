// Utility class for a ray.
// Based on the mathematical definition of a line L = A + tB
// A is a point on the line, and B is the direction vector. 
// t is scalar that helps us move along the line

use crate::vec3::{Point3, Vec3};

//r = A = tB
#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Point3
}

impl Ray {
    //constructor
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray{
            origin,
            direction
        }
    }

    //getters
    pub fn origin(&self) -> Point3 {
        Point3::new(self.origin.x(), self.origin.y(), self.origin.z())
    }

    pub fn direction(&self) -> Vec3 {
        Vec3::new(self.direction.x(), self.direction.y(), self.direction.z())
    }

    //returns position as Point3 given a value of t
    pub fn ray_at(&self, t: f64) -> Point3 {
        self.origin() + t * self.direction()
    }
}


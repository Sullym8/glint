use crate::{vec3::{Vec3, MAX, MIN}, ray::Ray};

#[derive(Debug, Clone, Copy)]

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn default() -> Self {
        AABB { min: MAX, max: MIN}
    }

    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB {min,max}
    }

    pub fn add(&mut self, point: Vec3) {
        self.min.v[0] = if point.x() < self.min.v[0] {point.x()} else {self.min.v[0]};
        self.min.v[1] = if point.y() < self.min.v[1] {point.y()} else {self.min.v[1]};
        self.min.v[2] = if point.z() < self.min.v[2] {point.z()} else {self.min.v[2]};

        self.max.v[0] = if point.x() > self.max.v[0] {point.x()} else {self.max.v[0]};
        self.max.v[1] = if point.y() > self.max.v[1] {point.y()} else {self.max.v[1]};
        self.max.v[2] = if point.z() > self.max.v[2] {point.z()} else {self.max.v[2]};
    }

    pub fn join(&mut self, existing: &AABB) {
        self.add(existing.min);
        self.add(existing.max);
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        // println!("{:?}", ray);
        let mut t_min: f64;
        let mut t_max: f64;

        let t_x_min = f64::min(
            (self.min.x() - ray.origin.x())/ray.direction.x(),
            (self.max.x() - ray.origin.x())/ray.direction.x()
        );
        let t_x_max = f64::max(
            (self.min.x() - ray.origin.x())/ray.direction.x(),
            (self.max.x() - ray.origin.x())/ray.direction.x()            
        );
        
        t_min = t_x_min;
        t_max = t_x_max;

        // println!("X: {t_min} {t_max}");

        let t_y_min = f64::min(
            (self.min.y() - ray.origin.y())/ray.direction.y(),
            (self.max.y() - ray.origin.y())/ray.direction.y()
        );
        let t_y_max = f64::max(
            (self.min.y() - ray.origin.y())/ray.direction.y(),
            (self.max.y() - ray.origin.y())/ray.direction.y()
        );

        // println!("Y: {t_y_min} {t_y_max}");


        if (t_y_max < t_min || t_y_min > t_max) {
            return false;
        }

        if (t_y_min > t_min) {
            t_min = t_y_min;
        }

        if (t_y_max < t_max) {
            t_max = t_y_max;
        }

        // println!("Y: {t_min} {t_max}");

        let t_z_min = f64::min(
            (self.min.z() - ray.origin.z())/ray.direction.z(),
            (self.max.z() - ray.origin.z())/ray.direction.z()
        );
        let t_z_max = f64::max(
            (self.min.z() - ray.origin.z())/ray.direction.z(),
            (self.max.z() - ray.origin.z())/ray.direction.z()
        );

        // println!("Z: {t_z_min} {t_z_max}");


        if (t_z_max < t_min || t_z_min > t_max) {
            return false;
        }

        if (t_z_min > t_min) {
            t_min = t_z_min;
        }

        if (t_z_max < t_max) {
            t_max = t_z_max;
        }

        // println!("AABB Hit!");

        return true;
    }

}

//vec3 utility class, Point3 and Color are Vec3 types

use std::ops::{Add, Neg, Sub, Mul, Div};
use std::fmt::Display;

use crate::util::gen_random_range;

//used to simplify dev, may remove
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub v: [f64; 3]
}

pub const WHITE: Vec3 = Vec3 {
    v: [1.0;3]
};

pub const BLACK: Vec3 = Vec3 {
    v: [0.0;3]
};

pub const ORIGIN: Vec3 = Vec3 {
    v: [0.0;3]
};

pub const MIN: Vec3 = Vec3 {
    v:[f64::NEG_INFINITY;3]
};

pub const MAX: Vec3 = Vec3 {
    v:[f64::INFINITY;3]
};



impl Vec3 {
    //constructor
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            v: [x,y,z]
        }
    }

    pub fn newf32(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            v: [x as f64,y as f64,z as f64]
        }
    }

    pub fn default() -> Vec3 {
        Vec3 {
            v: [0.0,0.0,0.0]
        }
    }

    //getters for x, y, z
    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }

    pub fn z(&self) -> f64 {
        self.v[2]
    }

    //calculate norm of vector (distance from Origin)
    pub fn length(&self) -> f64 {
        f64::sqrt(self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2])
    }

    //vector dot mult
    pub fn dot(u:Vec3, v:Vec3) -> f64 {
        u.x() * v.x() +
        u.y() * v.y() +
        u.z() * v.z()
    }

    //vector cross mult
    pub fn cross(u:Vec3, v:Vec3) -> Vec3 {
        Vec3::new(u.y() * v.z() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.y() * v.x())
    }

    //
    pub fn norm(&self) -> f64 {
        f64::sqrt(self.x() * self.x() + self.y() * self.y() + self.z() * self.z())
    }

    //returns unit vector. (Vector pointing in the same direction but normalized to have length 1)
    pub fn unit(&self) -> Vec3 {
        Vec3::new(self.x()/self.norm(), self.y()/self.norm(), self.z()/self.norm())
    }

    pub fn random() -> Self {
        Vec3 { v: [
            gen_random_range(-1.0, 1.0),
            gen_random_range(-1.0, 1.0),
            gen_random_range(-1.0, 1.0)
        ] }
    }

    pub fn random_vec() -> Self {
        loop {
            let vec = Vec3::random();
            if vec.length() < 1.0 {
                return vec;
            }
        }
    }

    pub fn random_unit_vec() -> Self {
        let vec = Vec3::random_vec();
        vec.unit()
    } 

    pub fn vec_in_unit_hemisphere(normal: Vec3) -> Self {
        let vec = Vec3::random_unit_vec();
        if Vec3::dot(normal, vec) > 0.0 {
            vec
        } else {
            -vec
        }
    }

    pub fn vec_in_unit_disk() -> Vec3 {
        loop {
            let vec = Vec3 { v: [
                gen_random_range(-1.0, 1.0),
                gen_random_range(-1.0, 1.0),
                0.0
            ]};

            if vec.length() < 1.0 {
                return vec
            } 
        }
    }

    // reflected ray = v + 2*h
    // v.n = v*n*cos(theta)
    // h = -vcos(theta)
    pub fn reflect(dir_in: Vec3, normal: Vec3) -> Vec3 {
        dir_in - (2.0 * Vec3::dot(dir_in, normal) * normal)
    }

    pub fn refract(dir_in: Vec3, normal: Vec3, refractive_ratio: f64) -> Vec3 {
        let cos = Vec3::dot(-dir_in, normal);
        let r_perp = refractive_ratio * (dir_in + cos * normal);
        let r_parallel = - f64::sqrt(1.0 - (Vec3::length(&r_perp)) * (Vec3::length(&r_perp))) * normal;
        r_perp + r_parallel
    }

}

pub type Point3 = Vec3;

//toString
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

//vector math
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x()/rhs, self.y()/rhs, self.z()/rhs)
    }
}







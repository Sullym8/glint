use tobj::GPU_LOAD_OPTIONS;

use crate::{vec3::Vec3, triangle::Triangle, aabb::AABB, hittable::{Hittable, Record}};

pub struct TriMesh {
    pub triangles: Vec<Triangle>,
    pub bounds: AABB,
}

impl TriMesh {
    pub fn new() -> TriMesh{
        let mut bounds = AABB::default();
        let obj = tobj::load_obj("stormtrooper.obj", &GPU_LOAD_OPTIONS);
        let (models, _) = obj.unwrap();

        let mut triangles: Vec<Triangle> = Vec::new();
        for model in models {
            // println!("{:?}", model.mesh);
            for i in (0..model.mesh.indices.len()).step_by(3) {
                // println!("{} {} {}", model.mesh.indices[i], model.mesh.indices[i + 1], model.mesh.indices[i + 2])
                let f1 = model.mesh.indices[i] as usize;
                let f2 = model.mesh.indices[i + 1] as usize;
                let f3 = model.mesh.indices[i + 2] as usize;

                let p1 = &model.mesh.positions[3* f1..3 * f1+3];
                let p2 = &model.mesh.positions[3 * f2..3 * f2+3];
                let p3 = &model.mesh.positions[3 * f3..3 * f3+3];

                // println!("{:?} {:?} {:?}", p1, p2, p3);




                let t = Triangle::new(
                    Vec3::newf32(p1[0], p1[1], p1[2]), 
                    Vec3::newf32(p2[0], p2[1], p2[2]), 
                    Vec3::newf32(p3[0], p3[1], p3[2])
                );
                triangles.push(t);
                bounds.join(&t.bounds);
                // println!("{:?}", bounds);
            }
        }

        TriMesh{
            triangles,
            bounds
        }
    }

}

impl Hittable for TriMesh {
    fn ray_hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::Record> {
        if self.bounds.hit(ray) {
            let mut curr_record: Record = Record::new();
            let mut curr_hit: bool = false;
            let mut curr_closest: f64 = t_max;

            for triangle in &self.triangles {
                let res = triangle.ray_hit(ray, t_min, curr_closest);
                match res {
                    Some(x) => {
                        curr_record = x;
                        curr_hit = true;
                        curr_closest = curr_record.t;
                    },
                    None => {}
                }
            }
            return if curr_hit {Some(curr_record)} else {None}


            // for triangle in &self.triangles {
            //     if triangle.bounds.hit(ray) {
            //         return triangle.ray_hit(ray, t_min, t_max)
            //     }
            // }
        }
        None
    }

    fn bounds(&self) -> &AABB {
        &self.bounds
    }
}
use tobj::{OFFLINE_RENDERING_LOAD_OPTIONS, LoadOptions, GPU_LOAD_OPTIONS};

use crate::{vec3::{Point3, Vec3}, triangle::Triangle};

pub struct TriMesh {
    pub triangles: Vec<Triangle>
}

impl TriMesh {
    pub fn new() -> TriMesh{
        let obj = tobj::load_obj("glint.obj", &GPU_LOAD_OPTIONS);
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
            }
        }

        TriMesh{
            triangles
        }

    }

}
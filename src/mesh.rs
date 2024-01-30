use tobj::GPU_LOAD_OPTIONS;

use crate::{aabb::{AABB, self}, color::Color, hittable::{Hittable, Record}, material::{self, Material}, triangle::Triangle, vec3::Vec3};

pub struct TriMesh {
    pub triangles: Vec<Triangle>,
    bounds: AABB
}

impl TriMesh {
    pub fn new(file_name: &str, material: Material) -> TriMesh{
        let obj = tobj::load_obj(file_name, &GPU_LOAD_OPTIONS);
        let (models, mats) = obj.unwrap();
        let materials = mats.unwrap();

        let mut triangles: Vec<Triangle> = Vec::new();
        for model in models {
            let mat = match model.mesh.material_id {
                Some(id) => match (materials[id].diffuse, &materials[id].name) {
                    (Some([r,g,b]), name) => {
                        println!("{name}");
                        if name.starts_with("d") {
                            Material::Diffuse { color: Color::newf32(r, g, b) }
                        } else if name.starts_with("g") {
                            Material::Glossy { color: Color::newf32(r, g, b), specularity: 0.15, roughness: 0.0 }
                        } else if name.starts_with("e") {
                            Material::Emission { color: Color::newf32(r, g, b), strength: 5.0 }
                        } else {
                            Material::Empty
                        }
                    },
                    _ => Material::Empty
                }
                None => Material::Empty
            };

            for i in (0..model.mesh.indices.len()).step_by(3) {
                // println!("{} {} {}", model.mesh.indices[i], model.mesh.indices[i + 1], model.mesh.indices[i + 2])
                let f1 = model.mesh.indices[i] as usize;
                let f2 = model.mesh.indices[i + 1] as usize;
                let f3 = model.mesh.indices[i + 2] as usize;

                let p1 = &model.mesh.positions[3 * f1..3 * f1+3];
                let p2 = &model.mesh.positions[3 * f2..3 * f2+3];
                let p3 = &model.mesh.positions[3 * f3..3 * f3+3];

                let n1 = &model.mesh.normals[3 * f1..3 * f1+3];
                let n2 = &model.mesh.normals[3 * f2..3 * f2+3];
                let n3 = &model.mesh.normals[3 * f3..3 * f3+3];

                let mut t = Triangle::new(
                    Vec3::newf32(p1[0], p1[1], p1[2]), 
                    Vec3::newf32(p2[0], p2[1], p2[2]), 
                    Vec3::newf32(p3[0], p3[1], p3[2]),
                    // Material::Diffuse { color: color }
                    // Material::Metal { color: color, roughness: 0.0 }
                    // Material::Glossy { color: color, specularity: 0.15, roughness: 0.0 }
                    mat
                );
                t.n1 = Some(Vec3::newf32(n1[0], n1[1], n1[2]));
                t.n2 = Some(Vec3::newf32(n2[0], n2[1], n2[2]));
                t.n3 = Some(Vec3::newf32(n3[0], n3[1], n3[2]));
                // println!("{:?}", t);
                triangles.push(t);
            }
        }

        TriMesh{
            triangles,
            bounds: AABB::default()
        }
    }

}

impl Hittable for TriMesh {
    fn ray_hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::Record> {
        // if self.bounds.hit(ray) {
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
            return if curr_hit {Some(curr_record)} else {None};


            // for triangle in &self.triangles {
            //     if triangle.bounds.hit(ray) {
            //         return triangle.ray_hit(ray, t_min, t_max)
            //     }
            // }
        // }
        None
    }

    fn bounds(&self) -> &AABB {
        &self.bounds
    }

    fn centroid(&self) -> Vec3 {
        Vec3::default()
    }
}
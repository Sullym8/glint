mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod util;
mod material;
mod plane;
mod image;
mod triangle;
mod mesh;
mod aabb;
mod bvh;
mod hittable2;

use camera::Camera;
use color::Color;
use material::Material;
use sphere::Sphere;
use mesh::TriMesh;
use vec3::{Point3, WHITE};

use crate::{vec3::{Vec3, BLACK}, hittable::HittableVec, bvh::BVHNode, hittable2::Primitive};

// where the raytracing appens

fn main() {


    //World is a list of objects that we want to be raytraced. So far there are spheres :)
    let mut world: HittableVec = HittableVec::new();


    // let ground_material = Material::Metal { color: Color::new(0.98, 0.75, 0.24), roughness: 0.0};
    let ground_material = Material::Glossy { color: Color::new(1.0, 0.3, 0.2), specularity: 0.15, roughness: 0.3};

    let m = TriMesh::new("helmet.obj", Material::Glossy { color: WHITE, roughness: 0.0, specularity: 0.02});

    let mut primitives: Vec<Primitive> = vec![];
    let mut i: usize = 0;
    let mut indices = vec![];
    for t in m.triangles {
    // primitives.push(Primitive::Sphere(Sphere::new(Vec3::default(), 1.0, Material::Empty)));
    // indices.push(i);
    // i += 1;
    // primitives.push(Primitive::Sphere(Sphere::new(Vec3::default(), 2.0, Material::Empty)));
    // indices.push(i);
    // i += 1;
    // primitives.push(Primitive::Sphere(Sphere::new(Vec3::default(), 3.0, Material::Empty)));
        primitives.push(Primitive::Triangle(t));
        indices.push(i);
        i += 1;
    }

    // primitives.push(Primitive::Sphere(
    //     Sphere::new(
    //         Point3::new(0.0, -50.0, 0.0), 
    //         50.0, 
    //         ground_material
    //     )
    // ));
    // indices.push(i);
    // i+= 1;

    // primitives.push(Primitive::Sphere(
    //     Sphere::new(
    //         Point3::new(0.0, 1100.0, 0.0), 
    //         1000.0, 
    //         Material::Emission { color: WHITE, strength: 1.0 }
    //     )
    // ));
    // indices.push(i);
    // i+= 1;


    // primitives.push(Primitive::Sphere(
    //     Sphere::new(
    //         Point3::new(-15.0, 0.0, 10.0), 
    //         5.0, 
    //         Material::Emission { 
    //             color: Color::new(0.0, 0.0, 1.0), 
    //             strength: 2.0 }
    //     )
    // ));
    // indices.push(i);
    // i+= 1;

    let mut bvh = BVHNode::default();
    println!("Building BVH...");
    bvh = bvh.new(&primitives, &mut indices, 0, i, 1024);
    println!("BVH Built");

    let mut camera: Camera = Camera::new();
    camera.image_width = 1920;
    camera.image_height = 1080;
    camera.samples = 256;
    camera.ray_depth = 5;
    camera.fov = 35.0;
    camera.look_from = Point3::new(60.0, 50.0,80.0);
    camera.look_at = Vec3::new(0.0, 25.0, 0.0);

    camera.look_from = Point3::new(16.0, 6.0,18.0);
    camera.look_at = Vec3::new(0.0, 1.5, 0.0);

    camera.look_from = Point3::new(-1.4, -0.3,9.0);
    camera.look_at = Vec3::new(-1.4, -0.3, 0.0);

    camera.render(&world, &bvh, &primitives);
    camera.output.export(camera.samples);

   
}

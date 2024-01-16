mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod util;
mod material;
mod diffuse;
mod plane;
mod image;
mod triangle;

use camera::Camera;
use color::Color;
use material::Material;
use plane::Plane;
use sphere::Sphere;
use triangle::Triangle;
use vec3::Point3;

use crate::{vec3::Vec3, hittable::HittableVec};

// where the raytracing magic happens

fn main() {


    //World is a list of objects that we want to be raytraced. So far there are spheres :)
    let mut world: HittableVec = HittableVec::new();

    let ground_material = Material::Glossy { color: Color::new(0.1, 0.1, 0.4), specularity: 0.02, roughness: 0.1 };
    // world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));
    world.add(Plane::new(Point3::new(0.0, -0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), ground_material));
    // for a in -11..12{
    //     for b in -11..12 {
    //         let choose_mat = gen_random();
    //         let center = Point3::new(a as f64 + 0.9*gen_random(), 0.2, b as f64 + 0.9*gen_random());

    //         if ((center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9) {
    //             let sphere_material: Material;

    //             if (choose_mat < 0.85) {
    //                 // diffuse
    //                 let albedo = Vec3::new(gen_random_range(0.0, 1.0), 
    //                 gen_random_range(0.0, 1.0), 
    //                 gen_random_range(0.0, 1.0));
    //                 sphere_material = Material::Glossy { color: albedo, specularity: 0.1, roughness: 0.05 };
    //                 world.add(Sphere::new(center, 0.2, sphere_material));
    //             } else if (choose_mat < 0.95) {
    //                 // metal
    //                 let albedo = Vec3::new(gen_random_range(0.5, 1.0), 
    //                 gen_random_range(0.5, 1.0), 
    //                 gen_random_range(0.5, 1.0));
    //                 let fuzz = gen_random_range(0.0, 0.5);
    //                 sphere_material = Material::Metal { color: albedo, roughness: fuzz };
    //                 world.add(Sphere::new(center, 0.2, sphere_material));
    //             } else {
    //                 // glass
    //                 sphere_material = Material::Dielectric { ior: 1.5 };
    //                 world.add(Sphere::new(center, 0.2, sphere_material));
    //             }
    //         }
    //     }
    // }

    let material1 = Material::Glossy { color: Color::new(1.0, 1.0, 1.0), specularity: 0.15, roughness: 0.0 };
    // let material1 = Material::Dielectric { ior: 1.5 };
    world.add(Sphere::new(Point3::new(2.0, 1.0, 2.0), 1.0, material1));

    // let material2 = Material::Diffuse { color: Color::new(0.4, 0.2, 0.1) };
    let material2 = Material::Dielectric { ior: 1.5 };

    // world.add(Sphere::new(Point3::new(0.0, 0.2, 2.0), 0.2, material2));

    // world.add(Sphere::new(Point3::new(0.0, 1.0, 2.0), 1.0, material2));

    let material3 = Material::Metal { color: Color::new(0.7, 0.6, 0.5), roughness: 0.2};
    world.add(Sphere::new(Point3::new(-2.0, 1.0, 2.0), 1.0, material3));

    world.add(Triangle::new(Vec3::new(0.0, 1.0, 2.0), Vec3::new(1.0, 1.0, 2.0), Vec3::new(0.0, 2.0, 0.0)));

    let mut camera: Camera = Camera::new();
    camera.image_width = 400;
    camera.image_height = 200;
    camera.samples = 100;
    camera.ray_depth = 5;
    camera.fov = 60.0;
    camera.look_from = Point3::new(0.0, 1.0,10.0);
    camera.look_at = Vec3::new(0.0, 1.0, 1.0);

    // camera.look_from = Point3::new(-0.0, 1.0, 1.0);
    // camera.look_at = Vec3::new(0.0, 1.0, -1.0);

    camera.render(&world);
    camera.output.export(camera.samples);


// }





    // let red_diffuse = Material::Diffuse { color: () }

    // let grey_diffuse = Material::Diffuse { color: Color::new(0.5,0.5,0.5) };
    // let color_diffuse = Material::Diffuse { color: Color::new(0.7,0.3,0.3) };
    // let white_metal: Material = Material::Metal { color: Color::new(1.0,1.0,1.0) , roughness: 0.1};
    // let white_diffuse: Material = Material::Diffuse { color: Color::new(0.87,0.9,0.95)};
    // let shiny_black_metal = Material::Metal { color: Color::new(0.4,0.2,0.3), roughness: 0.2 };
    // let glass: Material = Material::Dielectric { ior: 1.5 };
    // let red_plastic = Material::Dielectric { ior: 1.55 };

    // let blue_diffuse = Material::Diffuse { color: Color::new(0.0,0.0,1.0) };
    // world.add(Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0,1.0, 0.0), orange_diffuse));
    // world.add(Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0,-1.0, 0.0), grey_diffuse));


    // world.add(Plane::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0,0.0, 0.0), white_metal));
    // world.add(Plane::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::new(-1.1,0.0, 0.0), white_metal));


    // world.add(Sphere::new(Vec3::new(1.0, 0.0, 2.0), 0.5, white_metal));
    // world.add(Sphere::new(Vec3::new(-1.0, 0.0, 2.0), 0.5, gold_metal));
    // world.add(Sphere::new(Vec3::new(0.5, -0.5, 2.0), 0.5, gold_metal));

    // world.add(Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5, glass));
    // world.add(Sphere::new(Vec3::new(-0.6, 0.0, -3.0), 0.5, shiny_black_metal));

    
    // world.add(Sphere::new(Vec3::new(0.5, 0.0, 2.0), -0.22, glass));

    // world.add(Sphere::new(Vec3::new(-0.5, 0.00, 2.0), 0.5, white_metal));
    // world.add(Sphere::new(Vec3::new(-0.05, 0.55, 1.75), 0.05, shiny_black_metal));

    // world.add(Sphere::new(Vec3::new(-0.50, 0.50, 2.0), 0.25, gold_metal));


    // world.add(Sphere::new(Vec3::new(0.0, -100.5, 1.0), 100.0, grey_diffuse));
    // world.add(Sphere::new(Vec3::new(-0.5, -0.5, 2.0), 0.5, grey_diffuse));
    // world.add(Sphere::new(Vec3::new(0.5, 0.5, 1.5), 0.5));

    // let mut camera: Camera = Camera::new();
    // camera.image_width = 1920/4;
    // camera.image_height = 1080/4;
    // camera.samples = 100;
    // camera.ray_depth = 50;
    // camera.fov = 120.0;
    // camera.look_from = Point3::new(-0.0, 0.0, 0.25);
    // camera.look_at = Vec3::new(0.0, 0.0, -2.0);

    // camera.render(&world);
    
   
}

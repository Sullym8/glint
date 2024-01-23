use std::time::Instant;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tobj::Material;

use crate::{hittable::{HittableVec, Hittable}, vec3::{Point3, Vec3, WHITE}, ray::Ray, color::Color, util::gen_random, image::Image, hittable2::Primitive, bvh::BVHNode};

pub struct Camera {
    pub image_width: i32,
    pub image_height: i32,
    camera_center: Point3,
    upper_left_pixel: Point3,
    del_h: Vec3,
    del_w: Vec3,
    pub samples: i32,
    pub ray_depth: i32,
    pub fov: f64,
    pub look_at: Point3,
    pub look_from: Point3,
    v_up: Vec3,
    pub output: Image,
}

impl Camera {
    pub fn new() -> Self {
        let def = Vec3::default();
        Camera {
            image_width: 4,
            image_height: 4,
            camera_center: def,
            upper_left_pixel: def,
            del_h: def,
            del_w: def,
            samples: 1,
            ray_depth: 10,
            fov: 60.0,
            look_at: Point3::new(0.0, 0.0, -1.0),
            look_from: Point3::default(),
            v_up: Vec3::new(0.0, 1.0, 0.0),
            output: Image::new(4, 4, vec![])
        }
    }


    pub fn render(&mut self, world: &HittableVec, bvh: &BVHNode, primitives: &Vec<Primitive>) {
        self.init();

        let CHUNK_SIZE:usize = 1;

        //PPM header
        // println!("P3\n{} {}\n255", self.image_width, self.image_height);
        


        // let _ = (0..self.image_height * self.image_width).into_par_iter().map(|pixel| {
                // let mut color_accumulate = Color::default();
                // let i = pixel / self.image_width;
                // let j = pixel % self.image_width;

                // for _ in 0..self.samples {
                //     let ray: Ray = self.get_sample_ray(i, j);
                //     color_accumulate = color_accumulate + self.ray_color(&ray, world, self.ray_depth);
                //     color_accumulate.write_color(self.samples);
                // }
        // });

        let start = Instant::now();

        let mut pixels = vec![Color::default(); (self.image_height * self.image_width) as usize];

        // let chunks: Vec<(usize, &mut [Vec3])> = pixels.chunks_mut((1) as usize).enumerate().collect();
        // let _ = chunks.into_par_iter().for_each(|(x, chunk)| {
        //     // for x in 0..self.image_width {
        //         let mut color_accumulate = Color::default();
        //         for _ in 0..self.samples {
        //             let ray: Ray = self.get_sample_ray(x as i32 / self.image_width, x as i32 % self.image_width);
        //             // println!("{:?}", ray);
        //             color_accumulate = color_accumulate + self.ray_color(&ray, world, self.ray_depth);
        //         }
        //         chunk[0] = color_accumulate;
        //     // }
        // });

        
        let rows: Vec<(usize, &mut [Vec3])> = pixels.chunks_mut((self.image_width) as usize).enumerate().collect();
        let _ = rows.into_par_iter().for_each(|(y, row)| {
            for x in 0..self.image_width {
                let mut color_accumulate = Color::default();
                let mut color_accumulate2 = Color::default();

                for _ in 0..self.samples {
                    let ray: Ray = self.get_sample_ray(y as i32, x);
                    // println!("{y} {x} {:?}", ray);
                    // color_accumulate = color_accumulate + self.ray_color(&ray, world, self.ray_depth);
                    color_accumulate2 = color_accumulate2 + self.ray_color2(&ray, bvh, primitives, self.ray_depth);

                }
                row[x as usize] = color_accumulate2;
            }
        });

        self.output = Image::new(self.image_width as u32, self.image_height as u32, pixels);

        println!("Time elapsed: {}", start.elapsed().as_millis())


        // let _ = (0..self.image_height * self.image_width).into_par_iter().map(|pixel| {
        //     let mut color_accumulate = Color::default();
        //     let i = pixel / self.image_width;
        //     let j = pixel % self.image_width;

        //     for _ in 0..self.samples {
        //         let ray: Ray = self.get_sample_ray(i, j);
        //         color_accumulate = color_accumulate + self.ray_color(&ray, world, self.ray_depth);
        //         // color_accumulate.write_color(self.samples);
        //     }

        //     // self.update_pixel((i * self.image_height + j) as usize, color_accumulate);

        //     pixels[(i * self.image_height + j) as usize] = color_accumulate;
        // });

        //scanning across image and firing rays
        // for i in 0..self.image_height{
        //     eprintln!("Progress: {}", i);
        //     for j in 0..self.image_width {
        //         let mut color_accumulate = Color::default();
                
        //         for _ in 0..self.samples {
        //             let ray: Ray = self.get_sample_ray(i, j);
        //             color_accumulate = color_accumulate + self.ray_color(&ray, world, self.ray_depth);
        //         }



        //         // //finding ray using pixel offset
        //         // let pixel_vec = self.upper_left_pixel + (i as f64 * self.del_h) + (j as f64 * self.del_w);

        //         // //defining ray to be shot
        //         // let ray : Ray = Ray::new(self.camera_center, pixel_vec - self.camera_center);

        //         // //where the shooting happens -> color of the pixel
        //         // let color: Color = self.ray_color(&ray, &world);


        //         // color_accumulate.write_color(self.samples);
        //     }
        // }
        // eprintln!("Done");
    }

    fn get_sample_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_vec = self.upper_left_pixel + (i as f64 * self.del_h) + (j as f64 * self.del_w);
        let pixel_sample = pixel_vec + self.del_h * (gen_random() - 0.5) + self.del_w * (gen_random() - 0.5);

        Ray::new(self.camera_center, pixel_sample - self.camera_center)
    }

    fn init(&mut self) {

        //internal viewport used to translate between image pixels and ray.
        //Vieport is a rectangular box with the image aspect ratio placed 1 unit away from the origin
        let focal_length = (self.look_from-self.look_at).length();
        let h = f64::tan(f64::to_radians(self.fov * 0.5));

        let viewport_width: f64 = 2.0 * focal_length * h;
        let viewport_height: f64 = viewport_width / (self.image_width as f64/ self.image_height as f64);


        let z = (self.look_from - self.look_at).unit();
        let x = Vec3::cross(self.v_up, z).unit();
        let y = Vec3::cross(z, x).unit();

        self.camera_center = self.look_from;


        //camera is centered at the origin
        // self.camera_center = Point3::new(0.0, 0.0, 0.0);
        //v_w is the vector correspoinding to the length of the viewport
        let v_w: Vec3 = viewport_width * x;
        //v_h is the vector corresponding to the height of the viewport
        let v_h: Vec3 = -viewport_height * y;

        //delta vectors are found by dividing the viewport dimension vectors by the image dimensions (in pixels)
        //Essentially, this allows the raytracer to 'scan' across the viewport, firing rays required for each pixel
        self.del_w = v_w/(self.image_width as f64);
        self.del_h = v_h/(self.image_height as f64);

        //Scanning accross the viewport begins from the top left.
        let camera_upper_left= self.camera_center - (focal_length * z) - v_w/2.0 - v_h/2.0;
        self.upper_left_pixel = camera_upper_left + self.del_w/2.0 + self.del_h/2.0;

        // self.output = Image::new(self.image_height as u32, self.image_width as u32);

        eprintln!("Viewport: {}x{}", viewport_width, viewport_height);
    }

    fn ray_color(&self,r: &Ray, world: &HittableVec, curr_depth: i32) -> Color {

        if curr_depth <= 0 {
            return Color::default();
        } 

        // runs a ray trace to find the closest intersection for a given ray. Returns a bool and the Record of the intersection
        let res = world.ray_hit(r, 0.001, f64::INFINITY);
        // eprintln!("{:?}", res);

        match res {            
            Some(x) => {
                let res = x.material.scatter(r, &x);
                match res {
                    Some((color, scattered_ray)) => {
                        return color * self.ray_color(&scattered_ray, world, curr_depth - 1)
                    }
                    None => {
                        x.material.emit()
                    }
                }
            },
            None => {
                // return Color::default();
                let unit: Vec3 = r.direction().unit();
                let a = (unit.y() + 1.0) * 0.5;
                //lerp between Blue and White to create sky
                (1.0 - a) * Color::new(1.0,1.0,1.0) + a * Color::new(0.5, 0.7,  1.0)

            }
        }
        // Only if an intersection is identified, calculate the color of the object
        // if intersect {
        //     let scatter: bool;
        //     let color: Color;
        //     let scattered_ray: Ray;

        //     (scatter, color, scattered_ray) = intersection_record.material.scatter(r, &intersection_record);

        //     if scatter {
        //         return color * self.ray_color(&scattered_ray, world, curr_depth - 1);
        //     } else {
        //         return color;
        //     }

            // return Color::default();


            // let record:Record = res.1;
            // let random_dir = Vec3::vec_in_unit_hemisphere(record.normal) + record.normal;
            // let ray_bounce = Ray::new(record.point, random_dir);
            // eprintln!("About to fire bounce");

            // Color based on the normal vector at the point of intersection.
            // [-1,1] -> [0,1] transformation for x,y,z coordinates
            // return (Color::new(record.normal.x(), record.normal.y(), record.normal.z()) + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
        // }
    
        //Background color gradient, uses the y component of the ray to create a vertical gradient
        
    }

    fn ray_color2(&self,r: &Ray, bvh: &BVHNode, primitives: &Vec<Primitive>, curr_depth: i32) -> Color {

        if curr_depth <= 0 {
            return Color::default();
        } 

        // runs a ray trace to find the closest intersection for a given ray. Returns a bool and the Record of the intersection
        let res = bvh.ray_hit(primitives, r, 0.001, f64::INFINITY);
        // .ray_hit(r, 0.001, f64::INFINITY);
        // eprintln!("{:?}", res);

        match res {            
            Some(x) => {
                let res = x.material.scatter(r, &x);
                match res {
                    Some((color, scattered_ray)) => {
                        return color * self.ray_color2(&scattered_ray, bvh, primitives, curr_depth - 1)
                    }
                    None => {
                        x.material.emit()
                    }
                }
            },
            None => {
                // return Color::default();
                let unit: Vec3 = r.direction().unit();
                let a = (unit.y() + 1.0) * 0.5;
                //lerp between Blue and White to create sky
                (1.0 - a) * Color::new(1.0,1.0,1.0) + a * Color::new(0.5, 0.7,  1.0)

            }
        }
        // Only if an intersection is identified, calculate the color of the object
        // if intersect {
        //     let scatter: bool;
        //     let color: Color;
        //     let scattered_ray: Ray;

        //     (scatter, color, scattered_ray) = intersection_record.material.scatter(r, &intersection_record);

        //     if scatter {
        //         return color * self.ray_color(&scattered_ray, world, curr_depth - 1);
        //     } else {
        //         return color;
        //     }

            // return Color::default();


            // let record:Record = res.1;
            // let random_dir = Vec3::vec_in_unit_hemisphere(record.normal) + record.normal;
            // let ray_bounce = Ray::new(record.point, random_dir);
            // eprintln!("About to fire bounce");

            // Color based on the normal vector at the point of intersection.
            // [-1,1] -> [0,1] transformation for x,y,z coordinates
            // return (Color::new(record.normal.x(), record.normal.y(), record.normal.z()) + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
        // }
    
        //Background color gradient, uses the y component of the ray to create a vertical gradient
        
    }
}
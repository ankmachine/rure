extern crate image;
extern crate rand;

use image::RgbImage;

//internal
mod helper;
mod core;
mod hitable;

use helper::vec3::{Vec3, unit_vector};
use crate::core::ray::Ray;
use hitable::*;
use crate::core::camera::Camera;

fn main() {
	let nx = 600;
	let ny = 300;
	let ns = 100; // number of samples

	let mut img = RgbImage::new(nx, ny);
	println!("P3");
	println!("{} {}",nx,ny);
	println!("255");
	
	let camera = Camera {
	lower_left_corner : Vec3::new(-2.0, -1.0, -1.0),
	horizontal : Vec3::new(4.0, 0.0, 0.0),
	vertical : Vec3::new(0.0, 2.0, 0.0),
	origin : Vec3::new(0.0, 0.0, 0.0),
	};
	
	let spheres = vec![
     Sphere { center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 },
     Sphere { center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0 },
   ];
   let world: Vec<Box<Hitable>> = spheres.into_iter().map(|s| Box::new(s) as Box<Hitable>).collect();
	
	for j in (0..ny).rev(){
		for i in 0..nx{
			// let col = Vec3::new(i as f32/nx as f32, j as f32/ny as f32, 0.2);
			// let u = i as f32/nx as f32;
			// let v = j as f32/ny as f32;
			// fix by giving proper pixel value here
			// i and j should start from 0,0.
			let mut col = Vec3::new(0.0, 0.0, 0.0);
			let pixel = &mut img[(i, (ny-1) - j)];
			
			for _notused in 0..ns{
				let u = ((i as f32) + rand::random::<f32>()) / (nx as f32);
         			let v = ((j as f32) + rand::random::<f32>()) / (ny as f32);
				let r = camera.get_ray(u, v);
				col += color(r, &world);
			}
			col /= ns as f32;
			col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
			let ir = (255.99*col[0]) as i32;
			let ig = (255.99*col[1]) as i32;
			let ib = (255.99*col[2]) as i32;
			pixel[0] = ir as u8;
			pixel[1] = ig as u8;
			pixel[2] = ib as u8;
		}
	}
	img.save("solved_shadow_acne_issue.png").expect("Failed to save output image");
}

fn color(r:Ray, world: &Hitable)-> Vec3{
let hit = world.hit(&r, 0.001, std::f32::MAX);

   match hit {
     Some(rec) => {
       let target = rec.p + rec.normal + random_in_unit_sphere();
       return 0.5 * color(Ray::new(rec.p, target - rec.p), world);
     },
     None => {
       let unit_direction = unit_vector(r.direction);
       let t = 0.5 * (unit_direction.y() + 1.0);
       return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
     }
   }
}

fn random_in_unit_sphere()->Vec3{
	loop {
     let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - Vec3::new(1.0, 1.0, 1.0);
     if p.squared_length() <= 1.0 {
       return p;
     }
   }
}

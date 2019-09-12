extern crate image;
use image::RgbImage;
mod helper;
mod core;

use helper::vec3::{Vec3, unit_vector};
use self::core::ray::Ray;
fn main() {
	let nx = 200;
	let ny = 100;
	let mut img = RgbImage::new(nx, ny);
	println!("P3");
	println!("{} {}",nx,ny);
	println!("255");
	
	let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
	let horizontal = Vec3::new(4.0, 0.0, 0.0);
	let vertical = Vec3::new(0.0, 2.0, 0.0);
	let origin = Vec3::new(0.0, 0.0, 0.0);
	
	
	for j in (0..ny).rev(){
		for i in 0..nx{
			// let col = Vec3::new(i as f32/nx as f32, j as f32/ny as f32, 0.2);
			let u = i as f32/nx as f32;
			let v = j as f32/ny as f32;
			// fix by giving proper pixel value here
			// i and j should start from 0,0.
			let pixel = &mut img[(i, (ny-1) - j)];
			let r = Ray::new(origin, lower_left_corner+ u*horizontal + v*vertical);
			let col = color(r);
			let ir = (255.99*col[0]) as i32;
			let ig = (255.99*col[1]) as i32;
			let ib = (255.99*col[2]) as i32;
			pixel[0] = ir as u8;
			pixel[1] = ig as u8;
			pixel[2] = ib as u8;
		}
	}
	img.save("output.png").expect("Failed to save output image");
}

fn color(r:Ray)-> Vec3{
	let unit_direction = unit_vector(r.direction);
	let t = 0.5*(unit_direction.y() + 1.0);
	(1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

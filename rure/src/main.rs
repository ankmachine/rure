extern crate image;
use image::RgbImage;
mod helper;

use helper::vec3::Vec3;
fn main() {
	let nx = 200;
	let ny = 100;
	let mut img = RgbImage::new(nx, ny);
	println!("P3");
	println!("{} {}",nx,ny);
	println!("255");
	for j in (0..ny).rev(){
		for i in 0..nx{
			let col = Vec3::new(i as f32/nx as f32, j as f32/ny as f32, 0.2);
			// fix by giving proper pixel value here
			// i and j should start from 0,0.
			let pixel = &mut img[(i, (ny-1) - j)];
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

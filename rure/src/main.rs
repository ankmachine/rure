extern crate image;
use image::RgbImage;
fn main() {
	let nx = 200;
	let ny = 100;
	let mut img = RgbImage::new(nx, ny);
	println!("P3");
	println!("{} {}",nx,ny);
	println!("255");
	for j in (0..ny).rev(){
		for i in 0..nx{
			let r = i as f32/nx as f32;
			let g = j as f32/ny as f32;
			let b = 0.2;
			let pixel = &mut img[(i, j)];
			let ir = (255.99*r) as i32;
			let ig = (255.99*g) as i32;
			let ib = (255.99*b) as i32;
			println!("{} {} {}", ir, ig, ib);
			pixel[0] = ir as u8;
			pixel[1] = ig as u8;
			pixel[2] = ib as u8;
		}
	}
	img.save("output.png").expect("Failed to save output image");
}

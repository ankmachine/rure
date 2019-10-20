use crate::helper::vec3::Vec3;
use super::ray::Ray;


pub struct Camera {
	pub lower_left_corner: Vec3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
	pub origin: Vec3
}

impl Camera{
	pub fn get_ray(&self, u: f32, v: f32)-> Ray{
		Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
	}
}

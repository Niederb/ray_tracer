use super::Vec3;
use super::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3, 
    horizontal: Vec3,
    vertical: Vec3,

}

impl Camera{
    pub fn new() -> Camera {
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u:f64, v:f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}
use super::Vec3;
use super::Ray;

use std::f64::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3, 
    horizontal: Vec3,
    vertical: Vec3,

}

impl Camera{
    pub fn new(lookfrom:Vec3, lookat:Vec3, vup:Vec3, vfov:f64, aspect:f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);
        let lower_left_corner = lookfrom - u * half_width - v * half_width - w; 
        let horizontal = u * 2.0 * half_width;
        let vertical = v * 2.0 * half_height;
        Camera { origin: lookfrom, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u:f64, v:f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}
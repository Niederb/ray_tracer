extern crate rand;

use super::Material;
use super::Ray;
use super::Vec3;
use super::HitRecord;

#[derive(Debug)]
pub struct Dielectric {
    refraction_index:f64,
}

impl Dielectric {
    pub fn new(refraction_index:f64) -> Dielectric {
        Dielectric{ refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray:&Ray, hit_record:HitRecord) -> (Vec3, Ray) {
        let scattered = Ray::new(Vec3::origin(), Vec3::origin());
        let attenuation = Vec3::origin();
        (attenuation, scattered)
    }
}
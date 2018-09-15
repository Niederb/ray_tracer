extern crate rand;

use super::Material;
use super::Ray;
use super::Vec3;
use super::HitRecord;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if v.length() < 1.0 {
            return v;
        }
    }
}

#[derive(Debug)]
pub struct Lambertian {
    albedo:Vec3,
}

impl Lambertian {
    pub fn new(albedo:Vec3) -> Lambertian {
        Lambertian{ albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray:&Ray, hit_record:HitRecord) -> (Vec3, Ray) {
        let hit_point = *hit_record.hit_point();
        let normal = *hit_record.normal();
        let random_direction = random_in_unit_sphere();
        let target = hit_point + normal + random_direction;
        let scattered = Ray::new(hit_point, target - hit_point);
        let attenuation = self.albedo;
        (attenuation, scattered)
    }
}
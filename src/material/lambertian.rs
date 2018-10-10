extern crate rand;

use super::Material;
use geom::Ray;
use geom::Vec3;
use geom::HitRecord;
use geom::random_in_unit_sphere;

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
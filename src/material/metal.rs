

use crate::geom::random_in_unit_sphere;
use crate::geom::HitRecord;
use crate::geom::Ray;
use crate::geom::Vec3;
use crate::Material;

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * Vec3::dot(v, n) * 2.0
}

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Metal {
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> (Vec3, Ray) {
        let reflected = reflect(&ray.direction().unit_vector(), &hit_record.normal());
        let scattered = Ray::new(
            *hit_record.hit_point(),
            reflected + random_in_unit_sphere() * self.fuzziness,
        );
        let attenuation = self.albedo;
        (attenuation, scattered)
    }
}

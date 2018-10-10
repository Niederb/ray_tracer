extern crate rand;

use Material;
use geom::Ray;
use geom::Vec3;
use geom::HitRecord;
use reflect;

fn refract(v:&Vec3, n:&Vec3, ni_over_nt:f64) -> (bool, Vec3) {
    let uv = v.unit_vector();
    let dt = Vec3::dot(&uv, &n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt* (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted =  (uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt();
        return (true, refracted);
    } else {
        return (false, Vec3::origin());
    }
} 

fn schlick(cosine:f64, refraction_index:f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0_sq = r0.powi(2);
    r0_sq + (1.0 - r0_sq) * (1.0 - cosine).powi(5)
}

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
        let reflected = reflect(ray.direction(), hit_record.normal());
        let attenuation= Vec3::new(1.0, 1.0, 1.0);
        let det = Vec3::dot(ray.direction(), hit_record.normal());
        let (outward_normal, ni_over_nt) = if det > 0.0 {
            (*hit_record.normal() * -1.0, self.refraction_index) //TODO implement negative operator
        } else {
            (*hit_record.normal(), 1.0 / self.refraction_index)
        };
        let cosine = if det > 0.0 {
            self.refraction_index * Vec3::dot(ray.direction(), hit_record.normal()) / ray.direction().length()
        } else {
             (Vec3::dot(ray.direction(), hit_record.normal()) / ray.direction().length()) * -1.0
        };
        let (refracted, refracted_direction) = refract(ray.direction(), &outward_normal, ni_over_nt);
        let reflect_prob = if refracted {
            schlick(cosine, self.refraction_index)
        } else {
            1.0
        };
        let scattered = if rand::random::<f64>() < reflect_prob {
            Ray::new(*hit_record.hit_point(), reflected)
        } else {
            Ray::new(*hit_record.hit_point(), refracted_direction)
        };
        (attenuation, scattered)
    }
}
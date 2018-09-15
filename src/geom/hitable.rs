use super::Vec3;
use super::Ray;
use super::Material;
use super::Lambertian;

use std::rc::Rc;

pub struct HitRecord {
    is_hit:bool,
    t:f64,
    p:Vec3,
    normal:Vec3,
    material:Rc<Material>,
}

impl HitRecord {
    pub fn new(t:f64, p:Vec3, normal:Vec3, material:Rc<Material>) -> HitRecord {
        HitRecord { is_hit:true, t, p, normal, material }
    }

    pub fn no_hit() -> HitRecord {
        let material = Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0)));
        HitRecord { is_hit:false, t:0.0, p:Vec3::origin(), normal:Vec3::origin(), material }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn is_hit(&self) -> bool {
        self.is_hit
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn hit_point(&self) -> &Vec3 {
        &self.p
    }

    pub fn material(&self) -> Rc<Material> {
        Rc::clone(&self.material)
    }
}

pub trait Hitable {
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> HitRecord;
}

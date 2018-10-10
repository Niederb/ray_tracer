use Vec3;
use Ray;
use material::Material;

use std::rc::Rc;

pub struct HitRecord {
    t:f64,
    p:Vec3,
    normal:Vec3,
    material:Rc<Material>,
}

impl HitRecord {

    pub fn new(t:f64, p:Vec3, normal:Vec3, material:Rc<Material>) -> HitRecord {
        HitRecord { t, p, normal, material }
    }

    pub fn t(&self) -> f64 {
        self.t
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
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord>;
}

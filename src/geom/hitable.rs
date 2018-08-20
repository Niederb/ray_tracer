use super::Vec3;
use super::Ray;
use std;

pub struct HitRecord {
    is_hit:bool,
    t:f64,
    p:Vec3,
    normal:Vec3
}

impl HitRecord {
    pub fn new(t:f64, p:Vec3, normal:Vec3) -> HitRecord {
        HitRecord { is_hit:true, t:t, p:p, normal:normal }
    }

    pub fn no_hit() -> HitRecord {
        HitRecord { is_hit:false, t:0.0, p:Vec3::origin(), normal:Vec3::origin() }
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
}

pub trait Hitable {
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> HitRecord;
}

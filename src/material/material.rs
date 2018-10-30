use crate::geom::HitRecord;
use crate::geom::Ray;
use crate::geom::Vec3;

use std::fmt;

pub trait Material: fmt::Debug {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> (Vec3, Ray);
}

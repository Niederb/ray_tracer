use geom::HitRecord;
use geom::Ray;
use geom::Vec3;

use std::fmt;

pub trait Material: fmt::Debug {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> (Vec3, Ray);
}

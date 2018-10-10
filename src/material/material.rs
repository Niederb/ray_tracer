use geom::HitRecord;
use geom::Vec3;
use geom::Ray;

use std::fmt;

pub trait Material: fmt::Debug {
    fn scatter(&self, ray:&Ray, hit_record:HitRecord) -> (Vec3, Ray);
}
use super::HitRecord;
use super::Vec3;
use super::Ray;

use std::fmt;

pub trait Material: fmt::Debug {
    fn scatter(&self, ray:&Ray, hit_record:HitRecord) -> (Vec3, Ray);
}
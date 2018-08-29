use super::Vec3;
use super::Ray;
use super::Hitable;
use super::HitRecord;
use std;

#[derive(PartialEq)]
pub struct HitableList<T: Hitable + std::fmt::Debug> {
    l:Vec<Box<T>>
}

impl<T: Hitable + std::fmt::Debug> HitableList<T> {
    pub fn new() -> HitableList<T> {
        HitableList::<T> { l:Vec::new() }
    }

    pub fn add(&mut self, h:T) {
        self.l.push(Box::new(h))
    }
}

impl<T: Hitable + std::fmt::Debug> Hitable for HitableList<T> {
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> HitRecord {
        if self.l.len() == 0 {
            return HitRecord::no_hit()
        }
        let mut closest_record = self.l[0].hit(r, t_min, t_max);
        for h in &self.l {
            let current_record = h.hit(r, t_min, t_max);
            if current_record.is_hit() && (current_record.t() < closest_record.t() || !closest_record.is_hit()) {
                closest_record = current_record;
            }
        }
        closest_record
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;
    use super::Ray;
    #[test]
    fn point_at_parameter_test() {
    }
}
use Vec3;
use Ray;
use Hitable;
use HitRecord;
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
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        if self.l.len() == 0 {
            return None;
        }
        let mut closest_record: Option<HitRecord> = None;
        let mut min_distance = std::f64::MAX;
        
        for h in &self.l {
            let current_record = h.hit(r, t_min, t_max);
            match current_record {
                // The division was valid
                Some(hit_record) => {
                    if hit_record.t() < min_distance || closest_record.is_none() {
                        min_distance = hit_record.t();
                        closest_record = Some(hit_record);
                        
                    }
                }
                None    => (),
            }
            
        }
        closest_record
    }
}

#[cfg(test)]
mod test {
    use Vec3;
    use Ray;
    #[test]
    fn point_at_parameter_test() {
    }
}
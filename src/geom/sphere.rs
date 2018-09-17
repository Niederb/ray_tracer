use super::Vec3;
use super::Ray;
use super::Hitable;
use super::HitRecord;
use super::Material;

use std::rc::Rc;

#[derive(Debug)]
pub struct Sphere {
    center:Vec3,
    radius:f64, 
    material: Rc<Material>,
}

impl Sphere {
    pub fn new(center:Vec3, radius:f64, material:Rc<Material>) -> Sphere {
        Sphere { center, radius, material}
    }
}

impl Hitable for Sphere {
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        //let a = Vec3::dot(r.direction(), r.direction());
        let a = r.direction().squared_length();
        let b = Vec3::dot(&oc, r.direction());
        //let c = Vec3::dot(&oc, &oc) - self.radius.powi(2);
        let c = oc.squared_length() - self.radius.powi(2);
        let discriminant = b.powi(2) - a*c;
        if discriminant > 0.0 {
            let temp = (b.powi(2) - a*c).sqrt();
            let solution1 = (-b - temp) / a;
            if solution1 < t_max && solution1 > t_min {
                let hit_point = r.point_at_parameter(solution1);
                let normal = (hit_point - self.center) / self.radius;
                return Some(HitRecord::new(solution1, hit_point, normal, Rc::clone(&self.material)));
            }
            let solution2 = (-b + temp) / a;
            if solution2 < t_max && solution2 > t_min {
                let hit_point = r.point_at_parameter(solution2);
                let normal = (hit_point - self.center) / self.radius;
                return Some(HitRecord::new(solution2, hit_point, normal, Rc::clone(&self.material)));
            }
        }
        None
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
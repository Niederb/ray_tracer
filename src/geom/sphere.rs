use super::Vec3;
use super::Ray;
use super::Hitable;
use super::HitRecord;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    center:Vec3,
    radius:f64
}

impl Sphere {
    pub fn new(center:Vec3, radius:f64) -> Sphere {
        Sphere { center:center, radius:radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> HitRecord {
        let oc = *r.origin() - self.center;
        let a = Vec3::dot(r.direction(), r.direction());
        let b = Vec3::dot(&oc, r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a*c;
        if discriminant > 0.0 {
            let temp = (b.powi(2) - a*c).sqrt();
            let solution1 = (-b - temp) / a;
            if solution1 < t_max && solution1 > t_min {
                let hit_point = r.point_at_parameter(solution1);
                let normal = (hit_point - self.center) / self.radius;
                return HitRecord::new(solution1, hit_point, normal);
            }
            let solution2 = (-b + temp) / a;
            if solution2 < t_max && solution2 > t_min {
                let hit_point = r.point_at_parameter(solution2);
                let normal = (hit_point - self.center) / self.radius;
                return HitRecord::new(solution2, hit_point, normal);
            }
        }
        HitRecord::no_hit()
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
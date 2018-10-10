use Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin:Vec3, direction:Vec3) -> Ray {
        Ray { origin:origin, direction:direction }
    }

    pub fn point_at_parameter(&self, t:f64) -> Vec3 {
        self.origin + self.direction*t
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}

#[cfg(test)]
mod test {
    use Vec3;
    use Ray;
    #[test]
    fn point_at_parameter_test() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(-1.0, -4.0, 5.0);
        let ray = Ray::new(origin, direction);
        let result = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(result, ray.point_at_parameter(0.0));
        let result2 = Vec3::new(0.0, -2.0, 8.0);
        assert_eq!(result2, ray.point_at_parameter(1.0));
    }
}
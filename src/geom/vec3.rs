use std::ops::Add;
use std::ops::Mul;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3{ x:x, y:y, z:z}
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3{ x: self.x + rhs.x, y:self.y + rhs.y, z:self.z + rhs.z}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3{ x: t*self.x, y:t*self.y, z:t*self.z}
    }
}

#[test]
fn squared_length_test() {
    let my_vec = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(14.0, my_vec.squared_length());
}

#[test]
fn length_test() {
    let my_vec = Vec3::new(0.0, 4.0, 3.0);
    assert_eq!(5.0, my_vec.length());
}

#[test]
fn add_test() {
    let my_vec = Vec3::new(0.0, 4.0, 3.0);
    let my_vec2 = Vec3::new(-1.0, -4.0, 5.0);
    let result = Vec3::new(-1.0, 0.0, 8.0);
    assert_eq!(result, my_vec + my_vec2);
}

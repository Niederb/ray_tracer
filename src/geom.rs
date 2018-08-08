use std::ops::Add;

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
        Vec3{ x: self.x + rhs.x, y:0.0, z:0.0}
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
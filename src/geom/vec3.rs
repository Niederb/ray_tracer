use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn dot(left: &Vec3, right: &Vec3) -> f64 {
        left.x() * right.x() + left.y() * right.y() + left.z() * right.z()
    }

    pub fn cross(left: &Vec3, right: &Vec3) -> Vec3 {
        let x = left.y() * right.z() - left.z() * right.y();
        let y = -left.x() * right.z() - left.z() * right.x();
        let z = left.x() * right.y() - left.y() * right.x();
        Vec3::new(x, y, z)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * v.x(),
            y: self.y * v.y(),
            z: self.z * v.z(),
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: t * self.x,
            y: t * self.y,
            z: t * self.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Vec3;

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
    fn basic_operation_test() {
        let my_vec = Vec3::new(0.0, 4.0, 3.0);
        let my_vec2 = Vec3::new(-1.0, -4.0, 5.0);
        assert_eq!(Vec3::new(-1.0, 0.0, 8.0), my_vec + my_vec2);
        assert_eq!(Vec3::new(1.0, 8.0, -2.0), my_vec - my_vec2);
        assert_eq!(Vec3::new(0.0, -8.0, -6.0), my_vec * -2.0);
        assert_eq!(Vec3::new(0.0, 2.0, 1.5), my_vec / 2.0);
    }

    #[test]
    fn dot_test() {
        let my_vec = Vec3::new(0.0, 4.0, 3.0);
        let my_vec2 = Vec3::new(-1.0, -4.0, 5.0);
        assert_eq!(-1.0, Vec3::dot(&my_vec, &my_vec2));
    }
}

use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    fn squared_length(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3{ x: self.x + rhs.x, y:0.0, z:0.0}
    }
}

fn main() {
    let height = 200;
    let width = 300;

    let mut f2 = File::create("image.txt").expect("Unable to create file");
    write!(f2, "P3\n{} {}\n255\n", width, height).expect("cout not write to file<");

    for y in 0..height {
        for x in 0..width {
            let r = 256 * x / width;
            let g = 256 * y / height;
            let b = 256 * (x + y) / (width + height);
            write!(f2, "{} {} {}\n", r, g, b).expect("could not write to file");
        }
    }
    let my_vec = Vec3{ x:1.0, y:2.0, z:3.0};
    println!("squared length: {}", my_vec.squared_length());
    println!("length: {}", my_vec.length());
}

#[test]
fn squared_length_test() {
    let my_vec = Vec3{ x:1.0, y:2.0, z:3.0};
    assert_eq!(14.0, my_vec.squared_length());
}

#[test]
fn length_test() {
    let my_vec = Vec3{ x:0.0, y:4.0, z:3.0};
    assert_eq!(5.0, my_vec.length());
}
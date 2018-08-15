use std::fs::File;
use std::io::prelude::*;
use geom::*;

mod geom;

fn color(r:&Ray, s:&Sphere) -> Vec3 {
    let hit_record = s.hit(r, 0.0, 100000.0);
    if hit_record.is_hit() {
        let hit_point = hit_record.hit_point();
        let normal = hit_record.normal();
        return Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    }
    let direction = r.direction().unit_vector();
    let t:f64 = 0.5*(direction.y() + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    white*(1.0-t) + blue*t
}

fn main() {
    let height = 100;
    let width = 200;

    let mut f2 = File::create("image.txt").expect("Unable to create file");
    write!(f2, "P3\n{} {}\n255\n", width, height).expect("cout not write to file<");

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal_size = Vec3::new(4.0, 0.0, 0.0);
    let vertical_size = Vec3::new(0.0, 2.0, 0.0);
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / width as f64;
            let v = y as f64 / height as f64;
            
            let r = Ray::new(origin, lower_left_corner + horizontal_size*u + vertical_size*v);
            let color = color(&r, &sphere);
            let r = 255.99 * color.x();
            let g = 255.99 * color.y();
            let b = 255.99 * color.z();
            write!(f2, "{} {} {}\n", r, g, b).expect("could not write to file");
        }
    }
    let my_vec = Vec3::new(1.0, 2.0, 3.0);
    println!("squared length: {}", my_vec.squared_length());
    println!("length: {}", my_vec.length());
}

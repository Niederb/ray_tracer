extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use geom::*;

use std::rc::Rc;

mod geom;

fn color(r:&Ray, h:&Hitable, depth:i32) -> Vec3 {
    let hit_record = h.hit(r, 0.0001, 100000.0);
    if hit_record.is_hit() {
        if depth < 50 {
            let (attenuation, scattered) = hit_record.material().scatter(r, hit_record);
            return attenuation * color(&scattered, h, depth + 1);
        } else {
            return Vec3::origin();
        }
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
    let n_samples = 100;
    let mut f2 = File::create("image.txt").expect("Unable to create file");
    write!(f2, "P3\n{} {}\n255\n", width, height).expect("cout not write to file<");

    let camera = Camera::new();
    let mut hit_list = HitableList::new();
    let material:Rc<Material> = Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)));
    hit_list.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::clone(&material)));
    hit_list.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::clone(&material)));
    for y in (0..height).rev() {
        for x in 0..width {
            let mut total = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..n_samples {
                let u_delta:f64 = rand::random();
                let v_delta:f64 = rand::random();
                let u = (x as f64 + u_delta) / width as f64;
                let v = (y as f64 + v_delta) / height as f64;
                let r = camera.get_ray(u, v);
                let color = color(&r, &hit_list, 1);
                total = total + color;
            }
            total = total / n_samples as f64;
            total = Vec3::new(total.x().sqrt(), total.y().sqrt(), total.z().sqrt());
            let r = 255.99 * total.x();
            let g = 255.99 * total.y();
            let b = 255.99 * total.z();
            write!(f2, "{} {} {}\n", r, g, b).expect("could not write to file");
        }
    }
    let my_vec = Vec3::new(1.0, 2.0, 3.0);
    println!("squared length: {}", my_vec.squared_length());
    println!("length: {}", my_vec.length());
}

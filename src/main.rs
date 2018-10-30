extern crate image;
extern crate rand;

use self::geom::*;
use image::ImageBuffer;
use rand::random;
use std::time::Instant;

use std::rc::Rc;

mod geom;
mod material;

use crate::material::*;

fn color(r: &Ray, h: &Hitable, depth: i32) -> Vec3 {
    let hit_result = h.hit(r, 0.0001, 100_000.0);
    match hit_result {
        // The division was valid
        Some(hit_record) => {
            if depth < 50 {
                let (attenuation, scattered) = hit_record.material().scatter(r, hit_record);
                return attenuation * color(&scattered, h, depth + 1);
            } else {
                return Vec3::origin();
            }
        }
        // The division was invalid
        None => (),
    }
    let direction = r.direction().unit_vector();
    let t: f64 = 0.5 * (direction.y() + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    white * (1.0 - t) + blue * t
}

fn create_camera(width: i32, height: i32) -> Camera {
    let camera_origin = Vec3::new(13.0, 2.0, 3.0);
    let camera_lookat = Vec3::new(0.0, 0.0, 0.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    Camera::new(
        camera_origin,
        camera_lookat,
        camera_up,
        20.0,
        width as f64 / height as f64,
        aperture,
        dist_to_focus,
    )
}

fn main() {
    let height = 100;
    let width = 150;
    let n_samples = 10;
    let camera = create_camera(width, height);

    let mut hit_list = HitableList::new();
    let world_material: Rc<Material> = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let metal: Rc<Material> = Rc::new(Metal::new(Vec3::new(0.3, 0.3, 0.8), 0.0));

    hit_list.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&world_material),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::random();
            let center = Vec3::new(
                a as f64 + random::<f64>() * 0.9,
                0.2,
                b as f64 + random::<f64>() * 0.9,
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Rc<Material> = if choose_mat < 0.8 {
                    Rc::new(Lambertian::new(Vec3::new(
                        random::<f64>() * random::<f64>(),
                        random::<f64>() * random::<f64>(),
                        random::<f64>() * random::<f64>(),
                    )))
                } else if choose_mat < 0.95 {
                    Rc::new(Metal::new(
                        Vec3::new(
                            0.5 * (1.0 + random::<f64>()),
                            0.5 * (1.0 + random::<f64>()),
                            0.5 * (1.0 + random::<f64>()),
                        ),
                        0.5 * random::<f64>(),
                    ))
                } else {
                    Rc::new(Dielectric::new(1.5))
                };
                hit_list.add(Sphere::new(center, 0.2, Rc::clone(&material)));
            }
        }
    }
    hit_list.add(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    hit_list.add(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    ));
    hit_list.add(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    ));
    hit_list.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&metal),
    ));
    let total_time = Instant::now();
    let mut img: image::RgbaImage = ImageBuffer::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let mut total = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..n_samples {
                let u_delta: f64 = rand::random();
                let v_delta: f64 = rand::random();
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
            img.get_pixel_mut(x as u32, height as u32 - 1 - y as u32)
                .data = [r as u8, g as u8, b as u8, 255];
        }
        println!(
            "Lines to go: {} Estimated remaining time: {}",
            y,
            total_time.elapsed().as_secs() as f64 / (height as f64 - y as f64) * y as f64
        );
    }
    img.save("test.png").unwrap();
}

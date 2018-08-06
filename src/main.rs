use std::fs::File;
use std::io::prelude::*;

fn main() {
    let height = 200;
    let width = 300;

    let mut f2 = File::create("image.txt").expect("Unable to create file");
    write!(f2, "P3\n{} {}\n255\n", width, height).expect("cout not write to file");

    for y in 0..height {
        for x in 0..width {
            let r = 256 * x / width;
            let g = 256 * y / height;
            let b = 256 * (x + y) / (width + height);
            write!(f2, "{} {} {}\n", r, g, b).expect("could not write to file");
        }
    }
}

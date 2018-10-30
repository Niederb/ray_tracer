use rand;

use crate::Vec3;

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let v = Vec3::new(rand::random::<f64>(), rand::random::<f64>(), 0.0) * 2.0
            - Vec3::new(1.0, 1.0, 0.0);
        if v.length() < 1.0 {
            return v;
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        ) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);
        if v.length() < 1.0 {
            return v;
        }
    }
}

mod ray;
mod vec3;

use ray::Ray;
use vec3::{Point3, Vec3};

fn main() {
    // 原点から x方向に進むレイ
    let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

    println!("ray.at(0.0) = {:?}", r.at(0.0)); // (0, 0, 0) ← 始点
    println!("ray.at(1.0) = {:?}", r.at(1.0)); // (1, 0, 0)
    println!("ray.at(2.5) = {:?}", r.at(2.5)); // (2.5, 0, 0)

    // ななめに進むレイ
    let r2 = Ray::new(Point3::new(1.0, 1.0, 0.0), Vec3::new(2.0, 3.0, 0.0));
    println!("r2.at(0.5) = {:?}", r2.at(0.5)); // (2, 2.5, 0)
}

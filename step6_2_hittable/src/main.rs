mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use color::write_color;
use ray::Ray;
use vec3::{Color, Point3, Vec3, unit_vector};

use crate::{hittable::Hittable, sphere::Sphere};

fn ray_color(r: &Ray) -> Color {
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;
    let sphere = Sphere::new(sphere_center, sphere_radius);
    if let Some(rec) = sphere.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Vec3::new(1., 1., 1.));
    }

    let unit_direction = unit_vector(r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);

    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);

    (1.0 - a) * white + a * blue
}

fn main() {
    // ===== 画像設定 =====
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // ===== カメラ設定 =====
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point3 = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u: Vec3 = viewport_u / (image_width as f64);
    let pixel_delta_v: Vec3 = viewport_v / (image_height as f64);

    let viewport_upper_left: Point3 =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc: Point3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // ===== PPMヘッダー出力 =====
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    // ===== レンダリング =====
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64) * pixel_delta_u + (j as f64) * pixel_delta_v;

            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }

    eprintln!("\rDone.                          ");
}

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::write_color;
use ray::Ray;
use vec3::{Color, Point3, Vec3, unit_vector};

use crate::{hittable::Hittable, hittable_list::HittableList, sphere::Sphere};

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let new_origin = rec.p; // ヒット点が新しいレイの始点になる
        let new_direction = Vec3::new(
            rec.normal.x + rand::random_range(-1.0..1.0),
            rec.normal.y + rand::random_range(-1.0..1.0),
            rec.normal.z + rand::random_range(-1.0..1.0),
        );
        let new_ray = Ray::new(new_origin, new_direction);
        return 0.5 * ray_color(&new_ray, world, depth - 1);
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

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let camera = Camera::new(camera_center, pixel00_loc, pixel_delta_u, pixel_delta_v);
    let samples_per_pixel = 100;
    let max_depth = 50;

    // ===== レンダリング =====
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);

        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let r = camera.get_ray(i as f64, j as f64);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }

    eprintln!("\rDone.                          ");
}

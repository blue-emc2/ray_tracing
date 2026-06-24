use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    camera_center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        camera_center: Vec3,
        pixel00_loc: Vec3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
    ) -> Self {
        Camera {
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_center = self.pixel00_loc
            + (i + offset.x) * self.pixel_delta_u
            + (j + offset.y) * self.pixel_delta_v;
        let ray_direction = pixel_center - self.camera_center;
        Ray::new(self.camera_center, ray_direction)
    }

    fn sample_square() -> Vec3 {
        let x: f64 = rand::random_range(-0.5..0.5);
        let y: f64 = rand::random_range(-0.5..0.5);
        Vec3::new(x, y, 0.0)
    }
}

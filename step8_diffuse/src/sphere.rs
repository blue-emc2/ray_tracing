use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction); // 方向ベクトルの長さ
        let b = 2.0 * oc.dot(ray.direction); // 「レイが球の中心に向かっているか、遠ざかっているか」を表す量。内積が負なら向かっている、正なら遠ざかっている。
        let c = oc.dot(oc) - self.radius * self.radius; //
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-b - sqrtd) / (2.0 * a);
        if root <= t_min || root >= t_max {
            root = (-b + sqrtd) / (2.0 * a);
            if root <= t_min || root >= t_max {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;
        let mut hit_record = HitRecord {
            p: hit_point,
            normal: outward_normal,
            t: root,
            front_face: true,
        };
        hit_record.set_face_normal(ray, outward_normal);
        Some(hit_record)
    }
}

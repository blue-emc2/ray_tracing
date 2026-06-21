use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,          // ヒット点
    pub normal: Vec3,     // 法線
    pub t: f64,           // レイのパラメータ
    pub front_face: bool, // 表向きヒットか裏向きか
}

impl HitRecord {
    /**
     * ray:
     * outword_normal: 法線ベクトル
     */
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;

        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

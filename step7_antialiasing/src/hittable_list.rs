use crate::{Hittable, hittable::HitRecord, ray::Ray};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut result: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, t_max) {
                closest = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}

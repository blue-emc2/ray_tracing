use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        // ここを自分で書いてみてください
        // 公式: P(t) = origin + t * direction
        self.origin + t * self.direction
        // let px = self.origin.x + t * self.direction.x;
        // let py = self.origin.y + t * self.direction.y;
        // let pz = self.origin.z + t * self.direction.z;
        // Vec3::new(px, py, pz)
    }
}

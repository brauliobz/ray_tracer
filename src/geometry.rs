use glam::DVec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: DVec3,
    pub dir: DVec3,
}

pub trait Intersect {
    /// if it intersects, return the normal
    fn intersect(&self, ray: Ray) -> Option<Ray>;
}

impl Ray {
    pub fn reflect(&self, normal: Ray) -> Ray {
        Ray {
            origin: normal.origin,
            dir: (2.0 * normal.dir.dot(-self.dir) * normal.dir + self.dir).normalize(),
        }
    }
}

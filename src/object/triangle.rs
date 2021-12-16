use glam::DVec3;

use crate::geometry::{Intersect, Ray};

pub struct Triangle {
    pub p1: DVec3,
    pub p2: DVec3,
    pub p3: DVec3,
}

// pub struct TriangleMesh;

impl Intersect for Triangle {
    fn intersect(&self, _ray: Ray) -> Option<crate::geometry::Ray> {
        todo!()
    }
}

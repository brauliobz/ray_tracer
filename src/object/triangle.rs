use crate::geometry::{P3, Intersect};

pub struct Triangle {
    pub p1: P3,
    pub p2: P3,
    pub p3: P3,
}

pub struct TriangleMesh {

}

impl Intersect for Triangle {
    fn intersect(&self, ray: crate::geometry::Vec3) -> Option<crate::geometry::Vec3> {
        todo!()
    }
}

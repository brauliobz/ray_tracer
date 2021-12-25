use std::fmt::Debug;

use glam::DVec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: DVec3,
    pub dir: DVec3,
}

pub trait Intersect: Sync + Debug {
    /// if it intersects, return the normal at the intersection point
    fn intersect(&self, ray: Ray) -> Option<Ray>;
}

impl Ray {
    pub fn new(
        (origin_x, origin_y, origin_z): (f64, f64, f64),
        (dir_x, dir_y, dir_z): (f64, f64, f64),
    ) -> Self {
        Ray {
            origin: (origin_x, origin_y, origin_z).into(),
            dir: (dir_x, dir_y, dir_z).into(),
        }
    }

    pub fn from_to(
        (origin_x, origin_y, origin_z): (f64, f64, f64),
        (to_x, to_y, to_z): (f64, f64, f64),
    ) -> Self {
        Ray {
            origin: (origin_x, origin_y, origin_z).into(),
            dir: (DVec3::new(to_x, to_y, to_z) - DVec3::new(origin_x, origin_y, origin_z))
                .normalize(),
        }
    }

    pub fn reflect(&self, normal: Ray) -> Ray {
        Ray {
            origin: normal.origin,
            dir: (2.0 * normal.dir.dot(-self.dir) * normal.dir + self.dir).normalize(),
        }
    }
}

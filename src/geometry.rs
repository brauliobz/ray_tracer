use std::fmt::Debug;

use glam::DVec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: DVec3,
    pub dir: DVec3,
    /// direction reciprocal
    pub dir_recip: DVec3,
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
        let dir = DVec3::new(dir_x, dir_y, dir_z).normalize();
        Ray {
            origin: (origin_x, origin_y, origin_z).into(),
            dir,
            dir_recip: dir.recip(),
        }
    }

    #[allow(unused)] // used in tests
    pub fn from_to(
        (origin_x, origin_y, origin_z): (f64, f64, f64),
        (to_x, to_y, to_z): (f64, f64, f64),
    ) -> Self {
        let dir =
            (DVec3::new(to_x, to_y, to_z) - DVec3::new(origin_x, origin_y, origin_z)).normalize();
        Ray {
            origin: (origin_x, origin_y, origin_z).into(),
            dir,
            dir_recip: dir.recip(),
        }
    }

    pub fn reflect(&self, normal: Ray) -> Ray {
        let dir = (2.0 * normal.dir.dot(-self.dir) * normal.dir + self.dir).normalize();
        Ray {
            origin: normal.origin,
            dir,
            dir_recip: dir.recip(),
        }
    }
}

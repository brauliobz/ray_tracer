use std::fmt::Debug;

use crate::{Vec3, Float};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    /// direction reciprocal
    pub dir_recip: Vec3,
}

/// Axis-aligned bounding box defined by min and max points
#[derive(Clone, Copy, Debug, Default)]
pub struct AABBox {
    pub min: Vec3,
    pub max: Vec3,
}

pub trait Intersect: Sync + Debug {
    /// if it intersects, return the normal at the intersection point
    fn intersect(&self, ray: Ray) -> Option<Ray>;

    fn bounds(&self) -> AABBox;
}

impl Ray {
    pub fn new(
        (origin_x, origin_y, origin_z): (Float, Float, Float),
        (dir_x, dir_y, dir_z): (Float, Float, Float),
    ) -> Self {
        let dir = Vec3::new(dir_x, dir_y, dir_z).normalize();
        Ray {
            origin: (origin_x, origin_y, origin_z).into(),
            dir,
            dir_recip: dir.recip(),
        }
    }

    #[allow(unused)] // used in tests
    pub fn from_to(
        (origin_x, origin_y, origin_z): (Float, Float, Float),
        (to_x, to_y, to_z): (Float, Float, Float),
    ) -> Self {
        let dir =
            (Vec3::new(to_x, to_y, to_z) - Vec3::new(origin_x, origin_y, origin_z)).normalize();
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

impl AABBox {
    pub fn new(a: Vec3, b: Vec3) -> AABBox {
        AABBox {
            min: Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            max: Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        }
    }

    pub fn octants(&self) -> [AABBox; 8] {
        let middle = (self.min + self.max) / 2.0;

        [
            AABBox::new(self.min, middle),
            AABBox::new(Vec3::new(self.max.x, self.min.y, self.min.z), middle),
            AABBox::new(Vec3::new(self.min.x, self.max.y, self.min.z), middle),
            AABBox::new(Vec3::new(self.min.x, self.min.y, self.max.z), middle),
            AABBox::new(self.max, middle),
            AABBox::new(Vec3::new(self.min.x, self.max.y, self.max.z), middle),
            AABBox::new(Vec3::new(self.max.x, self.min.y, self.max.z), middle),
            AABBox::new(Vec3::new(self.max.x, self.max.y, self.min.z), middle),
        ]
    }

    pub fn intersect_other(&self, other: &Self) -> bool {
        fn interval_intersect(a: (Float, Float), b: (Float, Float)) -> bool {
            !(b.0 > a.1 || a.0 > b.1)
        }

        interval_intersect((self.min.x, self.max.x), (other.min.x, other.max.x))
            && interval_intersect((self.min.y, self.max.y), (other.min.y, other.max.y))
            && interval_intersect((self.min.z, self.max.z), (other.min.z, other.max.z))
    }
}

impl Intersect for AABBox {
    fn intersect(&self, ray: Ray) -> Option<Ray> {
        // slab method

        let mut tmin = Float::NEG_INFINITY;
        let mut tmax = Float::INFINITY;

        if ray.dir.x != 0.0 {
            let tx1 = (self.min.x - ray.origin.x) * ray.dir_recip.x;
            let tx2 = (self.max.x - ray.origin.x) * ray.dir_recip.x;

            tmin = tmin.max(tx1.min(tx2));
            tmax = tmax.min(tx1.max(tx2));
        }

        if ray.dir.y != 0.0 {
            let ty1 = (self.min.y - ray.origin.y) * ray.dir_recip.y;
            let ty2 = (self.max.y - ray.origin.y) * ray.dir_recip.y;

            tmin = tmin.max(ty1.min(ty2));
            tmax = tmax.min(ty1.max(ty2));
        }

        if ray.dir.z != 0.0 {
            let tz1 = (self.min.z - ray.origin.z) * ray.dir_recip.z;
            let tz2 = (self.max.z - ray.origin.z) * ray.dir_recip.z;

            tmin = tmin.max(tz1.min(tz2));
            tmax = tmax.min(tz1.max(tz2));
        }

        // TODO branchless is much slower. Why?

        // let tx1 = (self.min.x - ray.origin.x) * ray.dir_recip.x;
        // let tx2 = (self.max.x - ray.origin.x) * ray.dir_recip.x;

        // let mut tmin = tx1.min(tx2);
        // let mut tmax = tx1.max(tx2);

        // let ty1 = (self.min.y - ray.origin.y) * ray.dir_recip.y;
        // let ty2 = (self.max.y - ray.origin.y) * ray.dir_recip.y;

        // tmin = tmin.min(ty1).min(ty2);
        // tmax = tmax.max(ty1).max(ty2);

        // let tz1 = (self.min.z - ray.origin.z) * ray.dir_recip.z;
        // let tz2 = (self.max.z - ray.origin.z) * ray.dir_recip.z;

        // tmin = tmin.max(tz1).min(tz2);
        // tmax = tmax.max(tz1).max(tz2);

        if tmax >= tmin {
            Some(ray) // TODO use real reflection?
        } else {
            None
        }
    }

    fn bounds(&self) -> AABBox {
        *self
    }
}

#[cfg(test)]
mod test {
    use crate::geometry::AABBox;

    #[test]
    fn bbox_intersect() {
        let a = AABBox::new((0.0, 0.0, 0.0).into(), (5.0, 5.0, 5.0).into());
        let b = AABBox::new((4.0, 4.0, 4.0).into(), (10.0, 10.0, 10.0).into());
        assert!(a.intersect_other(&b));
        assert!(b.intersect_other(&a));
    }

    #[test]
    fn bbox_intersect_inside() {
        let a = AABBox::new((0.0, 0.0, 0.0).into(), (5.0, 5.0, 5.0).into());
        let b = AABBox::new((-4.0, -4.0, -4.0).into(), (10.0, 10.0, 10.0).into());
        assert!(a.intersect_other(&b));
        assert!(b.intersect_other(&a));
    }

    #[test]
    fn bbox_do_not_intersect() {
        let a = AABBox::new((0.0, 0.0, 0.0).into(), (5.0, 5.0, 5.0).into());
        let b = AABBox::new((-4.0, -4.0, -4.0).into(), (-1.0, -1.0, -1.0).into());
        assert!(!a.intersect_other(&b));
        assert!(!b.intersect_other(&a));
    }

    #[test]
    fn bbox_intersect_no_vertices_inside() {
        let a = AABBox::new((0.0, 0.0, 0.0).into(), (1.0, 1.0, 5.0).into());
        let b = AABBox::new((-5.0, -1.0, 2.0).into(), (5.0, 3.0, 3.0).into());
        assert!(a.intersect_other(&b));
        assert!(b.intersect_other(&a));
    }
}

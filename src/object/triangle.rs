use glam::DVec3;
use log::debug;
use nanorand::Rng;

use crate::geometry::{AABBox, Intersect, Ray};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub a: DVec3,
    pub b: DVec3,
    pub c: DVec3,
    normal: DVec3,
}

impl Triangle {
    pub fn new(a: DVec3, b: DVec3, c: DVec3) -> Self {
        Triangle {
            a,
            b,
            c,
            normal: (b - a).cross(c - a).normalize(),
        }
    }

    pub fn from_tuples(a: (f64, f64, f64), b: (f64, f64, f64), c: (f64, f64, f64)) -> Self {
        Triangle::new(a.into(), b.into(), c.into())
    }

    #[allow(unused)] // used in tests
    pub fn normal(&self) -> DVec3 {
        self.normal
    }

    #[allow(unused)] // used in tests
    fn opposite(&self) -> Self {
        Triangle {
            a: self.c,
            b: self.b,
            c: self.a,
            normal: -self.normal,
        }
    }

    #[allow(unused)] // used in tests
    fn almost_equals(&self, other: &Triangle) -> bool {
        self.a.abs_diff_eq(other.a, 1e-10)
            && self.b.abs_diff_eq(other.b, 1e-10)
            && self.c.abs_diff_eq(other.c, 1e-10)
    }
}

impl From<(DVec3, DVec3, DVec3)> for Triangle {
    fn from((a, b, c): (DVec3, DVec3, DVec3)) -> Self {
        Triangle::new(a, b, c)
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: Ray) -> Option<Ray> {
        debug!("checking intersection between {:?} and {:?}", ray, self);

        let n = self.normal;

        if n.dot(ray.dir) > 0.0 {
            debug!("ray comes from behind triangle or ray is parallel to the triangle plane");
            return None;
        }

        let d = n.dot(self.a);
        debug!("d = {}", d);

        let t = (self.a - ray.origin).dot(n) / n.dot(ray.dir);
        debug!("t = {}", t);

        if t <= 0.0 {
            debug!("triangle is behind the ray origin, t = {}", t);
            return None;
        }

        let p = ray.origin + t * ray.dir;
        debug!("intersection point is {}", p);

        let ab = self.b - self.a;
        let bc = self.c - self.b;
        let ca = self.a - self.c;

        debug!("ab = {:?}", ab);
        debug!("bc = {:?}", bc);
        debug!("ca = {:?}", ca);

        debug!("ab x ap = {}", ab.cross(p - self.a).normalize());
        debug!("bc x bp = {}", bc.cross(p - self.b).normalize());
        debug!("ca x cp = {}", ca.cross(p - self.c).normalize());

        debug!("rel ab = {:?}", n.dot(ab.cross(p - self.a).normalize()));
        debug!("rel bc = {:?}", n.dot(bc.cross(p - self.b).normalize()));
        debug!("rel ca = {:?}", n.dot(ca.cross(p - self.c).normalize()));

        let left_of_a_b = n.dot(ab.cross(p - self.a)) >= 0.0;
        let left_of_b_c = n.dot(bc.cross(p - self.b)) >= 0.0;
        let left_of_c_a = n.dot(ca.cross(p - self.c)) >= 0.0;

        debug!("= {} {}", ab.cross(p - self.a).normalize(), n);

        if left_of_a_b && left_of_b_c && left_of_c_a {
            let mut rng = nanorand::tls_rng();
            let rand = DVec3::new(
                rng.generate::<f64>() - 0.5,
                rng.generate::<f64>() - 0.5,
                rng.generate::<f64>() - 0.5,
            ) * 1.2;

            Some(Ray::new((p + 0.0001 * n).into(), (n + rand).normalize().into()))
        } else {
            None
        }
    }

    fn bounds(&self) -> AABBox {
        AABBox {
            min: DVec3::new(
                self.a.x.min(self.b.x).min(self.c.x),
                self.a.y.min(self.b.y).min(self.c.y),
                self.a.z.min(self.b.z).min(self.c.z),
            ),
            max: DVec3::new(
                self.a.x.max(self.b.x).max(self.c.x),
                self.a.y.max(self.b.y).max(self.c.y),
                self.a.z.max(self.b.z).max(self.c.z),
            ),
        }
    }
}

#[cfg(test)]
mod test {

    use glam::DVec3;

    use crate::{
        geometry::{Intersect, Ray},
        object::triangle::Triangle,
    };

    #[test]
    fn correct_calc_of_normal() {
        // towards +z
        assert!(
            (Triangle::from_tuples((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)).normal()
                - DVec3::new(0.0, 0.0, 1.0))
            .length()
                < 1e-9
        );

        // towards -z
        assert!(
            (Triangle::from_tuples((0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 0.0, 0.0)).normal()
                - DVec3::new(0.0, 0.0, -1.0))
            .length()
                < 1e-9
        );

        // towards x
        assert!(
            (Triangle::from_tuples((0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)).normal()
                - DVec3::new(1.0, 0.0, 0.0))
            .length()
                < 1e-9
        );

        // towards -x
        assert!(
            (Triangle::from_tuples((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), (0.0, 1.0, 0.0)).normal()
                - DVec3::new(-1.0, 0.0, 0.0))
            .length()
                < 1e-9
        );

        // towards y
        assert!(
            (Triangle::from_tuples((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0)).normal()
                - DVec3::new(0.0, 1.0, 0.0))
            .length()
                < 1e-9
        );

        // towards -y
        assert!(
            (Triangle::from_tuples((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0)).normal()
                - DVec3::new(0.0, -1.0, 0.0))
            .length()
                < 1e-9
        );
    }

    #[test]
    fn opposite_triangle_has_the_vertices_reversed() {
        let tri: Triangle = (DVec3::ZERO, DVec3::X, DVec3::Y).into();

        let opp = tri.opposite();

        let possibility_1 = Triangle::new(tri.a, tri.c, tri.b);
        let possibility_2 = Triangle::new(tri.c, tri.b, tri.a);
        let possibility_3 = Triangle::new(tri.b, tri.a, tri.c);

        assert!(
            opp.almost_equals(&possibility_1)
                || opp.almost_equals(&possibility_2)
                || opp.almost_equals(&possibility_3)
        );
    }

    #[test]
    fn opposite_triangle_has_opposite_normal() {
        let tri: Triangle = (DVec3::ZERO, DVec3::X, DVec3::Y).into();
        assert!((tri.opposite().normal() - (-DVec3::Z)).length() < 1e-9);
    }

    #[test]
    fn simple_intersection() {
        let tri = Triangle::from_tuples((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));

        let ray_center_into = Ray::from_to((0.25, 0.25, 1.0), (0.25, 0.25, -1.0));

        assert!(tri.intersect(ray_center_into).is_some());
    }

    #[test]
    fn intersection_normal_points_outwards() {
        let tri = Triangle::from_tuples((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));

        // normal points to z
        assert!(tri.normal().abs_diff_eq(DVec3::Z, 1e-9));

        let ray_center_into = Ray::from_to((0.25, 0.25, 1.0), (0.25, 0.25, -1.0));

        assert!(tri.intersect(ray_center_into).unwrap().dir.z > 0.0);
    }

    #[test]
    fn triangle_does_not_intersect_from_its_back() {
        let tri = Triangle::from_tuples((0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 0.0, 0.0));

        // normal points to -z
        assert!(tri.normal().abs_diff_eq(-DVec3::Z, 1e-9));

        let ray_center_into = Ray::from_to((0.25, 0.25, 1.0), (0.25, 0.25, -1.0));

        assert!(tri.intersect(ray_center_into).is_none());
    }

    #[test]
    fn triangle_does_not_intersect_with_back_of_ray() {
        let tri = Triangle::from_tuples((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));

        let ray_center_into = Ray::from_to((0.25, 0.25, -1.0), (0.25, 0.25, -2.0));

        assert!(tri.intersect(ray_center_into).is_none());
    }
}

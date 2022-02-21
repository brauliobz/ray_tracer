use crate::{Vec3, Float};
use nanorand::Rng;

use crate::geometry::{AABBox, Intersect, Ray};

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
}

impl Sphere {
    pub fn new((x, y, z): (Float, Float, Float), radius: Float) -> Self {
        Self {
            center: Vec3::new(x, y, z),
            radius,
        }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Ray> {
        fn delta(s: &Sphere, ray: Ray) -> Float {
            (2.0 * (ray.dir.dot(ray.origin - s.center))).powi(2)
                - 4.0 * ((ray.origin - s.center).length_squared() - s.radius * s.radius)
        }

        let delta = delta(self, ray);

        if delta <= 0.0 {
            return None;
        }

        let p = -2.0 * ray.dir.dot(ray.origin - self.center);
        let q = delta.sqrt();

        let d1 = (p + q) / 2.0;
        let d2 = (p - q) / 2.0;

        if d1 < 0.0 && d2 < 0.0 {
            return None;
        }

        let d = if d1 > 0.0 && d2 < 0.0 {
            d1
        } else if d1 < 0.0 && d2 > 0.0 {
            d2
        } else {
            d1.min(d2)
        };

        let intersect_point = ray.origin + d * ray.dir;
        let normal = (intersect_point - self.center).normalize();

        let mut rng = nanorand::tls_rng();
        let rand = Vec3::new(
            rng.generate::<Float>() - 0.5,
            rng.generate::<Float>() - 0.5,
            rng.generate::<Float>() - 0.5,
        ) / 16.0;

        Some(Ray::new(
            (intersect_point + 0.001 * normal).into(),
            (normal + rand).normalize().into(),
        ))
    }

    fn bounds(&self) -> crate::geometry::AABBox {
        AABBox {
            min: Vec3::new(
                self.center.x - self.radius,
                self.center.y - self.radius,
                self.center.z - self.radius,
            ),
            max: Vec3::new(
                self.center.x + self.radius,
                self.center.y + self.radius,
                self.center.z + self.radius,
            ),
        }
    }
}

#[test]
fn test_intersect() {
    let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));

    let ray2 = Ray::new((0.0, 0.0, 0.0), (0.0, 0.51, 3.0));

    let ray3 = Ray::new((0.0, 0.0, 0.0), (0.0, 0.49, 3.0));

    let obj = Sphere::new((0.0, 0.0, 3.0), 0.5);

    assert!(obj.intersect(ray).is_some());
    assert!(obj.intersect(ray2).is_none());
    assert!(obj.intersect(ray3).is_some());
}

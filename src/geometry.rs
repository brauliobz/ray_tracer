use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct P3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub origin: P3,
    pub dir: P3,
}

impl Vec3 {
    pub fn reflect(&self, normal: Vec3) -> Self {
        Vec3 {
            origin: normal.origin,
            dir: 2.0 * normal.dir.dot(-self.dir) * normal.dir + self.dir,
        }
    }
}

impl P3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        P3 { x, y, z }
    }

    pub fn normalize(&self) -> Self {
        if self.x == 0.0 && self.y == 0.0 && self.z == 0.0 {
            *self
        } else {
            let div = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
            P3 {
                x: self.x / div,
                y: self.y / div,
                z: self.z / div,
            }
        }
    }

    pub fn dot(&self, other: P3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn unit(&self) -> Self {
        *self / self.norm()
    }
}

pub trait Intersect {
    /// if it intersects, return the normal
    fn intersect(&self, ray: Vec3) -> Option<Vec3>;
}

impl Add<P3> for P3 {
    type Output = P3;

    fn add(self, rhs: P3) -> Self::Output {
        P3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<P3> for P3 {
    type Output = Self;

    fn sub(self, rhs: P3) -> Self::Output {
        P3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<P3> for f64 {
    type Output = P3;

    fn mul(self, rhs: P3) -> Self::Output {
        P3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for P3 {
    type Output = P3;

    fn div(self, rhs: f64) -> Self::Output {
        P3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for P3 {
    type Output = P3;

    fn neg(self) -> Self::Output {
        P3::new(-self.x, -self.y, -self.z)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Mat3 {
    v11: f64,
    v12: f64,
    v13: f64,
    v21: f64,
    v22: f64,
    v23: f64,
    v31: f64,
    v32: f64,
    v33: f64,
}

impl Mat3 {
    pub fn new(
        (v11, v12, v13): (f64, f64, f64),
        (v21, v22, v23): (f64, f64, f64),
        (v31, v32, v33): (f64, f64, f64),
    ) -> Self {
        Mat3 {
            v11,
            v12,
            v13,
            v21,
            v22,
            v23,
            v31,
            v32,
            v33,
        }
    }
}

impl Mul<P3> for Mat3 {
    type Output = P3;

    fn mul(self, p: P3) -> Self::Output {
        P3::new(
            self.v11 * p.x + self.v12 * p.y + self.v13 * p.z,
            self.v21 * p.x + self.v22 * p.y + self.v23 * p.z,
            self.v31 * p.x + self.v32 * p.y + self.v33 * p.z,
        )
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, other: Mat3) -> Self::Output {
        Mat3::new(
            (
                self.v11 * other.v11 + self.v12 * other.v21 + self.v13 * other.v31,
                self.v11 * other.v12 + self.v12 * other.v22 + self.v13 * other.v32,
                self.v11 * other.v13 + self.v12 * other.v23 + self.v13 * other.v33,
            ),
            (
                self.v21 * other.v11 + self.v22 * other.v21 + self.v23 * other.v31,
                self.v21 * other.v12 + self.v22 * other.v22 + self.v23 * other.v32,
                self.v21 * other.v13 + self.v22 * other.v23 + self.v23 * other.v33,
            ),
            (
                self.v31 * other.v11 + self.v32 * other.v21 + self.v33 * other.v31,
                self.v31 * other.v12 + self.v32 * other.v22 + self.v33 * other.v32,
                self.v31 * other.v13 + self.v32 * other.v23 + self.v33 * other.v33,
            ),
        )
    }
}

#[test]
fn test_norm_squared() {
    let a = P3::new(2.0, 2.0, 2.0);
    let b = P3::new(1.0, 1.0, 1.0);
    assert!(((a - b).norm_squared() - 3.0).abs() < 10e-9);
}

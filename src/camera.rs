use crate::{Vec3, Float};
use nanorand::Rng;

use crate::geometry::Ray;

#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub dir: Vec3,
    pub up: Vec3,
    pub x_fov: Float,
    pub y_fov: Float,
    pub sensor_distance: Float,
    pixel_lower_left: Vec3,
    x_vec: Vec3,
    y_vec: Vec3,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        dir: Vec3,
        up: Vec3,
        x_fov: Float,
        y_fov: Float,
        sensor_distance: Float,
    ) -> Camera {
        let y_vec = (y_fov / 2.0).tan() * up.normalize();
        let x_vec = (x_fov / 2.0).tan() * up.cross(dir).normalize();
        let lower_left = origin + dir * sensor_distance - x_vec - y_vec;

        Camera {
            origin,
            dir,
            sensor_distance,
            x_fov,
            y_fov,
            up,
            pixel_lower_left: lower_left,
            x_vec: x_vec * 2.0,
            y_vec: y_vec * 2.0,
        }
    }

    pub fn recalc(&mut self) {
        self.y_vec = (self.y_fov / 2.0).tan() * self.up.normalize() * 2.0;
        self.x_vec = (self.x_fov / 2.0).tan() * self.up.cross(self.dir).normalize() * 2.0;
        self.pixel_lower_left =
            self.origin + self.dir * self.sensor_distance - (self.x_vec / 2.0) - (self.y_vec / 2.0);
    }

    pub fn ray(&self, (x, y): (usize, usize), (x_res, y_res): (usize, usize)) -> Ray {
        let mut rng = nanorand::tls_rng();
        let dx = (x as Float + rng.generate::<Float>() - 0.5) / x_res as Float;
        let dy = (y as Float + rng.generate::<Float>() - 0.5) / y_res as Float;

        let dir = (self.pixel_lower_left + dx * self.x_vec + dy * self.y_vec) - self.origin;

        Ray::new(self.origin.into(), dir.into())
    }
}

#[test]
fn camera() {
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        (90.0 as Float).to_radians(),
        (90.0 as Float).to_radians(),
        1.0,
    );

    dbg!(&camera);

    // ray going near the center
    assert!((camera.ray((1000, 1000), (2000, 2000)).dir.x - 0.0).abs() <= 10e-4);
    assert!((camera.ray((1000, 1000), (2000, 2000)).dir.y - 0.0).abs() <= 10e-4);
    assert!((camera.ray((1000, 1000), (2000, 2000)).dir.z - 1.0).abs() <= 10e-4);

    // ray going near the lower left
    let p = Vec3::new(-1.0, -1.0, 1.0).normalize();
    assert!((camera.ray((0, 0), (2000, 2000)).dir.x - p.x).abs() <= 10e-4);
    assert!((camera.ray((0, 0), (2000, 2000)).dir.y - p.y).abs() <= 10e-4);
    assert!((camera.ray((0, 0), (2000, 2000)).dir.z - p.z).abs() <= 10e-4);

    // ray going near the lower right
    let p = Vec3::new(1.0, -1.0, 1.0).normalize();
    assert!((camera.ray((2000, 0), (2000, 2000)).dir.x - p.x).abs() <= 10e-4);
    assert!((camera.ray((2000, 0), (2000, 2000)).dir.y - p.y).abs() <= 10e-4);
    assert!((camera.ray((2000, 0), (2000, 2000)).dir.z - p.z).abs() <= 10e-4);

    // ray going near the top right
    let p = Vec3::new(1.0, 1.0, 1.0).normalize();
    assert!((camera.ray((2000, 2000), (2000, 2000)).dir.x - p.x).abs() <= 10e-4);
    assert!((camera.ray((2000, 2000), (2000, 2000)).dir.y - p.y).abs() <= 10e-4);
    assert!((camera.ray((2000, 2000), (2000, 2000)).dir.z - p.z).abs() <= 10e-4);
}

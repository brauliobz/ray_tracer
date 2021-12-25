use glam::DVec3;
use nanorand::Rng;

use crate::geometry::Ray;

#[derive(Debug)]
pub struct Camera {
    pub origin: DVec3,
    pub dir: DVec3,
    pub up: DVec3,
    pub x_fov: f64,
    pub y_fov: f64,
    pub sensor_distance: f64,
    pixel_lower_left: DVec3,
    x_vec: DVec3,
    y_vec: DVec3,
}

impl Camera {
    pub fn new(
        origin: DVec3,
        dir: DVec3,
        up: DVec3,
        x_fov: f64,
        y_fov: f64,
        sensor_distance: f64,
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

    pub fn ray(&self, (x, y): (usize, usize), (x_res, y_res): (usize, usize)) -> Ray {
        let mut rng = nanorand::tls_rng();
        let dx = (x as f64 + rng.generate::<f64>() - 0.5) / x_res as f64;
        let dy = (y as f64 + rng.generate::<f64>() - 0.5) / y_res as f64;

        let dir = (self.pixel_lower_left + dx * self.x_vec + dy * self.y_vec) - self.origin;

        Ray {
            origin: self.origin,
            dir: dir.normalize(),
        }
    }
}

#[test]
fn camera() {
    let camera = Camera::new(
        DVec3::new(0.0, 0.0, -1.0),
        DVec3::new(0.0, 0.0, 1.0),
        DVec3::new(0.0, 1.0, 0.0),
        90.0f64.to_radians(),
        90.0f64.to_radians(),
        1.0,
    );

    dbg!(&camera);

    // ray going near the center
    assert!((camera.ray((1000, 1000), (2000, 2000)).dir.x - 0.0).abs() <= 10e-4);
    assert!((camera.ray((1000, 1000), (2000, 2000)).dir.y - 0.0).abs() <= 10e-4);
    assert!((camera.ray((1000, 1000), (2000, 2000)).dir.z - 1.0).abs() <= 10e-4);

    // ray going near the lower left
    let p = DVec3::new(-1.0, -1.0, 1.0).normalize();
    assert!((camera.ray((0, 0), (2000, 2000)).dir.x - p.x).abs() <= 10e-4);
    assert!((camera.ray((0, 0), (2000, 2000)).dir.y - p.y).abs() <= 10e-4);
    assert!((camera.ray((0, 0), (2000, 2000)).dir.z - p.z).abs() <= 10e-4);

    // ray going near the lower right
    let p = DVec3::new(1.0, -1.0, 1.0).normalize();
    assert!((camera.ray((2000, 0), (2000, 2000)).dir.x - p.x).abs() <= 10e-4);
    assert!((camera.ray((2000, 0), (2000, 2000)).dir.y - p.y).abs() <= 10e-4);
    assert!((camera.ray((2000, 0), (2000, 2000)).dir.z - p.z).abs() <= 10e-4);

    // ray going near the top right
    let p = DVec3::new(1.0, 1.0, 1.0).normalize();
    assert!((camera.ray((2000, 2000), (2000, 2000)).dir.x - p.x).abs() <= 10e-4);
    assert!((camera.ray((2000, 2000), (2000, 2000)).dir.y - p.y).abs() <= 10e-4);
    assert!((camera.ray((2000, 2000), (2000, 2000)).dir.z - p.z).abs() <= 10e-4);
}

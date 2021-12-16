use glam::{DVec3, DMat3};
use nanorand::Rng;

use crate::geometry::Ray;

pub struct Camera {
    pub origin: DVec3,
    pub dir: DVec3,
    pub rotation_z: f64,
    pub x_fov: f64,
    pub y_fov: f64,
    pub film_distance: f64,
    pixel_lower_left: DVec3,
    x_vec: DVec3,
    y_vec: DVec3,
}

impl Camera {
    pub fn new(origin: DVec3, dir: DVec3, rotation_z: f64, x_fov: f64, y_fov: f64, film_distance: f64) -> Camera {
        let angle_x = x_fov / 2.0;
        let angle_y = y_fov / 2.0;

        let rotate_x_left = DMat3::from_cols(
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, angle_x.cos(), angle_x.sin()),
            DVec3::new(0.0, -angle_x.sin(), angle_x.cos()),
        ).transpose();

        let rotate_x_right = DMat3::from_cols(
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, angle_x.cos(), -angle_x.sin()),
            DVec3::new(0.0, angle_x.sin(), angle_x.cos()),
        ).transpose();

        let rotate_y_lower = DMat3::from_cols(
            DVec3::new(angle_y.cos(), 0.0, -angle_y.sin()),
            DVec3::new(0.0, 1.0, 0.0),
            DVec3::new(angle_y.sin(), 0.0, angle_y.cos()),
        ).transpose();

        let rotate_y_top = DMat3::from_cols(
            DVec3::new(angle_y.cos(), 0.0, angle_y.sin()),
            DVec3::new(0.0, 1.0, 0.0),
            DVec3::new(-angle_y.sin(), 0.0, angle_y.cos()),
        ).transpose();

        let lower_right = rotate_y_lower * rotate_x_right * dir + origin;
        let lower_left = rotate_y_lower * rotate_x_left * dir + origin;
        let top_left = rotate_y_top * rotate_x_left * dir + origin;

        let x_vec = lower_right - lower_left;
        let y_vec = top_left - lower_left;

        Camera {
            origin,
            dir,
            film_distance,
            x_fov,
            y_fov,
            rotation_z,
            pixel_lower_left: lower_left,
            x_vec,
            y_vec,
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

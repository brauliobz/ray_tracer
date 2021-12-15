use nanorand::Rng;

use crate::geometry::{Mat3, Vec3, P3};

pub struct Camera {
    pub dir: Vec3,
    pub rotation_z: f64,
    pub x_fov: f64,
    pub y_fov: f64,
    pub film_distance: f64,
    pixel_lower_left: P3,
    x_vec: P3,
    y_vec: P3,
}

impl Camera {
    pub fn new(dir: Vec3, rotation_z: f64, x_fov: f64, y_fov: f64, film_distance: f64) -> Camera {
        let angle_x = x_fov / 2.0;
        let angle_y = y_fov / 2.0;

        let rotate_x_left = Mat3::new(
            (1.0, 0.0, 0.0),
            (0.0, angle_x.cos(), angle_x.sin()),
            (0.0, -angle_x.sin(), angle_x.cos()),
        );

        let rotate_x_right = Mat3::new(
            (1.0, 0.0, 0.0),
            (0.0, angle_x.cos(), -angle_x.sin()),
            (0.0, angle_x.sin(), angle_x.cos()),
        );

        let rotate_y_lower = Mat3::new(
            (angle_y.cos(), 0.0, -angle_y.sin()),
            (0.0, 1.0, 0.0),
            (angle_y.sin(), 0.0, angle_y.cos()),
        );

        let rotate_y_top = Mat3::new(
            (angle_y.cos(), 0.0, angle_y.sin()),
            (0.0, 1.0, 0.0),
            (-angle_y.sin(), 0.0, angle_y.cos()),
        );

        let lower_right = rotate_y_lower * rotate_x_right * dir.dir + dir.origin;
        let lower_left = rotate_y_lower * rotate_x_left * dir.dir + dir.origin;
        let top_left = rotate_y_top * rotate_x_left * dir.dir + dir.origin;

        let x_vec = lower_right - lower_left;
        let y_vec = top_left - lower_left;

        Camera {
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

    pub fn rays(&self, x_res: usize, y_res: usize, samples_per_pixel: usize) -> RayIterator {
        RayIterator::new(self, x_res, y_res, samples_per_pixel)
    }

    pub fn ray(&self, (x, y): (usize, usize), (x_res, y_res): (usize, usize)) -> Vec3 {
        let mut rng = nanorand::tls_rng();
        let dx = (x as f64 + rng.generate::<f64>() - 0.5) / x_res as f64;
        let dy = (y as f64 + rng.generate::<f64>() - 0.5) / y_res as f64;

        let dir = (self.pixel_lower_left + dx * self.x_vec + dy * self.y_vec) - self.dir.origin;

        Vec3 {
            origin: self.dir.origin,
            dir: dir.normalize(),
        }
    }
}

pub struct RayIterator<'a> {
    camera: &'a Camera,
    x_res: usize,
    y_res: usize,
    x: usize,
    y: usize,
    lower_left: P3,
    x_vec: P3,
    y_vec: P3,
    samples_per_pixel: usize,
    cur_sample: usize,
}

impl<'a> RayIterator<'a> {
    pub fn new(cam: &'a Camera, x_res: usize, y_res: usize, samples_per_pixel: usize) -> Self {
        let angle_x = cam.x_fov / 2.0;
        let angle_y = cam.y_fov / 2.0;

        let rotate_x_left = Mat3::new(
            (1.0, 0.0, 0.0),
            (0.0, angle_x.cos(), angle_x.sin()),
            (0.0, -angle_x.sin(), angle_x.cos()),
        );

        let rotate_x_right = Mat3::new(
            (1.0, 0.0, 0.0),
            (0.0, angle_x.cos(), -angle_x.sin()),
            (0.0, angle_x.sin(), angle_x.cos()),
        );

        let rotate_y_lower = Mat3::new(
            (angle_y.cos(), 0.0, -angle_y.sin()),
            (0.0, 1.0, 0.0),
            (angle_y.sin(), 0.0, angle_y.cos()),
        );

        let rotate_y_top = Mat3::new(
            (angle_y.cos(), 0.0, angle_y.sin()),
            (0.0, 1.0, 0.0),
            (-angle_y.sin(), 0.0, angle_y.cos()),
        );

        let lower_right = rotate_y_lower * rotate_x_right * cam.dir.dir + cam.dir.origin;
        let lower_left = rotate_y_lower * rotate_x_left * cam.dir.dir + cam.dir.origin;
        let top_left = rotate_y_top * rotate_x_left * cam.dir.dir + cam.dir.origin;

        let x_vec = lower_right - lower_left;
        let y_vec = top_left - lower_left;

        RayIterator {
            camera: cam,
            x_res,
            y_res,
            x: 0,
            y: 0,
            lower_left,
            x_vec,
            y_vec,
            samples_per_pixel,
            cur_sample: 0,
        }
    }
}

impl<'a> Iterator for RayIterator<'a> {
    type Item = (usize, usize, Vec3);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.x, self.y);

        if self.y >= self.y_res {
            return None;
        }

        if self.cur_sample > self.samples_per_pixel {
            self.cur_sample = 0;
            if self.x == self.x_res - 1 {
                self.x = 0;
                self.y += 1;
            } else {
                self.x += 1;
            }
        } else {
            self.cur_sample += 1;
        }

        let mut rng = nanorand::tls_rng();
        let dx = (x as f64 + rng.generate::<f64>() - 0.5) / self.x_res as f64;
        let dy = (y as f64 + rng.generate::<f64>() - 0.5) / self.y_res as f64;
        let dir = (self.lower_left + dx * self.x_vec + dy * self.y_vec) - self.camera.dir.origin;

        let ray = Vec3 {
            origin: self.camera.dir.origin,
            dir: dir.normalize(),
        };

        Some((x, y, ray))
    }
}

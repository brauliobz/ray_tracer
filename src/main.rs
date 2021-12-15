use std::{
    f64::consts::PI,
    fs::File,
    io::BufWriter,
    ops::{Add, Div, Mul, Neg, Sub},
};

use rand::{random, thread_rng, Rng};

fn main() {
    // scene objects
    let scene = &vec![
        Sphere::new((0.1, 0.1, 0.1), 1.5),
        Sphere::new((3.0, 3.0, 0.0), 1.0),
        Sphere::new((3.0, -2.0, 0.0), 2.0),
        Sphere::new((0.0, 0.0, 100.0), 80.0),
    ];

    // lights
    let lights = &vec![
        Sphere::new((1000.0, 1000.0, -1000.0), 300.0),
        Sphere::new((1.5, 1.5, -1.0), 0.3),
        Sphere::new((1.0, -3.0, -1.0), 0.3),
    ];

    // camera
    let (x_res, y_res) = (1920, 1072);
    let fov = 80.0 * 2.0 * PI / 360.0;
    let camera = &Camera::new(
        Vec3 {
            origin: P3::new(1.0, 0.0, -5.0),
            dir: P3::new(0.0, 0.0, 1.0).normalize(),
        },
        0.0,
        fov,
        fov * (y_res as f64 / x_res as f64),
        0.1,
    );

    let mut image = vec![0.0; x_res * y_res];

    let num_threads = 16;
    let y_block_size = y_res / num_threads;

    // iterate rays to calculate pixels

    let samples_per_pixel = 128;
    let _ = crossbeam::scope(|scope| {
        for (thread_num, chunk) in image.chunks_mut(y_block_size * x_res).enumerate() {
            scope.spawn(move |_| {
                for y in 0..y_block_size {
                    for x in 0..x_res {
                        let abs_x = x;
                        let abs_y = thread_num * y_block_size + y;
                        for _ in 0..samples_per_pixel {
                            let ray = &camera.ray((abs_x, abs_y), (x_res, y_res));
                            chunk[y * x_res + x] += (1.0 / samples_per_pixel as f64)
                                * ray_trace(*ray, scene, lights, 5);
                        }
                    }
                }
            });
        }
    });

    // save pixels
    save_to_png(&image, x_res, y_res);
}

fn ray_trace(ray: Vec3, scene: &[Sphere], lights: &[Sphere], remaining_steps: usize) -> f64 {
    // find intersections
    let mut vec = vec![];
    for obj in scene {
        if let Some(normal) = obj.intersect(ray) {
            vec.push(((normal.origin - ray.origin).norm_squared(), normal, false));
        }
    }
    for light in lights {
        if let Some(normal) = light.intersect(ray) {
            vec.push(((normal.origin - ray.origin).norm_squared(), normal, true));
        }
    }

    // get the nearest
    vec.sort_by(|(dist_a, _, _), (dist_b, _, _)| {
        dist_a
            .partial_cmp(dist_b)
            .unwrap_or(std::cmp::Ordering::Less)
    });

    // calculate light intensity
    if let Some((_, _, true)) = vec.first() {
        1.0
    } else if let Some((_, normal, false)) = vec.first() {
        if remaining_steps > 1 {
            // calculate reflected ray
            let reflected = ray.reflect(*normal);
            0.95 * ray_trace(reflected, scene, lights, remaining_steps - 1)
        } else {
            0.1
        }
    } else {
        0.1
    }
}

fn save_to_png(image: &[f64], x_res: usize, y_res: usize) {
    let writer = BufWriter::new(File::create("output.png").unwrap());

    let mut encoder = png::Encoder::new(writer, x_res as u32, y_res as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);

    let mut data_writer = encoder.write_header().unwrap();

    let data: Vec<u8> = image.iter().map(|gray| (256.0 * gray) as u8).collect();
    data_writer.write_image_data(&data).unwrap();
}

pub trait Intersect {
    /// if it intersects, return the normal
    fn intersect(&self, ray: Vec3) -> Option<Vec3>;
}

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

pub struct Sphere {
    pub center: P3,
    pub radius: f64,
}

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

impl Sphere {
    pub fn new((x, y, z): (f64, f64, f64), radius: f64) -> Self {
        Self {
            center: P3 { x, y, z },
            radius,
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
        let dx = (x as f64 + random::<f64>() - 0.5) / x_res as f64;
        let dy = (y as f64 + random::<f64>() - 0.5) / y_res as f64;

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

        let dx = (x as f64 + random::<f64>() - 0.5) / self.x_res as f64;
        let dy = (y as f64 + random::<f64>() - 0.5) / self.y_res as f64;
        let dir = (self.lower_left + dx * self.x_vec + dy * self.y_vec) - self.camera.dir.origin;

        let ray = Vec3 {
            origin: self.camera.dir.origin,
            dir: dir.normalize(),
        };

        Some((x, y, ray))
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: Vec3) -> Option<Vec3> {
        let delta = (2.0 * (ray.dir.dot(ray.origin - self.center))).powi(2)
            - 4.0 * ((ray.origin - self.center).norm_squared() - self.radius * self.radius);

        if delta <= 0.0 {
            return None;
        }

        let p = -2.0 * ray.dir.dot(ray.origin - self.center);
        let q = delta.sqrt();

        let d1 = (p + q) / 2.0;
        let d2 = (p - q) / 2.0;

        if d1 < 1e-9 && d2 < 1e-9 {
            return None;
        }

        let d = if d1 > 1e-9 && d1 < 1e-9 {
            d1
        } else if d1 < 1e-9 && d2 > 1e-9 {
            d2
        } else {
            d1.min(d2)
        };

        let rand = P3::new(
            thread_rng().sample::<f64, _>(rand_distr::StandardNormal) - 0.5,
            thread_rng().sample::<f64, _>(rand_distr::StandardNormal) - 0.5,
            thread_rng().sample::<f64, _>(rand_distr::StandardNormal) - 0.5,
        ) / 64.0;

        let intersect_point = ray.origin + d * (ray.dir + rand);
        let normal = Vec3 {
            origin: intersect_point,
            dir: (intersect_point - self.center).normalize(),
        };

        Some(normal)
    }
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
    fn new(
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
fn test_intersect() {
    let ray = Vec3 {
        origin: P3::new(0.0, 0.0, 0.0),
        dir: P3::new(0.0, 0.0, 1.0),
    };

    let ray2 = Vec3 {
        origin: P3::new(0.0, 0.0, 0.0),
        dir: P3::new(0.0, 0.51, 3.0).normalize(),
    };

    let ray3 = Vec3 {
        origin: P3::new(0.0, 0.0, 0.0),
        dir: P3::new(0.0, 0.49, 3.0).normalize(),
    };

    let obj = Sphere::new((0.0, 0.0, 3.0), 0.5);

    assert!(obj.intersect(ray).is_some());
    assert!(obj.intersect(ray2).is_none());
    assert!(obj.intersect(ray3).is_some());
}

#[test]
fn test_norm_squared() {
    let a = P3::new(2.0, 2.0, 2.0);
    let b = P3::new(1.0, 1.0, 1.0);
    assert!(((a - b).norm_squared() - 3.0).abs() < 10e-9);
}

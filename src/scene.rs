use glam::DVec3;

use crate::{
    camera::Camera,
    geometry::Intersect,
    object::{sphere::Sphere, triangle::Triangle},
};

pub struct Scene {
    pub objects: Vec<Box<dyn Intersect>>,
    pub lights: Vec<Sphere>,
    pub camera: Camera,
}

pub fn icosahedron() -> Scene {
    let p = [
        DVec3::new(0.000000, -1.000000, 0.000000) * 2.0,
        DVec3::new(0.000000, -1.000000, 0.000000) * 2.0,
        DVec3::new(0.723600, -0.447215, 0.525720) * 2.0,
        DVec3::new(-0.276385, -0.447215, 0.850640) * 2.0,
        DVec3::new(-0.894425, -0.447215, 0.000000) * 2.0,
        DVec3::new(-0.276385, -0.447215, -0.850640) * 2.0,
        DVec3::new(0.723600, -0.447215, -0.525720) * 2.0,
        DVec3::new(0.276385, 0.447215, 0.850640) * 2.0,
        DVec3::new(-0.723600, 0.447215, 0.525720) * 2.0,
        DVec3::new(-0.723600, 0.447215, -0.525720) * 2.0,
        DVec3::new(0.276385, 0.447215, -0.850640) * 2.0,
        DVec3::new(0.894425, 0.447215, 0.000000) * 2.0,
        DVec3::new(0.000000, 1.000000, 0.000000) * 2.0,
    ];

    let objects: Vec<Box<dyn Intersect>> = vec![
        Box::new(Triangle::new(p[1], p[2], p[3])),
        Box::new(Triangle::new(p[2], p[1], p[6])),
        Box::new(Triangle::new(p[1], p[3], p[4])),
        Box::new(Triangle::new(p[1], p[4], p[5])),
        Box::new(Triangle::new(p[1], p[5], p[6])),
        Box::new(Triangle::new(p[2], p[6], p[11])),
        Box::new(Triangle::new(p[3], p[2], p[7])),
        Box::new(Triangle::new(p[4], p[3], p[8])),
        Box::new(Triangle::new(p[5], p[4], p[9])),
        Box::new(Triangle::new(p[6], p[5], p[10])),
        Box::new(Triangle::new(p[2], p[11], p[7])),
        Box::new(Triangle::new(p[3], p[7], p[8])),
        Box::new(Triangle::new(p[4], p[8], p[9])),
        Box::new(Triangle::new(p[5], p[9], p[10])),
        Box::new(Triangle::new(p[6], p[10], p[11])),
        Box::new(Triangle::new(p[7], p[11], p[12])),
        Box::new(Triangle::new(p[8], p[7], p[12])),
        Box::new(Triangle::new(p[9], p[8], p[12])),
        Box::new(Triangle::new(p[10], p[9], p[12])),
        Box::new(Triangle::new(p[11], p[10], p[12])),
    ];

    let lights = vec![
        Sphere::new((40.0, 0.0, 0.0), 19.0),
        Sphere::new((0.0, 40.0, 0.0), 19.0),
        Sphere::new((40.0, 40.0, 40.0), 19.0),
        Sphere::new((40.0, -40.0, 40.0), 19.0),
        Sphere::new((-40.0, 40.0, 40.0), 19.0),
        Sphere::new((-40.0, -40.0, 40.0), 19.0),
    ];

    let cam_origin = DVec3::new(0.0, 0.0, 8.0);
    let fov = 90.0f64.to_radians();
    let camera = Camera::new(
        cam_origin,
        (DVec3::new(0.0, 0.0, 0.0) - cam_origin).normalize(),
        DVec3::new(0.0, -1.0, 0.0).normalize(),
        fov,
        fov,
        2.0,
    );

    Scene {
        objects,
        lights,
        camera,
    }
}

use glam::DVec3;

use crate::{
    camera::Camera,
    geometry::Intersect,
    object::{import_from_wavefront_obj_file, sphere::Sphere, triangle::Triangle},
};

pub struct MovieScene {
    pub scene: Scene,
    pub n_frames: usize,
    pub calc_frame_fn: Option<Box<dyn Fn(&mut Scene, usize)>>,
}

pub struct Scene {
    pub objects: Vec<Box<dyn Intersect>>,
    pub lights: Vec<Sphere>,
    pub camera: Camera,
}

impl MovieScene {
    pub fn calc_frame(&mut self, frame: usize) {
        if let Some(calc_frame_fn) = &self.calc_frame_fn {
            (calc_frame_fn)(&mut self.scene, frame);
        }
    }
}

pub fn spheres() -> MovieScene {
    let cam_origin = DVec3::new(0.0, 0.0, 8.0);
    MovieScene {
        scene: Scene {
            objects: vec![
                Box::new(Sphere::new((0.0, 0.0, 0.0), 2.0)),
                Box::new(Sphere::new((5.0, 0.0, -3.0), 2.0)),
                Box::new(Sphere::new((-2.5, 0.0, 2.0), 2.0)),
                Box::new(Sphere::new((0.5, -1.5, 2.0), 1.0)),
                Box::new(Sphere::new((2.1, 2.1, 2.0), 0.6)),
                Box::new(Triangle::from_tuples(
                    (-100.0, -10.0, 100.0),
                    (100.0, -10.0, 100.0),
                    (0.0, -10.0, -200.0),
                )),
            ],
            lights: vec![Sphere::new((20.0, 30.0, 20.0), 10.0)],
            camera: Camera::new(
                cam_origin,
                (DVec3::new(0.0, 0.0, 0.0) - cam_origin).normalize(),
                DVec3::new(0.0, -1.0, 0.0).normalize(),
                90.0f64.to_radians(),
                90.0f64.to_radians(),
                2.0,
            ),
        },
        n_frames: 1,
        calc_frame_fn: None,
    }
}

pub fn icosahedron() -> MovieScene {
    let mut ico = spinning_icosahedron();
    ico.n_frames = 1;
    ico.calc_frame_fn = None;
    ico
}

pub fn spinning_icosahedron() -> MovieScene {
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
        Box::new(Triangle::from_tuples(
            (-100.0, -5.0, 100.0),
            (100.0, -5.0, 100.0),
            (0.0, -5.0, -200.0),
        )),
    ];

    let lights = vec![Sphere::new((40.0, 30.0, 0.0), 15.0)];

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

    let n_frames = 64;
    let calc_frame_fn = Box::new(move |scene: &mut Scene, frame: usize| {
        scene.camera.origin.z =
            8.0 * f64::cos(frame as f64 / n_frames as f64 * 2.0 * std::f64::consts::PI);
        scene.camera.origin.x =
            8.0 * f64::sin(frame as f64 / n_frames as f64 * 2.0 * std::f64::consts::PI);
        scene.camera.dir = (-scene.camera.origin).normalize();
        scene.camera.recalc();
    });

    MovieScene {
        scene: Scene {
            objects,
            lights,
            camera,
        },
        n_frames,
        calc_frame_fn: Some(calc_frame_fn),
    }
}

pub fn scene_from_obj_file() -> MovieScene {
    let lights = vec![Sphere::new((40.0, 30.0, 0.0), 15.0)];
    let mut objects = import_from_wavefront_obj_file("./torus.obj");

    // floor
    objects.push(Box::new(Triangle::from_tuples(
        (-100.0, -75.0, 100.0),
        (100.0, -75.0, 100.0),
        (0.0, -75.0, -200.0),
    )));

    let cam_origin = DVec3::new(0.0, 2.0, 2.0);
    let fov = 90.0f64.to_radians();
    let camera = Camera::new(
        cam_origin,
        (DVec3::new(0.0, -0.5, 0.0) - cam_origin).normalize(),
        DVec3::new(0.0, -1.0, 0.0).normalize(),
        fov,
        fov,
        1.0,
    );

    MovieScene {
        scene: Scene {
            camera,
            lights,
            objects,
        },
        n_frames: 1,
        calc_frame_fn: None,
    }
}

pub fn icosphere() -> MovieScene {
    let lights = vec![Sphere::new((40.0, 30.0, 0.0), 15.0)];
    let mut objects = import_from_wavefront_obj_file("./icosphere.obj");

    println!("loaded {} triangles", objects.len());

    // floor
    objects.push(Box::new(Triangle::from_tuples(
        (-100.0, -75.0, 100.0),
        (100.0, -75.0, 100.0),
        (0.0, -75.0, -200.0),
    )));

    let cam_origin = DVec3::new(0.0, 0.0, 2.1);
    let fov = 90.0f64.to_radians();
    let camera = Camera::new(
        cam_origin,
        (DVec3::new(0.0, -0.5, 0.0) - cam_origin).normalize(),
        DVec3::new(0.0, -1.0, 0.0).normalize(),
        fov,
        fov,
        2.0,
    );

    MovieScene {
        scene: Scene {
            camera,
            lights,
            objects,
        },
        n_frames: 1,
        calc_frame_fn: None,
    }
}

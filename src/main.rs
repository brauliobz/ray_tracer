mod camera;
mod geometry;
mod object;

use std::{f64::consts::PI, fs::File, io::BufWriter};

use camera::Camera;
use geometry::{Intersect, Vec3, P3};
use object::sphere::Sphere;

fn main() {
    // configs
    let frames = 1;
    let num_threads = 16;
    let samples_per_pixel = 128;
    let (x_res, y_res) = (16 * 16 * 4, 16 * 16 * 4);
    let fov = 60.0 * 2.0 * PI / 360.0;
    let max_reflections = 10;
    let gamma_correction = 1.0 / 2.0;

    // scene objects
    let scene = &vec![
        Sphere::new((0.1, 0.1, 0.1), 1.5),
        Sphere::new((3.0, 3.0, 0.0), 1.0),
        Sphere::new((3.0, -2.0, 0.0), 2.0),
        Sphere::new((0.0, 0.0, 100.0), 80.0),
    ];

    // lights
    let lights = &vec![
        Sphere::new((-8000.0, 0.0, 0.0), 4900.0),
        Sphere::new((10.5, 1.5, -1.0), 0.3),
        Sphere::new((-10.0, -3.0, -1.0), 0.3),
    ];

    // camera
    let mut camera = Camera::new(
        Vec3 {
            origin: P3::new(1.0, 0.0, -8.0),
            dir: P3::new(0.0, 0.0, 1.0).normalize(),
        },
        0.0,
        fov,
        fov * (y_res as f64 / x_res as f64),
        0.1,
    );

    for frame in 0..frames {
        let mut image = vec![0.0; x_res * y_res];

        let y_block_size = y_res / num_threads;

        // iterate rays to calculate pixels

        let cam_ref = &camera;
        let _ = crossbeam::scope(|scope| {
            for (thread_num, chunk) in image.chunks_mut(y_block_size * x_res).enumerate() {
                scope.spawn(move |_| {
                    for y in 0..y_block_size {
                        for x in 0..x_res {
                            let abs_x = x;
                            let abs_y = thread_num * y_block_size + y;
                            for _ in 0..samples_per_pixel {
                                let ray = cam_ref.ray((abs_x, abs_y), (x_res, y_res));
                                chunk[y * x_res + x] += (1.0 / samples_per_pixel as f64)
                                    * ray_trace(ray, scene, lights, max_reflections + 1);
                            }
                        }
                    }
                });
            }
        });

        // save pixels
        save_to_png(
            format!("output-{:04}.png", frame).as_str(),
            &image,
            x_res,
            y_res,
            gamma_correction,
        );
        println!("Frame {} completed", frame);

        // change camera position
        camera.dir.origin.z -= 0.01;
        camera.dir.origin.y += 0.01;
        camera.dir.dir = (P3::new(0.1, 0.1, 0.1) - camera.dir.origin).normalize();
    }
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
            0.98 * ray_trace(reflected, scene, lights, remaining_steps - 1)
        } else {
            0.0
        }
    } else {
        0.0
    }
}

fn save_to_png(name: &str, image: &[f64], x_res: usize, y_res: usize, gamma_correction: f64) {
    let writer = BufWriter::new(File::create(name).unwrap());

    let mut encoder = png::Encoder::new(writer, x_res as u32, y_res as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);

    let mut data_writer = encoder.write_header().unwrap();

    let data: Vec<u8> = image.iter().map(|gray| (256.0 * gray.powf(gamma_correction)) as u8).collect();
    data_writer.write_image_data(&data).unwrap();
}

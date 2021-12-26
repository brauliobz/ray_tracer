use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use log::debug;

use crate::{
    geometry::{Intersect, Ray},
    object::sphere::Sphere,
    scene::Scene,
};

pub fn render(
    scene: &Scene,
    x_res: usize,
    y_res: usize,
    num_threads: usize,
    samples_per_pixel: usize,
    max_reflections: usize,
    image: &mut Vec<f64>,
) {
    let y_block_size = y_res / num_threads;

    // iterate rays to calculate pixels

    let pixels_rendered = Arc::new(AtomicUsize::new(0));

    let _ = crossbeam::scope(|scope| {
        for (thread_num, chunk) in image.chunks_mut(y_block_size * x_res).enumerate() {
            let pixels_counter = pixels_rendered.clone();
            scope.spawn(move |_| {
                for y in 0..y_block_size {
                    let mut curr = 0;
                    for x in 0..x_res {
                        let abs_x = x;
                        let abs_y = thread_num * y_block_size + y;
                        for _ in 0..samples_per_pixel {
                            let ray = scene.camera.ray((abs_x, abs_y), (x_res, y_res));
                            chunk[y * x_res + x] += (1.0 / samples_per_pixel as f64)
                                * trace_ray(
                                    ray,
                                    &scene.objects,
                                    &scene.lights,
                                    max_reflections + 1,
                                );
                        }
                        curr = pixels_counter.fetch_add(1, Ordering::Acquire) + 1;
                    }
                    println!(
                        "{}/{} pixels rendered. {:.1}%",
                        curr,
                        x_res * y_res,
                        curr as f64 / (x_res as f64 * y_res as f64) * 100.0
                    );
                }
            });
        }
    });
}

pub fn trace_ray(
    ray: Ray,
    scene: &[Box<dyn Intersect>],
    lights: &[Sphere],
    remaining_steps: usize,
) -> f64 {
    debug!("tracing ray {:?}", ray);

    // find intersections
    let mut vec = vec![];
    for obj in scene {
        debug!("testing intersection with {:?}", obj);
        if let Some(normal) = obj.intersect(ray) {
            debug!(
                "ray intersects with {:?} with distance {:.3}",
                obj,
                (normal.origin - ray.origin).length()
            );
            vec.push(((normal.origin - ray.origin).length_squared(), normal, false));
        } else {
            debug!("no intersection with object {:?}", obj);
        }
    }
    for light in lights {
        debug!("testing intersection with light {:?}", light);
        if let Some(normal) = light.intersect(ray) {
            debug!(
                "ray intersects with light {:?} with distance {:.3}",
                light,
                (normal.origin - ray.origin).length()
            );
            vec.push(((normal.origin - ray.origin).length_squared(), normal, true));
        } else {
            debug!("no intersection with light {:?}", light);
        }
    }

    // get the nearest
    vec.sort_by(|(dist_a, _, _), (dist_b, _, _)| {
        dist_a
            .partial_cmp(dist_b)
            .unwrap_or(std::cmp::Ordering::Less)
    });

    // calculate light intensity
    if let Some((_, normal, true)) = vec.first() {
        debug!("ray reaches a light source at {}", normal.origin);
        1.0
    } else if let Some((_, normal, false)) = vec.first() {
        debug!("ray reaches an object at {}", normal.origin);
        if remaining_steps > 1 {
            // calculate reflected ray
            let reflected = ray.reflect(*normal);
            debug!("reflected ray is {:?}", reflected);
            debug!("{} remaining steps", remaining_steps - 1);
            0.98 * trace_ray(reflected, scene, lights, remaining_steps - 1)
        } else {
            debug!("no more recursion available");
            0.0
        }
    } else {
        0.0
    }
}

#[cfg(test)]
mod test {
    use log::debug;

    use crate::{
        geometry::{Intersect, Ray},
        object::{sphere::Sphere, triangle::Triangle},
        tracer::trace_ray,
    };

    // #[test]
    #[allow(unused)]
    fn reflect_triangle() {
        let ray_along_z = Ray::from_to((0.0, 0.0, -1.0), (0.0, 0.0, 1.0));
        let scene: Vec<Box<dyn Intersect>> = vec![Box::new(Triangle::from_tuples(
            (0.0, 10.0, 0.1),
            (10.0, -10.0, 0.0),
            (-10.0, -10.0, 0.0),
        ))];
        let lights = vec![Sphere::new((0.0, 0.0, -2.0), 0.2)];

        assert!(trace_ray(ray_along_z, &scene, &lights, 3) > 0.5);
    }

    #[test]
    fn reflect_triangle2() {
        env_logger::init();

        let ray = Ray::from_to((0.0, 0.0, 8.0), (4.0, -4.0, 0.0));
        let scene: Vec<Box<dyn Intersect>> = vec![Box::new(Triangle::from_tuples(
            (-2.5, -4.5, 0.0),
            (6.5, 4.5, 0.0),
            (6.5, -4.5, 0.0),
        ))];
        let lights = vec![Sphere::new((4.0, 0.0, 2.0), 1.0)];

        debug!("{:?}", trace_ray(ray, &scene, &lights, 10));
    }
}

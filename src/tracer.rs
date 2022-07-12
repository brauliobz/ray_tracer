use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use log::{debug, info};

use crate::{
    geometry::{AABBox, Intersect, Ray},
    object::sphere::Sphere,
    scene::Scene,
    space_partition::{kdtree::KdTree, octree::Octree, SpacePartition},
};

pub fn render(
    scene: &Scene,
    x_res: usize,
    y_res: usize,
    num_threads: usize,
    samples_per_pixel: usize,
    max_reflections: usize,
    image: &mut [f64],
) {
    let y_block_size = y_res / num_threads;

    // iterate rays to calculate pixels

    let pixels_rendered = Arc::new(AtomicUsize::new(0));

    // info!("Constructing octree...");
    // let space_partition = &Octree::new(
    //     &scene
    //         .objects
    //         .iter()
    //         .map(|obj_box| obj_box.as_ref())
    //         .collect::<Vec<_>>(),
    //     10,
    //     16,
    //     AABBox {
    //         min: DVec3::new(-10.0, -10.0, -10.0),
    //         max: DVec3::new(10.0, 10.0, 10.0),
    //     },
    // );
    // info!("Octree construction done");

    info!("Constructing KD Tree...");
    let space_partition =
        &KdTree::from_objects(scene.objects.iter().map(|box_obj| box_obj.as_ref()));
    info!("KD Tree construction done");
    // print_kd_tree(space_partition, &mut stdout());

    let _ = crossbeam::scope(|scope| {
        for (thread_num, chunk) in image.chunks_mut(y_block_size * x_res).enumerate() {
            let pixels_counter = pixels_rendered.clone();
            scope
                .builder()
                .name(format!("tracer#{}", thread_num))
                .spawn(move |_| {
                    info!("Started thread {}", std::thread::current().name().unwrap());
                    for y in 0..y_block_size {
                        let mut curr = 0;
                        for x in 0..x_res {
                            let abs_x = x;
                            let abs_y = thread_num * y_block_size + y;
                            info!("Tracing pixel ({}, {})", abs_x, abs_y);
                            for _ in 0..samples_per_pixel {
                                let ray = scene.camera.ray((abs_x, abs_y), (x_res, y_res));

                                chunk[y * x_res + x] += (1.0 / samples_per_pixel as f64)
                                    * trace_ray(
                                        ray,
                                        space_partition,
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
                    info!("Stopped thread {}", std::thread::current().name().unwrap());
                })
                .unwrap();
        }
    });
}

#[inline]
pub fn trace_ray(
    ray: Ray,
    objects: &dyn Intersect,
    lights: &[Sphere],
    remaining_steps: usize,
) -> f64 {
    debug!("tracing ray {:?}", ray);

    // find intersections
    let mut vec = vec![];

    if let Some(reflection_normal) = objects.intersect(ray) {
        vec.push((
            ray.origin.distance_squared(reflection_normal.origin),
            reflection_normal,
            false,
        ));
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

    let nearest = vec.iter().min_by(|(dist_a, _, _), (dist_b, _, _)| {
        dist_a
            .partial_cmp(dist_b)
            .unwrap_or(std::cmp::Ordering::Less)
    });

    // calculate light intensity
    if let Some((_, normal, true)) = nearest {
        debug!("ray reaches a light source at {}", normal.origin);
        1.0
    } else if let Some((_, normal, false)) = nearest {
        debug!("ray reaches an object at {}", normal.origin);
        if remaining_steps > 1 {
            // calculate reflected ray
            let reflected = ray.reflect(*normal);
            debug!("reflected ray is {:?}", reflected);
            debug!("{} remaining steps", remaining_steps - 1);
            0.98 * trace_ray(reflected, objects, lights, remaining_steps - 1)
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
    use glam::DVec3;
    use log::debug;

    use crate::{
        geometry::{AABBox, Intersect, Ray},
        object::{sphere::Sphere, triangle::Triangle},
        space_partition::octree::Octree,
        tracer::trace_ray,
    };

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
        let octree = Octree::new(
            &scene
                .iter()
                .map(|obj_box| obj_box.as_ref())
                .collect::<Vec<_>>(),
            2,
            2,
            AABBox {
                min: DVec3::new(-10.0, -10.0, -10.0),
                max: DVec3::new(10.0, 10.0, 10.0),
            },
        );

        debug!("{:?}", trace_ray(ray, &octree, &lights, 10));
    }
}

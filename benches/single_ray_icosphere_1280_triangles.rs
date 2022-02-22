use criterion::{criterion_group, criterion_main, Criterion, black_box};
use glam::DVec3;
use ray_tracer::{geometry::{Ray, AABBox}, object::import_from_wavefront_obj_file, tracer::trace_ray, octree::Octree};

pub fn single_ray_icosphere_1280_triangles(c: &mut Criterion) {
    let icosphere = import_from_wavefront_obj_file("./icosphere.obj");
    let ray = Ray::from_to((0.0, 0.0, 10.0), (0.0, 0.0, 0.0));
    let max_steps = 10;

    let octree = &Octree::new(
        &icosphere
            .iter()
            .map(|obj_box| obj_box.as_ref())
            .collect(),
        10,
        16,
        AABBox {
            min: DVec3::new(-10.0, -10.0, -10.0),
            max: DVec3::new(10.0, 10.0, 10.0),
        },
    );

    c.bench_function("single ray on icosphere 1280 triangles", |b| {
        b.iter(|| trace_ray(black_box(ray), octree, &[], max_steps))
    });
}

criterion_group!(benches, single_ray_icosphere_1280_triangles);
criterion_main!(benches);

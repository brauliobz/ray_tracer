pub mod camera;
pub mod geometry;
pub mod object;
pub mod scene;
pub mod tracer;
pub mod octree;
pub mod point_light;

#[cfg(feature = "f32")]
pub type Float = f32;
#[cfg(feature = "f64")]
pub type Float = f64;

#[cfg(feature = "f32")]
use std::f32 as float;
#[cfg(feature = "f64")]
use std::f64 as float;

#[cfg(feature = "f32")]
pub type Vec3 = glam::Vec3;
#[cfg(feature = "f64")]
pub type Vec3 = glam::DVec3;

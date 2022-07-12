use crate::geometry::Intersect;

pub mod kdtree;
pub mod octree;

pub trait SpacePartition<'objects, I>: Intersect
where
    Self: 'objects,
    I: Iterator<Item = &'objects dyn Intersect> + Clone,
{
    fn from_objects(objects: I) -> Self;
}

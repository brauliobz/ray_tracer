use std::cmp::Ordering;

use crate::geometry::{AABBox, Intersect, Ray};

use super::SpacePartition;

#[derive(Debug)]
pub struct Octree<'objects> {
    root: Option<Octant<'objects>>,
}

#[derive(Debug, Default)]
pub struct Octant<'objects> {
    bbox: AABBox,
    children: [Option<Box<Octant<'objects>>>; 8],
    objects: Vec<&'objects dyn Intersect>,
}

impl<'objects> Octree<'objects> {
    pub fn new(
        objects: &[&'objects dyn Intersect],
        max_depth: usize,
        max_objects_in_leaf: usize,
        bbox: AABBox,
    ) -> Octree<'objects> {
        Octree {
            root: Some(Octant::new(
                bbox,
                objects,
                max_depth,
                1,
                max_objects_in_leaf,
            )),
        }
    }
}

impl<'objects> Octant<'objects> {
    fn new(
        bbox: AABBox,
        objects: &[&'objects dyn Intersect],
        // 1-based
        max_depth: usize,
        cur_depth: usize,
        max_objects_in_leaf: usize,
    ) -> Octant<'objects> {
        if cur_depth == max_depth || objects.len() <= max_objects_in_leaf {
            return Octant {
                bbox,
                children: [None, None, None, None, None, None, None, None],
                objects: objects.to_vec(),
            };
        }

        let bboxes = bbox.octants();
        let mut objects_in_child = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        let mut children = [None, None, None, None, None, None, None, None];

        for obj in objects {
            for i in 0..8 {
                if bboxes[i].intersect_other(&obj.bounds()) {
                    objects_in_child[i].push(*obj);
                }
            }
        }

        for i in 0..8 {
            if !objects_in_child[i].is_empty() {
                children[i] = Some(Box::new(Octant::new(
                    bboxes[i],
                    &objects_in_child[i],
                    max_depth,
                    cur_depth + 1,
                    max_objects_in_leaf,
                )));
            }
        }

        Octant {
            bbox,
            children,
            objects: vec![],
        }
    }
}

impl<'objects> Intersect for Octree<'objects> {
    fn intersect(&self, ray: Ray) -> Option<Ray> {
        self.root
            .as_ref()
            .map(|octant| octant.intersect(ray))
            .unwrap()
    }

    fn bounds(&self) -> AABBox {
        self.root.as_ref().map(|oct| oct.bbox).unwrap_or_default()
    }
}

impl<'objects> Intersect for Octant<'objects> {
    fn intersect(&self, ray: Ray) -> Option<Ray> {
        self.bbox.intersect(ray)?;

        let child_intersects = self
            .children
            .iter()
            .filter_map(|child| child.as_ref())
            .filter_map(|child| child.intersect(ray));

        let object_intersects = self.objects.iter().filter_map(|obj| obj.intersect(ray));

        // get nearest
        child_intersects.chain(object_intersects).min_by(|a, b| {
            let dist_a = ray.origin.distance_squared(a.origin);
            let dist_b = ray.origin.distance_squared(b.origin);
            if dist_a < dist_b {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    }

    fn bounds(&self) -> AABBox {
        self.bbox
    }
}

// impl<'objects> SpacePartition<'objects> for Octree<'objects> {
//     fn from_objects(objects: &'objects [&'objects dyn Intersect]) -> Self {
//         todo!()
//     }
// }

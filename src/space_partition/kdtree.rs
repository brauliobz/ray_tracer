use crate::geometry::{AABBox, Intersect, Ray};

use super::SpacePartition;

const MIN_OBJECTS_IN_LEAF: usize = 32;

/// Tree where every non-leaf divides the space into two regions
/// using an axis-aligned plane
#[derive(Debug)]
pub struct KdTree<'objects> {
    pub(crate) root: Node,
    pub(crate) objects: Vec<&'objects dyn Intersect>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Node {
    /// Partitioned node
    Branch(AABBox, Box<Node>, Box<Node>),
    /// Leaf with objects (vec of indexes)
    Leaf(AABBox, Vec<usize>),
}

impl<'objects, I> SpacePartition<'objects, I> for KdTree<'objects>
where
    I: Iterator<Item = &'objects dyn Intersect> + Clone,
{
    fn from_objects(objects: I) -> KdTree<'objects> {
        let all_objects = objects.collect::<Vec<_>>();
        let root = build_node(
            &all_objects,
            &(0..all_objects.len()).into_iter().collect::<Vec<_>>(),
            0,
        );

        KdTree {
            root,
            objects: all_objects,
        }
    }
}

fn build_node<'objects>(
    all_objects: &[&'objects dyn Intersect],
    object_idxs: &[usize],
    current_axis: usize,
) -> Node {
    let bbox = object_idxs
        .iter()
        .fold(all_objects[object_idxs[0]].bounds(), |bbox, &idx| {
            bbox.merge(&all_objects[idx].bounds())
        });

    if object_idxs.len() < MIN_OBJECTS_IN_LEAF {
        return Node::Leaf(bbox, object_idxs.to_vec());
    }

    let median = {
        let mut vec = Vec::from_iter(
            object_idxs
                .iter()
                .map(|&idx| all_objects[idx].bounds().min[current_axis])
                .chain(
                    object_idxs
                        .iter()
                        .map(|&idx| all_objects[idx].bounds().max[current_axis]),
                ),
        );
        let mid = vec.len() / 2;
        vec.select_nth_unstable_by(mid, f64::total_cmp);
        vec[mid]
    };

    // partition into two vectors

    let left_objs = object_idxs
        .iter()
        .copied()
        .filter(|&idx| all_objects[idx].bounds().min[current_axis] <= median)
        .collect::<Vec<_>>();
    let right_objs = object_idxs
        .iter()
        .copied()
        .filter(|&idx| all_objects[idx].bounds().max[current_axis] >= median)
        .collect::<Vec<_>>();

    // create left and right nodes recursively

    let left = Box::new(build_node(all_objects, &left_objs, (current_axis + 1) % 3));
    let right = Box::new(build_node(all_objects, &right_objs, (current_axis + 1) % 3));

    Node::Branch(bbox, left, right)
}

impl<'objects> Intersect for KdTree<'objects> {
    fn intersect(&self, ray: Ray) -> Option<Ray> {
        self.root.intersect(ray, 0, &self.objects)
    }

    fn bounds(&self) -> AABBox {
        self.root.bounds()
    }
}

impl Node {
    fn intersect(
        &self,
        ray: Ray,
        current_axis: usize,
        all_objects: &[&dyn Intersect],
    ) -> Option<Ray> {
        self.bounds().intersect(ray)?;

        match &self {
            Node::Branch(_, left, right) => {
                let left_intersect = left.intersect(ray, (current_axis + 1) % 3, all_objects);
                let right_intersect = right.intersect(ray, (current_axis + 1) % 3, all_objects);

                // nearest between left and right, if any exists
                match (left_intersect, right_intersect) {
                    (Some(left), Some(right)) => Some(
                        if ray.origin.distance_squared(left.origin)
                            < ray.origin.distance_squared(left.origin)
                        {
                            left
                        } else {
                            right
                        },
                    ),
                    (left, None) => left,
                    (None, right) => right,
                }
            }
            Node::Leaf(_, objects) => {
                // nearest intersection
                objects
                    .iter()
                    .filter_map(|idx| all_objects[*idx].intersect(ray))
                    .min_by(|a, b| {
                        ray.origin
                            .distance_squared(a.origin)
                            .total_cmp(&ray.origin.distance_squared(b.origin))
                    })
            }
        }
    }

    fn bounds(&self) -> AABBox {
        match self {
            Node::Branch(bbox, _, _) => *bbox,
            Node::Leaf(bbox, _) => *bbox,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::object::sphere::Sphere;

    use super::*;

    #[test]
    fn test_single_object() {
        let sphere = Sphere::new((0.0, 0.0, 0.0), 1.0);
        let objects: Vec<&dyn Intersect> = vec![&sphere];
        let tree = KdTree::from_objects(objects.iter().copied());

        assert_eq!(
            tree.root,
            Node::Leaf(
                AABBox::new((-1.0, -1.0, -1.0).into(), (1.0, 1.0, 1.0).into()),
                vec![0]
            )
        );
    }

    #[test]
    fn test_two_objects() {
        let sphere_a = Sphere::new((0.0, 0.0, 0.0), 1.0);
        let sphere_b = Sphere::new((2.0, 0.0, 0.0), 1.0);
        let objects: Vec<&dyn Intersect> = vec![&sphere_a, &sphere_b];
        let tree = KdTree::from_objects(objects.iter().copied());

        assert_eq!(
            tree.root,
            Node::Leaf(
                AABBox::new((-1.0, -1.0, -1.0).into(), (3.0, 1.0, 1.0).into()),
                vec![0, 1]
            )
        );
    }
}

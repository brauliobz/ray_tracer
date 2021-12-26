use std::{
    fs::File,
    io::{BufReader, Read},
};

use wavefront_obj::obj::Primitive;

use crate::{geometry::Intersect, object::triangle::Triangle};

pub mod sphere;
pub mod triangle;

pub fn import_from_wavefront_obj_file(path: &str) -> Vec<Box<dyn Intersect>> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let parsed_obj = wavefront_obj::obj::parse(content.as_str()).unwrap();

    let mut triangles: Vec<Box<dyn Intersect>> = Vec::new();

    for obj in parsed_obj.objects {
        for geom in obj.geometry {
            for shape in geom.shapes {
                if let Primitive::Triangle(a, b, c, ..) = shape.primitive {
                    triangles.push(Box::new(Triangle::from_tuples(
                        (
                            obj.vertices[a.0].x,
                            obj.vertices[a.0].y,
                            obj.vertices[a.0].z,
                        ),
                        (
                            obj.vertices[b.0].x,
                            obj.vertices[b.0].y,
                            obj.vertices[b.0].z,
                        ),
                        (
                            obj.vertices[c.0].x,
                            obj.vertices[c.0].y,
                            obj.vertices[c.0].z,
                        ),
                    )));
                }
            }
        }
    }

    triangles
}

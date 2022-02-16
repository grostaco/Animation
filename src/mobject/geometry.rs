use nannou::prelude::Point3;
use palette::{named, Srgb};

use super::{constants::*, mobject::Mobject, vectorized_mobject::VMobject};

pub struct Rectangle {
    color: Srgb<u8>,
    points: Vec<Point3>,
}

trait Polygram: VMobject {
    fn add_vertices(&mut self, vertices: &[Point3]) {
        let (first_vertex, vertices) = vertices.split_at(1);
        self.start_new_path(first_vertex.first().unwrap());
        self.add_points_as_corners(vertices.iter().chain(first_vertex.iter()));
    }
}

trait Polygon: Polygram {}

impl Mobject for Rectangle {
    fn new() -> Self {
        let mut rect = Self {
            color: named::from_str("olive").unwrap(),
            points: Vec::new(),
        };
        rect.add_vertices(&[
            Point3::new(1., 1., 0.),
            Point3::new(-1., 1., 0.),
            Point3::new(-1., -1., 0.),
            Point3::new(1., -1., 0.),
        ]);

        rect
    }
    fn points(&self) -> &Vec<Point3> {
        &self.points
    }

    fn points_mut(&mut self) -> &mut Vec<Point3> {
        &mut self.points
    }
}

impl VMobject for Rectangle {}
impl Polygram for Rectangle {}

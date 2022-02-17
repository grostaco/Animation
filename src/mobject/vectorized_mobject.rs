use itertools_num::linspace;

use super::{constants, integer_interpolate, mobject::Mobject, Point3};

const N_POINTS_PER_CUBIC_CURVE: usize = 4;

pub trait VMobject: Mobject {
    fn add_line_to(&mut self, point: &Point3) {
        let last_point = self.get_last_point().unwrap().clone();
        let mut l = linspace(0., 1., N_POINTS_PER_CUBIC_CURVE)
            .skip(1)
            .map(|i| last_point.lerp(point, i))
            .take(3);
        self.add_cubic_bezier_curve_to(&l.next().unwrap(), &l.next().unwrap(), &l.next().unwrap());
    }
    fn get_last_point(&self) -> Option<&Point3> {
        self.points().last()
    }
    fn append_points(&mut self, points: &[&Point3]) {
        self.points_mut().extend(points.iter().cloned());
    }
    fn add_cubic_bezier_curve_to(&mut self, handle1: &Point3, handle2: &Point3, anchor: &Point3) {
        let new_points = [
            &self.get_last_point().cloned().unwrap_or(constants::ZERO),
            handle1,
            handle2,
            anchor,
        ];
        if self.has_new_path_started() {
            self.append_points(&new_points[1..])
        } else {
            self.append_points(&new_points)
        }
    }
    fn add_points_as_corners<'a, I>(&mut self, points: I)
    where
        I: Iterator<Item = &'a Point3>,
    {
        for point in points {
            self.add_line_to(point);
        }
    }
    fn has_new_path_started(&self) -> bool {
        self.points().len() % N_POINTS_PER_CUBIC_CURVE == 1
    }
    fn start_new_path(&mut self, point: &Point3) -> &mut Self {
        self.append_points(&[point]);
        self
    }
    fn get_start_anchors(&self) -> Vec<&Point3> {
        self.points().iter().step_by(4).collect()
    }

    #[inline]
    fn get_cubic_bezier_tuples_from_points(points: &Vec<Point3>) -> Vec<&[Point3]> {
        points.chunks(4).collect()
    }
    fn get_cubic_bezier_tuples(&self) -> Vec<&[Point3]> {
        Self::get_cubic_bezier_tuples_from_points(self.points())
    }

    fn clear_points(&mut self) {
        *self.points_mut() = vec![constants::ZERO];
    }

    fn pointwise_become_partial(&mut self, vmobject: impl VMobject, a: f32, b: f32) -> &mut Self {
        if a <= 0. && b >= 1. {
            *self.points_mut() = vmobject.points().clone();
            return self;
        }

        let bezier_quads = vmobject.get_cubic_bezier_tuples();
        let num_cubics = bezier_quads.len() as f32;

        let (lower_index, lower_residule) = integer_interpolate(0., num_cubics, a);
        let (upper_index, upper_residule) = integer_interpolate(0., num_cubics, b);

        self.clear_points();

        if num_cubics == 0. {
            return self;
        }

        if lower_index == upper_index {}
        self
    }
}

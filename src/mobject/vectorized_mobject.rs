use itertools_num::linspace;

use super::{constants, integer_interpolate, mobject::Mobject, partial_bezier_points, Point3};

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
    fn append_points<'a, T>(&mut self, points: T)
    where
        T: IntoIterator<Item = &'a Point3>,
    {
        self.points_mut().extend(points);
    }
    fn add_cubic_bezier_curve_to(&mut self, handle1: &Point3, handle2: &Point3, anchor: &Point3) {
        let new_points = [
            &self.get_last_point().cloned().unwrap_or(constants::ZERO),
            handle1,
            handle2,
            anchor,
        ];
        if self.has_new_path_started() {
            self.append_points(new_points.into_iter().skip(1))
        } else {
            self.append_points(new_points.into_iter())
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
        self.append_points([point.clone()].iter());
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
        self.points_mut().clear()
    }

    fn pointwise_become_partial(&mut self, vmobject: impl VMobject, a: f32, b: f32) -> &mut Self {
        if a <= 0. && b >= 1. {
            *self.points_mut() = vmobject.points().clone();
            return self;
        }

        let bezier_quads = vmobject.get_cubic_bezier_tuples();
        let num_cubics = bezier_quads.len() as f32;

        let (lower_index, lower_residue) = integer_interpolate(0., num_cubics, a);
        let (upper_index, upper_residue) = integer_interpolate(0., num_cubics, b);

        self.clear_points();

        if num_cubics == 0. {
            return self;
        }

        /*

        self.append_points(
                partial_bezier_points(bezier_quads[lower_index], lower_residue, 1),
            )
            for quad in bezier_quads[lower_index + 1 : upper_index]:
                self.append_points(quad)
            self.append_points(
                partial_bezier_points(bezier_quads[upper_index], 0, upper_residue),
            )
            */

        if lower_index == upper_index {
            self.points_mut()
                .extend(bezier_quads[lower_index as usize].iter());
        } else {
            self.points_mut().extend(partial_bezier_points(
                bezier_quads[lower_index as usize],
                lower_residue,
                1.,
            ));
            for quad in bezier_quads[lower_index as usize + 1..upper_index as usize].iter() {
                self.points_mut().extend(quad.iter());
            }
            self.points_mut().extend(partial_bezier_points(
                bezier_quads[upper_index as usize],
                0.,
                upper_residue,
            ));
        }
        self
    }
}

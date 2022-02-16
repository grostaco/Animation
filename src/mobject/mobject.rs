use std::cmp::Ordering;

use itertools::{Itertools, MinMaxResult};
use nannou::prelude::Point3;

pub trait Mobject {
    fn new() -> Self;
    fn points(&self) -> &Vec<Point3>;
    fn points_mut(&mut self) -> &mut Vec<Point3>;

    fn stretch(
        &mut self,
        factor: f32,
        dim: usize,
        about_point: Option<Point3>,
        about_edge: Option<Point3>,
    ) {
        let func = |points: &mut Vec<Point3>| {
            for point in points {
                point[dim] *= factor;
            }
        };
        self.apply_points_function_about_point(func, about_point, about_edge)
    }

    fn scale(
        &mut self,
        scale_factor: f32,
        about_point: Option<Point3>,
        about_edge: Option<Point3>,
    ) {
        let func = |points: &mut Vec<Point3>| {
            for point in points {
                *point *= scale_factor;
            }
        };
        self.apply_points_function_about_point(func, about_point, about_edge)
    }

    fn apply_points_function_about_point<F>(
        &mut self,
        f: F,
        about_point: Option<Point3>,
        about_edge: Option<Point3>,
    ) where
        F: FnOnce(&mut Vec<Point3>),
    {
        let about_edge = about_edge.unwrap_or(Point3::ZERO);
        let about_point = about_point.unwrap_or(self.get_critical_point(about_edge));

        let mut points = self.points_mut();
        for point in points.iter_mut() {
            *point -= about_point;
        }

        f(&mut points);

        for point in points.iter_mut() {
            *point += about_point;
        }
    }

    fn get_extremum_along_dim(&self, points: Option<&Vec<Point3>>, dim: usize, key: f32) -> f32 {
        let points = points.unwrap_or(self.points());
        let values = points.iter().map(|v| v[dim]);

        match key.partial_cmp(&0.).unwrap() {
            Ordering::Less => values.min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
            Ordering::Equal => {
                if let MinMaxResult::MinMax(a, b) = values.minmax() {
                    (a + b) / 2.
                } else {
                    panic!("Cannot get extremum")
                }
            }
            _ => values.max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
        }
    }
    fn get_critical_point(&self, direction: Point3) -> Point3 {
        let mut result = Point3::ZERO.clone();
        let all_points = self.points();
        if all_points.len() != 0 {
            for dim in 0..3 {
                result[dim] = self.get_extremum_along_dim(Some(all_points), dim, direction[dim])
            }
        }
        result
    }
}

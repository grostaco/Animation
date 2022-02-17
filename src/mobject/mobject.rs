use std::{cmp::Ordering, ops::IndexMut};

use super::{constants, Point3};
use itertools::{Itertools, MinMaxResult};

pub trait Mobject {
    fn points(&self) -> &Vec<Point3>;
    fn points_mut(&mut self) -> &mut Vec<Point3>;

    fn stretch(
        &mut self,
        factor: f32,
        dim: usize,
        about_point: Option<Point3>,
        about_edge: Option<Point3>,
    ) {
        self.apply_points_function_about_point(
            |points| {
                for point in points {
                    let v = point.index_mut(dim);
                    *v *= factor;
                }
            },
            about_point,
            about_edge,
        )
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

    fn reduce_across_dim<F, FO, G, GO>(
        &self,
        points_func: F,
        reduce_func: G,
        dim: usize,
    ) -> Option<GO>
    where
        FO: std::fmt::Debug,
        F: Fn(&Vec<f32>) -> Option<FO>,
        G: Fn(FO) -> Option<GO>,
    {
        let points = points_func(&Point3::get_along_dim(self.points(), dim));

        if points.is_none() {
            return None;
        }
        reduce_func(points.unwrap())
    }

    fn length_over_dim(&self, dim: usize) -> f32 {
        self.reduce_across_dim(
            |x| x.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).cloned(),
            |y| Some(y),
            dim,
        )
        .unwrap()
            - self
                .reduce_across_dim(
                    |x| x.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).cloned(),
                    |y| Some(y),
                    dim,
                )
                .unwrap()
    }

    fn rescale_to_fit(&mut self, length: f32, dim: usize, stretch: bool) -> &Self {
        let old_length = self.length_over_dim(dim);
        if old_length != 0. {
            if stretch {
                self.stretch(length / old_length, dim, None, None);
            } else {
                self.scale(length / old_length, None, None);
            }
        }
        self
    }

    fn stretch_to_fit_height(&mut self, height: f32) -> &Self {
        self.rescale_to_fit(height, 1, true)
    }

    fn stretch_to_fit_width(&mut self, width: f32) -> &Self {
        self.rescale_to_fit(width, 0, true)
    }

    fn apply_points_function_about_point<F>(
        &mut self,
        f: F,
        about_point: Option<Point3>,
        about_edge: Option<Point3>,
    ) where
        F: FnOnce(&mut Vec<Point3>),
    {
        let about_edge = about_edge.unwrap_or(constants::ZERO);
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
        let mut result = constants::ZERO.clone();
        let all_points = self.points();
        if all_points.len() != 0 {
            for dim in 0..3 {
                result[dim] = self.get_extremum_along_dim(Some(all_points), dim, direction[dim])
            }
        }
        result
    }
}

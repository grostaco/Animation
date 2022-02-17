use super::{choose, Point3};
use std::ops::Add;

#[inline]
pub fn interpolate(start: f32, end: f32, alpha: f32) -> f32 {
    start * (1. - alpha) + end * alpha
}

#[inline]
pub fn integer_interpolate(start: f32, end: f32, alpha: f32) -> (u64, f32) {
    if alpha >= 1. {
        (end as u64 - 1, 1.)
    } else if alpha <= 0. {
        (start as u64, 0.)
    } else {
        let value = interpolate(start, end, alpha) as u64;
        let residue = ((end - start) * alpha) % 1.;

        (value, residue)
    }
}

pub fn bezier<'a>(points: &'a [Point3]) -> impl FnOnce(f32) -> Point3 + 'a {
    let n = points.len() - 1;

    move |t: f32| {
        points
            .iter()
            .enumerate()
            .map(move |(k, point)| {
                point
                    * (1. - t).powi((n - k) as i32)
                    * t.powi(k as i32)
                    * choose(n as u64, k as u64, true) as f32
            })
            .reduce(Point3::add)
            .unwrap()
    }
}

pub fn partial_bezier_points(points: &[Point3], a: f32, b: f32) -> Vec<Point3> {
    if a == 1. {
        vec![points.last().unwrap().clone(); points.len()]
    } else {
        let a_to_1 = (0..points.len())
            .map(|i| bezier(&points[i..])(a))
            .collect::<Vec<_>>();
        let end_prop = (b - a) / (1. - a);

        (0..points.len())
            .map(|i| bezier(&a_to_1[..i + 1])(end_prop))
            .collect()
    }
}

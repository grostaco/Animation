use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::interpolate;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);
pub type Point3 = Vec3;

macro_rules! ops_impl_for {
    ($op:ty => $block:tt, $t:ty) => {
        impl $op for $t $block
    };
    ($op:ty => $block:tt, $t:ty, $($ts:ty),+) => {
        impl $op for $t $block
        ops_impl_for!($op => $block, $($ts),* );
    }
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self(e0, e1, e2)
    }

    pub fn lerp(&self, other: &Self, alpha: f32) -> Self {
        Self(
            interpolate(self.0, other.0, alpha),
            interpolate(self.1, other.1, alpha),
            interpolate(self.2, other.2, alpha),
        )
    }

    pub fn get_along_dim(vecs: &Vec<Point3>, dim: usize) -> Vec<f32> {
        vecs.iter().map(|v| v[dim]).collect()
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}
impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

ops_impl_for!(Neg => {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}, Vec3, &Vec3);

ops_impl_for!(Mul<f32> => {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}, Vec3, &Vec3);

ops_impl_for!(MulAssign<f32> => {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}, Vec3, &mut Vec3);

ops_impl_for!(Mul => {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}, Vec3, &Vec3);

ops_impl_for!(Div<f32> => {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}, Vec3, &Vec3);

ops_impl_for!(Add => {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)    
    }
}, Vec3, &Vec3);

ops_impl_for!(AddAssign => {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}, Vec3, &mut Vec3);

ops_impl_for!(Sub => {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)    
    }
}, Vec3, &Vec3);

ops_impl_for!(SubAssign => {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}, Vec3, &mut Vec3);

ops_impl_for!(Index<usize> => {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Vec3 index must be in [0, 3)"),
        }
    }
}, Vec3, &Vec3, &mut Vec3);

ops_impl_for!(IndexMut<usize> => {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Vec3 index must be in [0, 3)"),
        }
    }
}, Vec3, &mut Vec3);

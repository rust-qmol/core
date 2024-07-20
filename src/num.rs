pub mod one;
pub mod zero;

pub mod convert;

pub mod complex;
pub mod float;

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::num::{complex::Complex, one::One, zero::Zero};

pub trait Num
where
    Self: Zero + One,
    Self: Debug + Copy,
    Self: Neg<Output = Self>,
    Self: Add<Output = Self> + AddAssign,
    Self: Sub<Output = Self> + SubAssign,
    Self: Div<Output = Self> + DivAssign,
    Self: Mul<Output = Self> + MulAssign,
{
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
}

impl Num for f64 {
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}
impl Num for f32 {
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}
impl Num for Complex<f64> {
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}
impl Num for Complex<f32> {
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}

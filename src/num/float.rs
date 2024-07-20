use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

use super::{one::One, zero::Zero};

pub trait Float
where
    Self: Display + Debug,
    Self: Copy,
    Self: FromStr,
    Self: From<f32>,
    Self: From<u8> + From<u16>,
    Self: From<i8> + From<i16>,
    Self: Zero + One,
    Self: Neg<Output = Self>,
    Self: Add<Output = Self> + AddAssign,
    Self: Sub<Output = Self> + SubAssign,
    Self: Div<Output = Self> + DivAssign,
    Self: Mul<Output = Self> + MulAssign,
    Self: PartialOrd,
    Self: Sum,
{
    fn sqrt(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

macro_rules! float_func_impl {
    ($T:ident) => {
        impl Float for $T {
            fn sqrt(self) -> Self {
                Self::sqrt(self)
            }
            fn powi(self, n: i32) -> Self {
                Self::powi(self, n)
            }
            fn sin(self) -> Self {
                Self::sin(self)
            }
            fn cos(self) -> Self {
                Self::cos(self)
            }
        }
    };
}

float_func_impl!(f32);
float_func_impl!(f64);

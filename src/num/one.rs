use std::ops::{Add, Mul, Neg};

use super::{complex::Complex, float::Float};

pub trait One:
    Sized + Add<Output = Self> + Mul<Output = Self> + Mul<Self, Output = Self> + Neg<Output = Self>
{
    fn one() -> Self;
    fn is_one(&self) -> bool;
    fn set_one(&mut self) {
        *self = One::one();
    }
}
impl One for f32 {
    fn one() -> Self {
        1.0
    }
    fn is_one(&self) -> bool {
        *self == 1.0
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
    fn is_one(&self) -> bool {
        *self == 1.0
    }
}

impl<T: Float> One for Complex<T> {
    fn one() -> Self {
        Self {
            re: T::one(),
            im: T::zero(),
        }
    }
    fn is_one(&self) -> bool {
        self.re.is_one() && self.im.is_zero()
    }

    fn set_one(&mut self) {
        *self = One::one();
    }
}

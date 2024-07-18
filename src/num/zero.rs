use std::ops::{Add, Mul};

pub trait Zero:
    Sized + Add<Output = Self> + Mul<Output = Self> + Mul<Self, Output = Self>
{
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
}
impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}
impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

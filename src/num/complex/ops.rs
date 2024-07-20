use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::{super::float::Float, Complex};

macro_rules! complex_ops_raw_impl {
    ($vec_name:ident, $bound_assign:ident, $method_assign:ident, $bound:ident, $method:ident) => {
        impl<T: Float> $bound_assign for $vec_name<T> {
            fn $method_assign(&mut self, rhs: Self) {
                self.re = $bound::$method(self.re, rhs.re);
                self.im = $bound::$method(self.im, rhs.im);
            }
        }

        impl<T: Float> $bound for $vec_name<T> {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self {
                let mut r = self;
                $bound_assign::$method_assign(&mut r, rhs);
                r
            }
        }
    };
}

macro_rules! complex_mul_div_impl {
    ($vec_name:ident) => {
        impl<T: Float> MulAssign for $vec_name<T> {
            fn mul_assign(&mut self, rhs: Self) {
                let re = self.re * rhs.re - self.im * rhs.im;
                let im = self.re * rhs.im + self.im * rhs.re;
                self.re = re;
                self.im = im;
            }
        }

        impl<T: Float> Mul for $vec_name<T> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut r = self;
                r *= rhs;
                r
            }
        }

        impl<T: Float> DivAssign for $vec_name<T> {
            fn div_assign(&mut self, rhs: Self) {
                let nrm2 = rhs.nrm2();
                *self *= rhs.conj();
                self.re /= nrm2;
                self.im /= nrm2;
            }
        }

        impl<T: Float> Div for $vec_name<T> {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                let mut r = self;
                r /= rhs;
                r
            }
        }
    };
}

macro_rules! complex_ops_impl {
    ($vec_name:ident) => {
        complex_ops_raw_impl!($vec_name, AddAssign, add_assign, Add, add);
        complex_ops_raw_impl!($vec_name, SubAssign, sub_assign, Sub, sub);
        complex_mul_div_impl!($vec_name);
    };
}

complex_ops_impl!(Complex);

impl<T: Float> Neg for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

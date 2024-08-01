use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

use super::{one::One, zero::Zero};

macro_rules! float_func_impl {
    (
        ($($fn_name_self: ident),*)
    , ($(($fn_name_other: ident, $(($para: ident, $para_type: ident)),*)),*)
) => {
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
            fn sin_cos(self) -> (Self, Self);
            $(
                fn $fn_name_self(self) -> Self;
            )*
            $(
                fn $fn_name_other(self,
                    $(
                        $para: $para_type,
                    )*
                ) -> Self;
            )*
        }

        macro_rules! float_func_raw_impl {
            ($T:ident) => {
                impl Float for $T {
                    fn sin_cos(self) -> (Self, Self){
                        Self::sin_cos(self)
                    }
                    $(
                        fn $fn_name_self(self) -> Self {
                            Self::$fn_name_self(self)
                        }
                    )*
                    $(
                        fn $fn_name_other(self,
                            $(
                                $para: $para_type,
                            )*
                        ) -> Self{
                            Self::$fn_name_other(self,
                                $(
                                    $para,
                                )*
                            )
                        }
                    )*
                }
            };
        }

        float_func_raw_impl!(f32);
        float_func_raw_impl!(f64);
    };
}

float_func_impl!(
    (
        floor, ceil, round, trunc, fract, abs, signum, sqrt, exp, exp2, ln, log2, log10, cbrt, sin,
        cos, tan, asin, acos, atan, exp_m1, ln_1p, sinh, cosh, tanh, asinh, acosh, atanh
    ),
    (
        (copysign, (sign, Self)),
        (div_euclid, (rhs, Self)),
        (rem_euclid, (rhs, Self)),
        (powi, (n, i32)),
        (powf, (n, Self)),
        (log, (base, Self)),
        (hypot, (other, Self)),
        (atan2, (other, Self)),
        (mul_add, (a, Self), (b, Self))
    )
);

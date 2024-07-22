use super::float::Float;

mod ops;

pub type ComplexF32 = Complex<f32>;
pub type ComplexF64 = Complex<f64>;

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct Complex<T: Float> {
    pub re: T,
    pub im: T,
}

impl<T: Float> Complex<T> {
    pub fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
    pub fn inv(self) -> Self {
        let nrm2 = self.nrm2();
        Self {
            re: self.re / nrm2,
            im: self.im / nrm2,
        }
    }
    pub fn nrm2(self) -> T {
        self.re.powi(2) + self.im.powi(2)
    }
    pub fn max(self, other: Self) -> Self {
        let a_sum = self.re + self.im;
        let b_sum = other.re + other.im;
        if a_sum > b_sum {
            self
        } else {
            other
        }
    }
    pub fn min(self, other: Self) -> Self {
        let a_sum = self.re + self.im;
        let b_sum = other.re + other.im;
        if a_sum < b_sum {
            self
        } else {
            other
        }
    }
}

use super::float::Float;

pub fn autoaa<T: Float>(x: T) -> T {
    x * 0.52917726.into()
}

pub fn aatoau<T: Float>(x: T) -> T {
    x * (1.0 / 0.52917726).into()
}

pub const AMUTOKG: f64 = 1.660539040e-27;
pub const KGTOAMU: f64 = 1.0 / AMUTOKG;
pub const METOKG: f64 = 9.10938356e-31;
pub const KGTOME: f64 = 1.0 / METOKG;
pub const AMUTOAU: f64 = AMUTOKG * KGTOME;

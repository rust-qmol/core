pub mod pos;
pub mod prop;

use crate::num::float::Float;

pub type AtomCharg = i8;
pub type AtomNuc = u8;
pub type CoordinatesLike<T> = [T; 3];

pub struct Atom<T: Float> {
    pub nuc: AtomNuc,
    pub charg: AtomCharg,
    pub pos: CoordinatesLike<T>,
}

impl<T: Float> Atom<T> {
    pub fn new(nuc: AtomNuc, charg: AtomCharg, pos: CoordinatesLike<T>) -> Self {
        Self { nuc, charg, pos }
    }
    pub fn empty() -> Self {
        Self {
            nuc: 0,
            charg: 0,
            pos: [T::zero(); 3],
        }
    }
}

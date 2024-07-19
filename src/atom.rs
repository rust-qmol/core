mod atm_move;
mod prop;
mod set;

use crate::num::float::Float;


pub type AtomCharg = i8;
pub type AtomNuc = u8;
pub type CoordinatesLike<T> = [T; 3];

pub struct Atom<T: Float> {
    nuc: AtomNuc,
    charg: AtomCharg,
    pos: CoordinatesLike<T>,
}

impl<T: Float> Atom<T> {
    pub fn new(nuc: AtomNuc, charg: AtomCharg, pos: [T; 3]) -> Self {
        Self { nuc, charg, pos }
    }
}

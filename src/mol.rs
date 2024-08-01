mod index;
pub mod pos;
pub mod prop;
mod rotate;

use crate::{
    atom::{Atom, CoordinatesLike},
    num::float::Float,
};

pub type MolCharg = i8;
pub type MolNuc = u16;
pub type BigMolNuc = i32;
pub type Spin = u8;

pub struct Molecule<T: Float> {
    nuc: MolNuc,
    charg: MolCharg,
    spin: Spin,
    pub center: CoordinatesLike<T>,
    pub angle_local: CoordinatesLike<T>,
    atoms: Vec<Atom<T>>,
}

impl<T: Float> Molecule<T> {
    pub fn new(atoms: Vec<Atom<T>>) -> Self {
        Self {
            nuc: atoms.iter().map(|atm| atm.nuc as MolNuc).sum(),
            charg: atoms.iter().map(|atm| atm.charg as MolCharg).sum(),
            spin: 0,
            center: [T::zero(); 3],
            angle_local: [T::zero(); 3],
            atoms,
        }
    }
}

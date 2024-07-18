mod mol_move;
mod ops;
mod prop;
mod rotate;
mod set;

use crate::{atom::Atom, num::Float};

pub type MolCharg = i8;
pub type MolNuc = u16;
pub type BigMolNuc = i32;
pub type Spin = u8;

pub struct Molecule<T: Float> {
    nuc: MolNuc,
    charg: MolCharg,
    spin: Spin,
    center: [T; 3],
    local_angle: [T; 3],
    atoms: Vec<Atom<T>>,
}

impl<T: Float> Molecule<T> {
    pub fn new(atoms: Vec<Atom<T>>) -> Self {
        Self {
            nuc: atoms.iter().map(|atm| atm.nuc() as MolNuc).sum(),
            charg: atoms.iter().map(|atm| atm.charg() as MolCharg).sum(),
            spin: 0,
            center: [T::zero(); 3],
            local_angle: [T::zero(); 3],
            atoms,
        }
    }
}

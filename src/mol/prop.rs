use crate::{
    atom::{Atom, CoordinatesLike},
    num::float::Float,
};

use super::{rotate::rotate, BigMolNuc, MolCharg, MolNuc, Molecule, Spin};

impl<T: Float> Molecule<T> {
    pub fn nuc(&self) -> MolNuc {
        self.nuc
    }
    pub fn charg(&self) -> MolCharg {
        self.charg
    }
    pub fn spin(&self) -> Spin {
        self.spin
    }
    pub fn center(&self) -> [T; 3] {
        self.center
    }

    pub fn tot_ele(&self) -> MolNuc {
        (self.nuc as BigMolNuc - self.charg as BigMolNuc) as MolNuc
    }
    pub fn nele(&self) -> [MolNuc; 2] {
        let tot_ele = self.nuc as BigMolNuc - self.charg as BigMolNuc;
        let nbeta = ((tot_ele - self.spin as BigMolNuc) / 2) as MolNuc;
        [nbeta + self.spin as MolNuc, nbeta]
    }

    pub fn natm(&self) -> usize {
        self.atoms.len()
    }
    pub fn atoms(&self) -> &Vec<Atom<T>> {
        &self.atoms
    }
    pub fn atoms_pos_local(&self) -> Vec<CoordinatesLike<T>> {
        self.atoms.iter().map(|atm| atm.pos()).collect()
    }
    pub fn atoms_pos_globe(&self) -> Vec<CoordinatesLike<T>> {
        rotate(self.atoms_pos_local(), self.local_angle)
            .iter()
            .map(|xyz| {
                [
                    xyz[0] + self.center[0],
                    xyz[1] + self.center[1],
                    xyz[2] + self.center[2],
                ]
            })
            .collect()
    }
}

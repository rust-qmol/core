use crate::{mol::BigMolNuc, num::float::Float};

use super::{MolCharg, Molecule, Spin};

impl<T: Float> Molecule<T> {
    pub fn set_ele_prop(mut self, charg: MolCharg, spin: Spin) -> Self {
        assert_eq!(
            (self.nuc as BigMolNuc - charg as BigMolNuc) % 2,
            spin as BigMolNuc
        );
        self.spin = spin;
        self.charg = charg;
        self
    }
}

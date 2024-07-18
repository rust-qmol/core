use crate::num::Float;

use super::{Atom, AtomCharg, AtomNuc};

impl<T: Float> Atom<T> {
    pub fn set_nuc(&mut self, nuc: AtomNuc) -> &mut Self {
        self.nuc = nuc;
        self
    }
    pub fn set_charg(&mut self, charg: AtomCharg) -> &mut Self {
        self.charg = charg;
        self
    }
}

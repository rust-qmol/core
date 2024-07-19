use crate::{io::PERIODIC_TABLE, num::float::Float};

use super::{Atom, AtomCharg, AtomNuc};

impl<T: Float> Atom<T> {
    pub fn nuc(&self) -> AtomNuc {
        self.nuc
    }
    pub fn charg(&self) -> AtomCharg {
        self.charg
    }
    pub fn pos(&self) -> [T; 3] {
        self.pos
    }

    pub fn symbol(&self) -> &'static str {
        PERIODIC_TABLE[(self.nuc - 1) as usize]
    }
}

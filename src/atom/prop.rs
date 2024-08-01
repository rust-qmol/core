use crate::{io::PERIODIC_TABLE, num::float::Float};

use super::{Atom, AtomCharg, AtomNuc, CoordinatesLike};

pub trait AtomBasicProp<T: Float> {
    fn nuc(&self) -> AtomNuc;
    fn set_nuc(&mut self, nuc: AtomNuc) -> &Self;
    fn charg(&self) -> AtomCharg;
    fn set_charg(&mut self, charg: AtomCharg) -> &Self;
    fn pos(&self) -> CoordinatesLike<T>;
    fn pos_mut(&mut self) -> &mut CoordinatesLike<T>;
    fn symbol(&self) -> &'static str;
}

impl<T: Float> AtomBasicProp<T> for Atom<T> {
    fn nuc(&self) -> AtomNuc {
        self.nuc
    }
    fn set_nuc(&mut self, nuc: AtomNuc) -> &Self {
        self.nuc = nuc;
        self
    }

    fn charg(&self) -> AtomCharg {
        self.charg
    }
    fn set_charg(&mut self, charg: AtomCharg) -> &Self {
        self.charg = charg;
        self
    }

    fn pos(&self) -> CoordinatesLike<T> {
        self.pos
    }
    fn pos_mut(&mut self) -> &mut CoordinatesLike<T> {
        &mut self.pos
    }

    fn symbol(&self) -> &'static str {
        PERIODIC_TABLE[(self.nuc - 1) as usize]
    }
}

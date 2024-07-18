use std::ops::Index;

use crate::{atom::Atom, num::Float};

use super::Molecule;

impl<T: Float> Index<usize> for Molecule<T> {
    type Output = Atom<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.atoms[index]
    }
}

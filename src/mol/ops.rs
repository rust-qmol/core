use std::ops::Index;


use crate::{atom::Atom, num::float::Float};

use super::Molecule;

impl<T: Float> Index<usize> for Molecule<T> {
    type Output = Atom<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.atoms[index]
    }
}

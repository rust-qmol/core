use crate::{atom::CoordinatesLike, num::float::Float};

use super::{prop::MoleculeBasicProp, Molecule};

pub trait MoleculeMove<T: Float>
where
    Self: MoleculeBasicProp<T>,
{
    fn move_to(&mut self, new_pos: CoordinatesLike<T>) -> &mut Self {
        *self.center_mut() = new_pos;
        self
    }
    fn move_to_x(&mut self, new_x: T) -> &mut Self {
        self.center_mut()[0] = new_x;
        self
    }
    fn move_to_y(&mut self, new_y: T) -> &mut Self {
        self.center_mut()[1] = new_y;
        self
    }
    fn move_to_z(&mut self, new_z: T) -> &mut Self {
        self.center_mut()[3] = new_z;
        self
    }

    fn move_position(&mut self, move_vec: CoordinatesLike<T>) -> &mut Self {
        self.center_mut()
            .iter_mut()
            .zip(move_vec.iter())
            .for_each(|(self_x, x)| *self_x = *x);
        self
    }
    fn move_position_x(&mut self, move_vec_x: T) -> &mut Self {
        self.center_mut()[0] += move_vec_x;
        self
    }
    fn move_position_y(&mut self, move_vec_y: T) -> &mut Self {
        self.center_mut()[1] += move_vec_y;
        self
    }
    fn move_position_z(&mut self, move_vec_z: T) -> &mut Self {
        self.center_mut()[1] += move_vec_z;
        self
    }
}

impl<T: Float> MoleculeMove<T> for Molecule<T> {}

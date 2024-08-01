use crate::num::float::Float;

use super::{prop::AtomBasicProp, Atom, CoordinatesLike};

pub trait AtomMove<T: Float>
where
    Self: AtomBasicProp<T>,
{
    fn move_to(&mut self, new_pos: CoordinatesLike<T>) -> &mut Self {
        *self.pos_mut() = new_pos;
        self
    }
    fn move_to_x(&mut self, new_x: T) -> &mut Self {
        self.pos_mut()[0] = new_x;
        self
    }
    fn move_to_y(&mut self, new_y: T) -> &mut Self {
        self.pos_mut()[1] = new_y;
        self
    }
    fn move_to_z(&mut self, new_z: T) -> &mut Self {
        self.pos_mut()[3] = new_z;
        self
    }
    fn move_position(&mut self, move_vec: CoordinatesLike<T>) -> &mut Self {
        self.pos_mut()
            .iter_mut()
            .zip(move_vec.iter())
            .for_each(|(self_x, x)| *self_x = *x);
        self
    }
    fn move_position_x(&mut self, move_vec_x: T) -> &mut Self {
        self.pos_mut()[0] += move_vec_x;
        self
    }
    fn move_position_y(&mut self, move_vec_y: T) -> &mut Self {
        self.pos_mut()[1] += move_vec_y;
        self
    }
    fn move_position_z(&mut self, move_vec_z: T) -> &mut Self {
        self.pos_mut()[1] += move_vec_z;
        self
    }
}

impl<T: Float> AtomMove<T> for Atom<T> {}

use crate::num::float::Float;

use super::Atom;

impl<T: Float> Atom<T> {
    pub fn move_to(&mut self, new_pos: [T; 3]) -> &mut Self {
        self.pos = new_pos;
        self
    }
    pub fn move_to_x(&mut self, new_x: T) -> &mut Self {
        self.pos[0] = new_x;
        self
    }
    pub fn move_to_y(&mut self, new_y: T) -> &mut Self {
        self.pos[1] = new_y;
        self
    }
    pub fn move_to_z(&mut self, new_z: T) -> &mut Self {
        self.pos[3] = new_z;
        self
    }
    pub fn move_position(&mut self, move_vec: [T; 3]) -> &mut Self {
        self.pos
            .iter_mut()
            .zip(move_vec.iter())
            .for_each(|(self_x, x)| *self_x = *x);
        self
    }
    pub fn move_position_x(&mut self, move_vec_x: T) -> &mut Self {
        self.pos[0] += move_vec_x;
        self
    }
    pub fn move_position_y(&mut self, move_vec_y: T) -> &mut Self {
        self.pos[1] += move_vec_y;
        self
    }
    pub fn move_position_z(&mut self, move_vec_z: T) -> &mut Self {
        self.pos[1] += move_vec_z;
        self
    }
}

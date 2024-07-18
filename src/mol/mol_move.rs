use crate::num::Float;

use super::Molecule;

impl<T: Float> Molecule<T> {
    pub fn move_to(&mut self, new_pos: [T; 3]) -> &mut Self {
        self.center = new_pos;
        self
    }
    pub fn move_to_x(&mut self, new_x: T) -> &mut Self {
        self.center[0] = new_x;
        self
    }
    pub fn move_to_y(&mut self, new_y: T) -> &mut Self {
        self.center[1] = new_y;
        self
    }
    pub fn move_to_z(&mut self, new_z: T) -> &mut Self {
        self.center[3] = new_z;
        self
    }

    pub fn move_position(&mut self, move_vec: [T; 3]) -> &mut Self {
        self.center
            .iter_mut()
            .zip(move_vec.iter())
            .for_each(|(self_x, x)| *self_x = *x);
        self
    }
    pub fn move_position_x(&mut self, move_vec_x: T) -> &mut Self {
        self.center[0] += move_vec_x;
        self
    }
    pub fn move_position_y(&mut self, move_vec_y: T) -> &mut Self {
        self.center[1] += move_vec_y;
        self
    }
    pub fn move_position_z(&mut self, move_vec_z: T) -> &mut Self {
        self.center[1] += move_vec_z;
        self
    }
}

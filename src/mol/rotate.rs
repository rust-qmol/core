use crate::{
    atom::{pos::AtomMove, prop::AtomBasicProp, CoordinatesLike},
    num::float::Float,
};

use super::Molecule;

pub(super) fn rotate_matrix<T: Float>(angle: CoordinatesLike<T>) -> [CoordinatesLike<T>; 3] {
    let xc = angle[0].cos();
    let xs = angle[0].sin();
    let yc = angle[1].cos();
    let ys = angle[1].sin();
    let zc = angle[2].cos();
    let zs = angle[2].sin();
    let xs_ys_zc = xs * ys * zc;
    let xc_zs = xc * zs;
    let xc_ys_zc = xc * ys * zc;
    let xs_zs = xs * zs;
    [
        [yc * zc, yc * zs, -ys],
        [xs_ys_zc - xc_zs, xs_ys_zc + xc_zs, xs * yc],
        [xc_ys_zc + xs_zs, xc_ys_zc - xs_zs, xc * yc],
    ]
}

pub(super) fn rotate<T: Float>(
    pos: Vec<CoordinatesLike<T>>,
    angle: CoordinatesLike<T>,
) -> Vec<CoordinatesLike<T>> {
    let rotate_matrix = rotate_matrix(angle);
    pos.iter()
        .map(|xyz| {
            [
                rotate_matrix[0]
                    .iter()
                    .zip(xyz)
                    .map(|(r, x)| (*r) * (*x))
                    .sum(),
                rotate_matrix[1]
                    .iter()
                    .zip(xyz)
                    .map(|(r, x)| (*r) * (*x))
                    .sum(),
                rotate_matrix[2]
                    .iter()
                    .zip(xyz)
                    .map(|(r, x)| (*r) * (*x))
                    .sum(),
            ]
        })
        .collect()
}

pub trait MoleculeRotate<T: Float> {
    fn rotate_to_global(&mut self, angle: CoordinatesLike<T>) -> &mut Self;
    fn rotate_to_local(&mut self, agnle: CoordinatesLike<T>) -> &mut Self;
}

impl<T: Float> MoleculeRotate<T> for Molecule<T> {
    fn rotate_to_global(&mut self, angle: CoordinatesLike<T>) -> &mut Self {
        let rotate_matrix = rotate_matrix(angle);
        self.atoms.iter_mut().for_each(|atm| {
            atm.move_to([
                rotate_matrix[0]
                    .iter()
                    .zip(atm.pos())
                    .map(|(r, x)| (*r) * x)
                    .sum(),
                rotate_matrix[1]
                    .iter()
                    .zip(atm.pos())
                    .map(|(r, x)| (*r) * x)
                    .sum(),
                rotate_matrix[2]
                    .iter()
                    .zip(atm.pos())
                    .map(|(r, x)| (*r) * x)
                    .sum(),
            ]);
        });
        self
    }
    fn rotate_to_local(&mut self, agnle: CoordinatesLike<T>) -> &mut Self {
        self.angle_local
            .iter_mut()
            .zip(agnle.iter())
            .for_each(|(self_angle, angle)| *self_angle = *angle);
        self
    }
}

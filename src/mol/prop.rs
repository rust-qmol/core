use crate::{
    atom::{prop::AtomBasicProp, Atom, CoordinatesLike},
    num::float::Float,
};

use super::{rotate::rotate, BigMolNuc, MolCharg, MolNuc, Molecule, Spin};

pub trait MoleculeBasicProp<T: Float> {
    fn nuc(&self) -> MolNuc;

    fn charg(&self) -> MolCharg;
    fn set_charg(&mut self, charg: MolCharg) -> &Self;

    fn spin(&self) -> Spin;
    fn set_spin(&mut self, spin: Spin) -> &Self;

    fn set_ele_prop(&mut self, charg: MolCharg, spin: Spin) -> &Self {
        assert_eq!(
            (self.nuc() as BigMolNuc - charg as BigMolNuc) % 2,
            spin as BigMolNuc
        );
        self.set_spin(spin);
        self.set_charg(charg);
        self
    }

    fn tot_ele(&self) -> MolNuc {
        (self.nuc() as BigMolNuc - self.charg() as BigMolNuc) as MolNuc
    }
    fn nele(&self) -> [MolNuc; 2] {
        let nuc = self.nuc();
        let charg = self.charg();
        let spin = self.spin();
        let tot_ele = nuc as BigMolNuc - charg as BigMolNuc;
        let nbeta = ((tot_ele - spin as BigMolNuc) / 2) as MolNuc;
        [nbeta + spin as MolNuc, nbeta]
    }

    fn center(&self) -> CoordinatesLike<T>;
    fn center_mut(&mut self) -> &mut CoordinatesLike<T>;

    fn angle_local(&self) -> CoordinatesLike<T>;
    fn angle_local_mut(&mut self) -> &mut CoordinatesLike<T>;

    fn atoms(&self) -> &Vec<impl AtomBasicProp<T>>;
    fn natm(&self) -> usize {
        self.atoms().len()
    }

    fn atoms_pos_local(&self) -> Vec<CoordinatesLike<T>> {
        self.atoms().iter().map(|atm| atm.pos()).collect()
    }
    fn atoms_pos_global(&self) -> Vec<CoordinatesLike<T>> {
        rotate(self.atoms_pos_local(), self.angle_local())
            .iter()
            .map(|xyz| {
                [
                    xyz[0] + self.center()[0],
                    xyz[1] + self.center()[1],
                    xyz[2] + self.center()[2],
                ]
            })
            .collect()
    }

    fn distance_matrix(&self) -> Vec<Vec<T>> {
        let natm = self.natm();
        let mut distance_matrix = vec![vec![T::zero(); natm]; natm];
        self.atoms_pos_local()
            .iter()
            .enumerate()
            .for_each(|(i, v1)| {
                self.atoms_pos_local()[0..i]
                    .iter()
                    .enumerate()
                    .for_each(|(j, v2)| {
                        let d = v1
                            .iter()
                            .zip(v2.iter())
                            .map(|(x, y)| (*x - *y).powi(2))
                            .sum::<T>()
                            .sqrt();
                        distance_matrix[i][j] = d;
                        distance_matrix[j][i] = d;
                    })
            });

        distance_matrix
    }
    fn distance_matrix_packed(&self, upper: bool) -> Vec<T> {
        let range = |i| {
            if upper {
                0..i
            } else {
                i..self.natm()
            }
        };

        self.atoms_pos_local()
            .iter()
            .enumerate()
            .flat_map(|(i, v1)| {
                self.atoms_pos_local()[range(i)]
                    .iter()
                    .map(|v2| {
                        v1.iter()
                            .zip(v2.iter())
                            .map(|(x, y)| (*x - *y).powi(2))
                            .sum::<T>()
                            .sqrt()
                    })
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}

impl<T: Float> MoleculeBasicProp<T> for Molecule<T> {
    fn nuc(&self) -> MolNuc {
        self.nuc
    }

    fn charg(&self) -> MolCharg {
        self.charg
    }

    fn set_charg(&mut self, charg: MolCharg) -> &Self {
        self.charg = charg;
        self
    }

    fn spin(&self) -> Spin {
        self.spin
    }

    fn set_spin(&mut self, spin: Spin) -> &Self {
        self.spin = spin;
        self
    }

    fn center(&self) -> CoordinatesLike<T> {
        self.center
    }

    fn center_mut(&mut self) -> &mut CoordinatesLike<T> {
        &mut self.center
    }

    fn angle_local(&self) -> CoordinatesLike<T> {
        self.angle_local
    }

    fn angle_local_mut(&mut self) -> &mut CoordinatesLike<T> {
        &mut self.angle_local
    }

    #[allow(refining_impl_trait)]
    fn atoms(&self) -> &Vec<Atom<T>> {
        &self.atoms
    }
}

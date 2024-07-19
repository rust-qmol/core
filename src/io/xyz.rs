use std::str::FromStr;

use crate::{
    atom::Atom,
    io::PERIODIC_TABLE,
    mol::Molecule, num::{convert::aatoau, float::Float},
};

impl<T: Float> Atom<T> {
    pub fn from_xyz(xyz_str: &str) -> Result<Self, <T as FromStr>::Err> {
        let mut split_s = xyz_str.trim().split_whitespace();
        assert_eq!(split_s.clone().count(), 4);
        let nuc = match split_s.next() {
            Some(symbol) => match PERIODIC_TABLE.iter().position(|ele| *ele == symbol) {
                None => panic!(""),
                Some(nuc) => (nuc + 1) as u8,
            },
            None => panic!(""),
        };

        let pos: Vec<T> = split_s
            .map(|x_str| match x_str.parse() {
                Ok(x) => aatoau(x),
                Err(_) => panic!(""),
            })
            .collect();
        Ok(Atom::new(nuc, 0, [pos[0], pos[1], pos[2]]))
    }

    pub fn from_mul_xyz(xyz_str: Vec<&str>) -> Result<Vec<Self>, <T as FromStr>::Err> {
        Ok(xyz_str
            .iter()
            .map(|xyz_line| match Atom::<T>::from_xyz(xyz_line) {
                Ok(atm) => atm,
                Err(_) => panic!(""),
            })
            .collect())
    }
}

impl<T: Float> Molecule<T> {
    pub fn from_xyz(xyz_str: &str) -> Self {
        #[allow(unused_assignments)]
        let mut natm: usize = 0;
        let xyz_list: Vec<&str> = xyz_str.trim_end().lines().collect();
        match xyz_list[0].parse() {
            Err(why) => panic!("{:?}", why),
            Ok(v) => natm = v,
        }

        let atoms = match Atom::<T>::from_mul_xyz(xyz_list[2..xyz_list.len()].to_vec()) {
            Ok(atms) => atms,
            Err(_) => panic!(""),
        };
        assert_eq!(natm, atoms.len());

        Self::new(atoms)
    }

    pub fn from_mul_xyz(xyz_str: &str) -> Vec<Self> {
        let xyz_list: Vec<&str> = xyz_str.trim_end().lines().collect();

        let mut nline: usize = 0;
        let mol_list = vec![];
        while nline < xyz_list.len() {
            match xyz_list[nline].parse::<usize>() {
                Err(why) => panic!("{:?}", why),
                Ok(v) => nline += v + 2,
            }
        }
        mol_list
    }

    pub fn to_xyz(&self) -> String {
        let mut xyz_str = String::from(format!(
            "{}\ncharg:{},spin:{}\n",
            self.atoms().len(),
            self.charg(),
            self.spin()
        ));
        self.atoms()
            .iter()
            .map(|atom| atom.symbol())
            .zip(self.atoms_pos_globe().iter())
            .map(|(sym, xyz)| {
                String::from(format!(
                    "{}  {:14.7}{:14.7}{:14.7}\n",
                    sym, xyz[0], xyz[1], xyz[2]
                ))
            })
            .for_each(|xyz_line| xyz_str += &xyz_line);
        xyz_str
    }
}

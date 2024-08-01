use crate::{
    atom::{prop::AtomBasicProp, Atom},
    io::PERIODIC_TABLE,
    mol::{prop::MoleculeBasicProp, Molecule},
    num::{convert::aa2au, float::Float},
};

pub trait IoXYZ<T: Float>
where
    Self: Sized,
{
    fn from_xyz(xyz_str: &str) -> Result<Self, &str>;
    fn from_mul_xyz(xyz_str_list: Vec<&str>) -> Result<Vec<Self>, &str> {
        let mut out_list = vec![];
        for xyz_str in xyz_str_list.iter() {
            out_list.push(match Self::from_xyz(xyz_str) {
                Ok(object) => object,
                Err(_) => return Err(""),
            });
        }
        Ok(out_list)
    }

    fn to_xyz(&self) -> String;
}

impl<T: Float> IoXYZ<T> for Atom<T> {
    fn from_xyz(xyz_str: &str) -> Result<Self, &str> {
        let mut split_s = xyz_str.trim().split_whitespace();
        assert_eq!(split_s.clone().count(), 4);
        let nuc = match split_s.next() {
            Some(symbol) => match PERIODIC_TABLE.iter().position(|ele| *ele == symbol) {
                Some(nuc) => (nuc + 1) as u8,
                None => return Err(""),
            },
            None => return Err(""),
        };

        let mut pos = [T::one(); 3];
        for (i, x_str) in split_s.enumerate() {
            match x_str.parse::<T>() {
                Ok(x) => pos[i] = aa2au(x),
                Err(_) => return Err(""),
            }
        }

        Ok(Atom::new(nuc, 0, pos))
    }

    fn to_xyz(&self) -> String {
        String::from(format!(
            "{}  {:14.7}{:14.7}{:14.7}\n",
            self.symbol(),
            self.pos[0],
            self.pos[1],
            self.pos[2]
        ))
    }
}

impl<T: Float> IoXYZ<T> for Molecule<T> {
    fn from_xyz(xyz_str: &str) -> Result<Self, &str> {
        let natm: usize;
        let xyz_list: Vec<&str> = xyz_str.trim_end().lines().collect();
        match xyz_list[0].parse() {
            Ok(v) => natm = v,
            Err(_) => return Err(""),
        }

        let atoms = match Atom::<T>::from_mul_xyz(xyz_list[2..xyz_list.len()].to_vec()) {
            Ok(atms) => atms,
            Err(_) => return Err(""),
        };
        if natm != atoms.len() {
            return Err("");
        }

        Ok(Self::new(atoms))
    }

    fn to_xyz(&self) -> String {
        String::from(format!(
            "{}\ncharg:{},spin:{}\n",
            self.atoms().len(),
            self.charg(),
            self.spin()
        )) + &self
            .atoms()
            .iter()
            .map(|atom| atom.to_xyz())
            .collect::<Vec<String>>()
            .concat()
    }
}

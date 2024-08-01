use super::float::Float;

//  convert bohr (a.u.) to Ångström and back
const AUTOAA: f32 = 0.52917726;
const AATOAU: f32 = 1.0 / AUTOAA;
//  convert Hartree to eV and back
const AUTOEV: f32 = 27.21138505;
const EVTOAU: f32 = 1.0 / AUTOEV;
//  convert Hartree to kcal/mol and back
const AUTOKCAL: f32 = 627.50947428;
const KCALTOAU: f32 = 1.0 / AUTOKCAL;
//  convert Hartree to kJ/mol and back
const AUTOKJ: f32 = 2625.49964038;
const KJTOAU: f32 = 1.0 / AUTOKJ;
//  convert Hartree (a.u.) to Joule and back
const AUTOJ: f32 = 4.3597447222071e-18;
const JTOAU: f32 = 1.0 / AUTOJ;
//  convert Hartree to reciproce centimeters/wavenumbers and back
const AUTORCM: f32 = 219474.63067;
const AUTOWAV: f32 = AUTORCM;
const RCMTOAU: f32 = 1.0 / AUTORCM;
const WAVTOAU: f32 = 1.0 / AUTOWAV;
//  convert Hartree to nanometers and back
const AUTONM: f32 = 45.56335266;
const NMTOAU: f32 = 1.0 / AUTONM;
//  masses
//  amu -> kg :: conversion from atomic mass units to kg
//  me  -> kg :: electron mass (a.u.) in kg
//  amu -> au :: conversion from a.u. to amu
const AMUTOKG: f32 = 1.660539040e-27;
const KGTOAMU: f32 = 1.0 / AMUTOKG;
const METOKG: f32 = 9.10938356e-31;
const KGTOME: f32 = 1.0 / METOKG;
const AMUTOAU: f32 = AMUTOKG * KGTOME;
const AUTOAMU: f32 = KGTOAMU * METOKG;
//  femtosectons to atomic time units
const FSTOAU: f32 = 41.3413733365614;
const AUTOFS: f32 = 1.0 / FSTOAU;
//  Coulomb to atomic charge units (electrons)
const AUTOC: f32 = 1.6021766208e-19;
const CTOAU: f32 = 1.0 / AUTOC;

macro_rules! convert_impl {
    ($(($from_fn: ident, $to_fn: ident, $convert_const: expr)),*) => {
        $(
            pub fn $from_fn<T: Float>(x: T) -> T {
                x * ($convert_const).into()
            }

            pub fn $to_fn<T: Float>(x: T) -> T {
                x * (1.0 / $convert_const).into()
            }
        )*
    };
}

convert_impl!(
    (au2aa, aa2au, AUTOAA),
    (au2ev, ev2au, AUTOEV),
    (au2kcal, kcal2au, AUTOKCAL),
    (au2kj, kj2au, AUTOKJ),
    (au2j, j2au, AUTOJ),
    (au2rcm, rcm2au, AUTORCM),
    (au2wav, wav2au, AUTOWAV),
    (au2nm, nm2au, AUTONM),
    (amu2kg, kg2amu, AMUTOKG),
    (me2kg, kg2me, METOKG),
    (au2amu, amu2au, AUTOAMU),
    (au2fs, fs2au, AUTOFS),
    (au2c, c2au, AUTOC)
);

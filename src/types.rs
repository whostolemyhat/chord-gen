use std::hash::Hash;
use std::str::FromStr;

#[derive(Hash, Default)]
pub struct Chord<'a> {
    pub frets: Vec<i32>,       // -1 = skip
    pub fingers: Vec<&'a str>, // 'x' = skip
    pub title: Option<&'a String>,
    pub hand: Hand,
    pub suffix: Option<&'a String>,
    pub mode: Mode,
    pub use_background: bool,
    pub barres: Option<Vec<i32>>,
}

#[derive(Debug)]
pub enum GuitarString {
    E = 0,
    A = 1,
    D = 2,
    G = 3,
    B = 4,
    HighE = 5,
}

impl From<usize> for GuitarString {
    fn from(value: usize) -> Self {
        match value {
            1 => GuitarString::A,
            2 => GuitarString::D,
            3 => GuitarString::G,
            4 => GuitarString::B,
            5 => GuitarString::HighE,
            _ => GuitarString::E,
        }
    }
}

pub const LIGHT_COLOUR: &str = "#FBF6E2";
pub const DARK_COLOUR: &str = "#160c1c";

#[derive(Hash, Default, Copy, Clone)]
pub enum Mode {
    #[default]
    Light,
    Dark,
}

#[derive(PartialEq, Hash, Default)]
pub enum Hand {
    #[default]
    Right,
    Left,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Hand::Left),
            _ => Ok(Hand::Right),
        }
    }
}

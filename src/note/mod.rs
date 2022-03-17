pub mod pitch_note;

use crate::pitch::Pitch;
use core::fmt::{self, Debug};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Letter(u8);

impl Letter {
    pub const A: Self = Self(0);
    pub const B: Self = Self(1);
    pub const C: Self = Self(2);
    pub const D: Self = Self(3);
    pub const E: Self = Self(4);
    pub const F: Self = Self(5);
    pub const G: Self = Self(6);

    pub const fn next(self) -> Self {
        Self((self.0 + 1) % (Self::G.0 + 1))
    }

    pub const fn to_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
            _ => todo!(),
        }
    }
}

impl fmt::Debug for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accidental {
    Natrual,
    Flat,
    DoubleFlat,
    Sharp,
    DoubleSharp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    pub letter: Letter,
    pub accidental: Accidental,
}

impl Note {
    pub const fn new(letter: Letter, accidental: Accidental) -> Self {
        Self { letter, accidental }
    }

    pub const fn natural(letter: Letter) -> Self {
        Self::new(letter, Accidental::Natrual)
    }

    pub const fn flat(letter: Letter) -> Self {
        Self::new(letter, Accidental::Flat)
    }

    pub const fn sharp(letter: Letter) -> Self {
        Self::new(letter, Accidental::Sharp)
    }

    pub const fn from_sharp(pitch: Pitch) -> Self {
        match pitch {
            Pitch::C => Self::natural(Letter::C),
            Pitch::D => Self::natural(Letter::D),
            Pitch::G_SHARP => Self::sharp(Letter::G),
            _ => todo!(),
        }
    }

    pub const fn from_flat(pitch: Pitch) -> Self {
        match pitch {
            Pitch::C => Self::natural(Letter::C),
            Pitch::D => Self::natural(Letter::D),
            Pitch::G_SHARP => Self::flat(Letter::A),
            _ => todo!(),
        }
    }
}

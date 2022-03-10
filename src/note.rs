use core::fmt::{self, Debug};
use core::ops::Add;
use std::ops::Sub;

use crate::Interval;

// TODO custom debug
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Pitch(u8);

impl Pitch {
    pub const C: Self = Self(0);
    pub const C_SHARP: Self = Self(1);

    pub const D: Self = Self(2);
    pub const D_SHARP: Self = Self(3);

    pub const E: Self = Self(4);

    pub const F: Self = Self(5);
    pub const F_SHARP: Self = Self(6);

    pub const G: Self = Self(7);
    pub const G_SHARP: Self = Self(8);

    pub const A: Self = Self(9);
    pub const A_SHARP: Self = Self(10);

    pub const B: Self = Self(11);
}

impl From<Letter> for Pitch {
    fn from(letter: Letter) -> Self {
        match letter {
            Letter::C => Self::C,
            Letter::D => Self::D,
            Letter::E => Self::E,
            Letter::F => Self::F,
            Letter::G => Self::G,
            Letter::A => Self::A,
            Letter::B => Self::B,
            _ => todo!(),
        }
    }
}

impl Add<Interval> for Pitch {
    type Output = Self;

    fn add(self, interval: Interval) -> Self {
        Self((self.0 + interval.semitones()) % (Self::B.0 + 1))
    }
}

impl Sub for Pitch {
    type Output = Interval;

    fn sub(self, rhs: Self) -> Interval {
        Interval::new(self.0 - rhs.0)
    }
}

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
}

impl From<Pitch> for Note {
    fn from(note: Pitch) -> Self {
        match note {
            Pitch::C => Self::natural(Letter::C),
            Pitch::D => Self::natural(Letter::D),
            _ => todo!(),
        }
    }
}

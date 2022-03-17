use std::ops::{Add, Sub};

use crate::{
    note::{Letter, Note},
    Interval,
};

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

    pub const fn natural(letter: Letter) -> Self {
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

    pub const fn add_interval(self, interval: Interval) -> Self {
        Self((self.0 + interval.semitones()) % (Self::B.0 + 1))
    }

    pub const fn into_byte(self) -> u8 {
        self.0
    }

    pub const fn sub(self, rhs: Self) -> Interval {
        Interval::new(self.0 - rhs.0)
    }
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

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        todo!()
    }
}

impl Add<Interval> for Pitch {
    type Output = Self;

    fn add(self, interval: Interval) -> Self {
       self.add_interval(interval)
    }
}

impl Sub for Pitch {
    type Output = Interval;

    fn sub(self, rhs: Self) -> Interval {
        self.sub(rhs)
    }
}

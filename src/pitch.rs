use std::ops::{Add, Sub};

use crate::{
    note::{Accidental, Letter, Note},
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

    pub const fn from_byte(byte: u8) -> Self {
        Self(byte % (Self::B.into_byte() + 1))
    }

    pub const fn from_byte_unchecked(byte: u8) -> Self {
        Self(byte)
    }

    pub const fn from_note(note: Note) -> Self {
        let natural = Self::natural(note.letter);
        match note.accidental {
            Accidental::Natrual => natural,
            Accidental::Flat => natural.sub_interval(Interval::MINOR_SECOND),
            Accidental::DoubleFlat => natural.sub_interval(Interval::MAJOR_SECOND),
            Accidental::Sharp => natural.add_interval(Interval::MINOR_SECOND),
            Accidental::DoubleSharp => natural.add_interval(Interval::MAJOR_SECOND),
        }
    }

    pub const fn add_interval(self, interval: Interval) -> Self {
        Self((self.0 + interval.semitones()) % (Self::B.0 + 1))
    }

    pub const fn sub_interval(self, interval: Interval) -> Self {
        Self::from_byte((self.0 as i8 - interval.semitones() as i8).abs() as u8)
    }

    pub const fn into_byte(self) -> u8 {
        self.0
    }

    pub const fn sub(self, rhs: Self) -> Interval {
        Interval::new(self.0 - rhs.0)
    }

    pub fn transpose(self, key: Pitch, to: Pitch) -> Pitch {
        let f = self - key;
        to + f
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
        Self::from_note(note)
    }
}

impl From<Pitch> for u8 {
    fn from(pitch: Pitch) -> Self {
        pitch.into_byte()
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

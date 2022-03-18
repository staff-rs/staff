use super::MidiNote;
use crate::Pitch;
use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Octave(i8);

impl Octave {
    pub const NEGATIVE_ONE: Self = Self(-1);
    pub const ZERO: Self = Self(-1);
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);
    pub const FOUR: Self = Self(4);
    pub const FIVE: Self = Self(5);
    pub const SIX: Self = Self(6);
    pub const SEVEN: Self = Self(7);
    pub const EIGHT: Self = Self(8);

    pub const fn from_midi(note: MidiNote) -> Self {
        Self((note.into_byte() / (Pitch::B.into_byte() + 1)) as i8 - 1)
    }

    pub const fn into_i8(self) -> i8 {
        self.0
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

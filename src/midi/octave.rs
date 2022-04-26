use super::MidiNote;
use crate::Pitch;
use core::fmt;

/// A note's octave in MIDI.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Octave(i8);

impl Octave {
    /// Octave -1
    pub const NEGATIVE_ONE: Self = Self(-1);
    /// Octave 0
    pub const ZERO: Self = Self(0);
    /// Octave 1
    pub const ONE: Self = Self(1);
    /// Octave 2
    pub const TWO: Self = Self(2);
    /// Octave 3
    pub const THREE: Self = Self(3);
    /// Octave 4
    pub const FOUR: Self = Self(4);
    /// Octave 5
    pub const FIVE: Self = Self(5);
    /// Octave 6
    pub const SIX: Self = Self(6);
    /// Octave 7
    pub const SEVEN: Self = Self(7);
    /// Octave 8
    pub const EIGHT: Self = Self(8);

    /// Return the `Octave` of the given midinote.
    pub const fn from_midi(note: MidiNote) -> Self {
        Self((note.into_byte() / (Pitch::B.into_byte() + 1)) as i8 - 1)
    }

    /// Return the i8 representation of `self`.
    pub const fn into_i8(self) -> i8 {
        self.0
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

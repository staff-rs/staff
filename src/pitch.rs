use crate::{midi::MidiNote, note::Note, Interval, Natural};
use core::ops::{Add, Sub};

/// Pitch class that can be found on the chromatic scale.
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

    /// Returns the natural pitch for the given `Natural`.
    /// ```
    /// use music_theory::{Pitch, Natural};
    ///
    /// let pitch = Pitch::natural(Natural::F);
    /// assert_eq!(pitch, Pitch::F);
    /// ```
    pub const fn natural(letter: Natural) -> Self {
        match letter {
            Natural::C => Self::C,
            Natural::D => Self::D,
            Natural::E => Self::E,
            Natural::F => Self::F,
            Natural::G => Self::G,
            Natural::A => Self::A,
            Natural::B => Self::B,
        }
    }

    pub const fn from_byte(byte: u8) -> Self {
        Self(byte % (Self::B.into_byte() + 1))
    }

    pub const fn from_byte_unchecked(byte: u8) -> Self {
        Self(byte)
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

impl From<Natural> for Pitch {
    fn from(letter: Natural) -> Self {
        match letter {
            Natural::C => Self::C,
            Natural::D => Self::D,
            Natural::E => Self::E,
            Natural::F => Self::F,
            Natural::G => Self::G,
            Natural::A => Self::A,
            Natural::B => Self::B,
        }
    }
}

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        note.pitch()
    }
}

impl From<MidiNote> for Pitch {
    fn from(midi: MidiNote) -> Self {
        midi.pitch()
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

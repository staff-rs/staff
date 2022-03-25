use crate::{midi::MidiNote, note::Note, Interval, Natural};
use core::mem;
use core::ops::{Add, Sub};

/// Pitch class that can be found on the chromatic scale.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Pitch {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Pitch {
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
        unsafe { mem::transmute(byte % (Self::B.into_byte() + 1)) }
    }

    pub const fn add_interval(self, interval: Interval) -> Self {
        unsafe { mem::transmute((self as u8 + interval.semitones()) % (Self::B as u8 + 1)) }
    }

    pub const fn sub_interval(self, interval: Interval) -> Self {
        Self::from_byte((self as u8 as i8 - interval.semitones() as i8).abs() as u8)
    }

    pub const fn into_byte(self) -> u8 {
        self as _
    }

    pub const fn sub(self, rhs: Self) -> Interval {
        Interval::new(self as u8 - rhs as u8)
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

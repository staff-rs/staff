use crate::note::{Accidental, Note};
use crate::{midi::MidiNote, Interval, Natural};
use core::ops::{Add, Sub};
use core::{fmt, mem};

/// Pitch class that can be found on the chromatic scale.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// use staff::{Pitch, Natural};
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
        let byte = if self as u8 > interval.semitones() {
            self as u8 - interval.semitones()
        } else {
            self as u8 + Self::B as u8 + 1 - interval.semitones()
        };
        Self::from_byte(byte)
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

    pub fn abs_diff(self, rhs: Self) -> Interval {
        Interval::new((self as u8).abs_diff(rhs as u8))
    }

    pub fn is_natural(self) -> bool {
        match self {
            Self::C | Self::D | Self::E | Self::F | Self::G | Self::A | Self::B => true,
            _ => false,
        }
    }
}

impl From<u8> for Pitch {
    fn from(byte: u8) -> Self {
        Self::from_byte(byte)
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

impl From<MidiNote> for Pitch {
    fn from(midi: MidiNote) -> Self {
        midi.pitch()
    }
}

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        let pitch: Pitch = note.natural.into();
        match note.accidental {
            Accidental::Natural => pitch,
            Accidental::Flat => pitch - Interval::MINOR_SECOND,
            Accidental::DoubleFlat => pitch - Interval::MAJOR_SECOND,
            Accidental::Sharp => pitch + Interval::MINOR_SECOND,
            Accidental::DoubleSharp => pitch + Interval::MAJOR_SECOND,
        }
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

impl Sub<Interval> for Pitch {
    type Output = Self;

    fn sub(self, interval: Interval) -> Self {
        self.sub_interval(interval)
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Pitch::C => "C",
            Pitch::CSharp => "C#",
            Pitch::D => "D",
            Pitch::DSharp => "D#",
            Pitch::E => "E",
            Pitch::F => "F",
            Pitch::FSharp => "F#",
            Pitch::G => "G",
            Pitch::GSharp => "G#",
            Pitch::A => "A",
            Pitch::ASharp => "A#",
            Pitch::B => "B",
        };
        f.write_str(s)
    }
}

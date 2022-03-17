pub mod pitch_note;

use crate::{pitch::Pitch, Interval};
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

impl Accidental {
    pub fn interval(self) -> Interval {
        match self {
            Self::Natrual => Interval::UNISON,
            Self::Flat | Self::Sharp => Interval::MINOR_SECOND,
            Self::DoubleFlat | Self::DoubleSharp => Interval::MAJOR_SECOND,
        }
    }
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
            Pitch::C_SHARP => Self::sharp(Letter::C),
            Pitch::D => Self::natural(Letter::D),
            Pitch::D_SHARP => Self::sharp(Letter::D),
            Pitch::E => Self::natural(Letter::E),
            Pitch::F => Self::natural(Letter::F),
            Pitch::F_SHARP => Self::sharp(Letter::F),
            Pitch::G => Self::natural(Letter::G),
            Pitch::G_SHARP => Self::sharp(Letter::G),
            Pitch::A => Self::natural(Letter::A),
            Pitch::A_SHARP => Self::sharp(Letter::A),
            Pitch::B => Self::natural(Letter::B),
            _ => todo!(),
        }
    }

    pub const fn from_flat(pitch: Pitch) -> Self {
        match pitch {
            Pitch::C => Self::natural(Letter::C),
            Pitch::C_SHARP => Self::flat(Letter::D),
            Pitch::D => Self::natural(Letter::D),
            Pitch::D_SHARP => Self::flat(Letter::E),
            Pitch::E => Self::natural(Letter::E),
            Pitch::F => Self::natural(Letter::F),
            Pitch::F_SHARP => Self::flat(Letter::G),
            Pitch::G => Self::natural(Letter::G),
            Pitch::G_SHARP => Self::flat(Letter::A),
            Pitch::A => Self::natural(Letter::A),
            Pitch::A_SHARP => Self::flat(Letter::B),
            Pitch::B => Self::natural(Letter::B),
            _ => todo!(),
        }
    }

    /// Returns the enharmonic note for `self` in flat notation.
    ///
    /// # Examples
    ///
    /// Convert a `Note` in sharp notation to flats
    /// ```
    /// use music::note::{Letter, Note};
    ///
    /// let note = Note::sharp(Letter::G);
    /// assert_eq!(note.into_flat(), Note::flat(Letter::A))
    /// ```
    ///
    /// Find a natural enharmonic note
    /// ```
    /// use music::note::{Letter, Note};
    ///
    /// let note = Note::flat(Letter::F);
    /// assert_eq!(note.into_flat(), Note::natural(Letter::E))
    /// ```
    pub const fn into_flat(self) -> Self {
        Self::from_flat(Pitch::from_note(self))
    }

    /// Returns the enharmonic note for `self` in sharp notation.
    ///
    /// # Examples
    ///
    /// Convert a `Note` in flat notation to sharps
    /// ```
    /// use music::note::{Letter, Note};
    ///
    /// let note = Note::flat(Letter::D);
    /// assert_eq!(note.into_sharp(), Note::sharp(Letter::C))
    /// ```
    ///
    /// Find a natural enharmonic note
    /// ```
    /// use music::note::{Letter, Note};
    ///
    /// let note = Note::sharp(Letter::B);
    /// assert_eq!(note.into_sharp(), Note::natural(Letter::C))
    /// ```
    pub const fn into_sharp(self) -> Self {
        Self::from_sharp(Pitch::from_note(self))
    }
}

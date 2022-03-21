use crate::{pitch::Pitch, Interval, Letter};
use core::fmt::{self, Debug};

mod accidental;
pub use accidental::Accidental;

mod pitch_note;
pub use pitch_note::PitchNote;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    pub letter: Letter,
    pub accidental: Accidental,
}

impl Note {
    /// Create a new `Note` from a `Letter` and `Accidental`.
    /// ```
    /// use music_theory::{Accidental, Letter, Note};
    ///
    /// let note = Note::new(Letter::D, Accidental::Sharp);
    /// assert_eq!(note.to_string(), "D#");
    /// ```
    pub const fn new(letter: Letter, accidental: Accidental) -> Self {
        Self { letter, accidental }
    }

    /// Create a new natural `Note` from a `Letter` with [`Accidental::Natural`].
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::natural(Letter::E);
    /// assert_eq!(note.to_string(), "E");
    /// ```
    pub const fn natural(letter: Letter) -> Self {
        Self::new(letter, Accidental::Natural)
    }

    /// Create a new `Note` from a `Letter` with [`Accidental::Flat`].
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::flat(Letter::F);
    /// assert_eq!(note.to_string(), "Fb");
    /// ```
    pub const fn flat(letter: Letter) -> Self {
        Self::new(letter, Accidental::Flat)
    }

    /// Create a new `Note` from a `Letter` with [`Accidental::DoubleFlat`].
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::double_flat(Letter::G);
    /// assert_eq!(note.to_string(), "Gbb");
    /// ```
    pub const fn double_flat(letter: Letter) -> Self {
        Self::new(letter, Accidental::DoubleFlat)
    }

    /// Create a new `Note` from a `Letter` with [`Accidental::Sharp`].
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::sharp(Letter::E);
    /// assert_eq!(note.to_string(), "E#");
    /// ```
    pub const fn sharp(letter: Letter) -> Self {
        Self::new(letter, Accidental::Sharp)
    }

    /// Create a new `Note` from a `Letter` with [`Accidental::DoubleSharp`].
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::double_sharp(Letter::D);
    /// assert_eq!(note.to_string(), "D##");
    /// ```
    pub const fn double_sharp(letter: Letter) -> Self {
        Self::new(letter, Accidental::DoubleSharp)
    }

    /// Return the `Note` for the given `Pitch`.
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
            _ => unreachable!(),
        }
    }

    /// Return the `Note` for the given `Pitch`.
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
            _ => unreachable!(),
        }
    }

    /// Returns the enharmonic note for `self` in flat notation.
    ///
    /// # Examples
    ///
    /// Convert a `Note` in sharp notation to flats
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::sharp(Letter::G);
    /// assert_eq!(note.into_flat(), Note::flat(Letter::A))
    /// ```
    ///
    /// Find a natural enharmonic note
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::flat(Letter::F);
    /// assert_eq!(note.into_flat(), Note::natural(Letter::E))
    /// ```
    pub const fn into_flat(self) -> Self {
        Self::from_flat(self.pitch())
    }

    /// Returns the enharmonic note for `self` in sharp notation.
    ///
    /// # Examples
    ///
    /// Convert a `Note` in flat notation to sharps
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::flat(Letter::D);
    /// assert_eq!(note.into_sharp(), Note::sharp(Letter::C))
    /// ```
    ///
    /// Find a natural enharmonic note
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::sharp(Letter::B);
    /// assert_eq!(note.into_sharp(), Note::natural(Letter::C))
    /// ```
    pub const fn into_sharp(self) -> Self {
        Self::from_sharp(self.pitch())
    }

    /// Returns the pitch of the given `Note`.
    /// ```
    /// use music_theory::{Pitch, Letter};
    ///
    /// let pitch = Pitch::natural(Letter::F);
    /// assert_eq!(pitch, Pitch::F);
    /// ```
    pub const fn pitch(self) -> Pitch {
        let natural = Pitch::natural(self.letter);
        match self.accidental {
            Accidental::Natural => natural,
            Accidental::Flat => natural.sub_interval(Interval::MINOR_SECOND),
            Accidental::DoubleFlat => natural.sub_interval(Interval::MAJOR_SECOND),
            Accidental::Sharp => natural.add_interval(Interval::MINOR_SECOND),
            Accidental::DoubleSharp => natural.add_interval(Interval::MAJOR_SECOND),
        }
    }

    /// Returns `true` if the `self` is enharmonically equivalent to `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::flat(Letter::D);
    /// assert!(note.is_enharmonic(Note::sharp(Letter::C)))
    /// ```
    ///
    /// This function will also return true if the notes are the same.
    /// ```
    /// use music_theory::{Letter, Note};
    ///
    /// let note = Note::natural(Letter::C);
    /// assert!(note.is_enharmonic(note))
    /// ```
    pub const fn is_enharmonic(self, other: Self) -> bool {
        self.pitch().into_byte() == other.pitch().into_byte()
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let accidental = match self.accidental {
            Accidental::Natural => "",
            Accidental::Sharp => "#",
            Accidental::DoubleSharp => "##",
            Accidental::Flat => "b",
            Accidental::DoubleFlat => "bb",
        };
        write!(f, "{}{}", self.letter, accidental)
    }
}

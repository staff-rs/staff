use crate::{pitch::Pitch, Interval, Natural};
use core::fmt::{self, Debug};

mod accidental;
pub use accidental::Accidental;

mod pitch_note;
pub use pitch_note::PitchNote;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    pub letter: Natural,
    pub accidental: Accidental,
}

impl Note {
    /// Create a new `Note` from a `Natural` and `Accidental`.
    /// ```
    /// use music_theory::{Accidental, Natural, Note};
    ///
    /// let note = Note::new(Natural::D, Accidental::Sharp);
    /// assert_eq!(note.to_string(), "D#");
    /// ```
    pub const fn new(letter: Natural, accidental: Accidental) -> Self {
        Self { letter, accidental }
    }

    /// Create a new natural `Note` from a `Natural` with [`Accidental::Natural`].
    /// ```
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::natural(Natural::E);
    /// assert_eq!(note.to_string(), "E");
    /// ```
    pub const fn natural(letter: Natural) -> Self {
        Self::new(letter, Accidental::Natural)
    }

    /// Create a new `Note` from a `Natural` with [`Accidental::Flat`].
    /// ```
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::flat(Natural::F);
    /// assert_eq!(note.to_string(), "Fb");
    /// ```
    pub const fn flat(letter: Natural) -> Self {
        Self::new(letter, Accidental::Flat)
    }

    /// Create a new `Note` from a `Natural` with [`Accidental::DoubleFlat`].
    /// ```
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::double_flat(Natural::G);
    /// assert_eq!(note.to_string(), "Gbb");
    /// ```
    pub const fn double_flat(letter: Natural) -> Self {
        Self::new(letter, Accidental::DoubleFlat)
    }

    /// Create a new `Note` from a `Natural` with [`Accidental::Sharp`].
    /// ```
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::sharp(Natural::E);
    /// assert_eq!(note.to_string(), "E#");
    /// ```
    pub const fn sharp(letter: Natural) -> Self {
        Self::new(letter, Accidental::Sharp)
    }

    /// Create a new `Note` from a `Natural` with [`Accidental::DoubleSharp`].
    /// ```
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::double_sharp(Natural::D);
    /// assert_eq!(note.to_string(), "D##");
    /// ```
    pub const fn double_sharp(letter: Natural) -> Self {
        Self::new(letter, Accidental::DoubleSharp)
    }

    /// Return the `Note` for the given `Pitch`.
    pub const fn from_sharp(pitch: Pitch) -> Self {
        match pitch {
            Pitch::C => Self::natural(Natural::C),
            Pitch::CSharp => Self::sharp(Natural::C),
            Pitch::D => Self::natural(Natural::D),
            Pitch::DSharp => Self::sharp(Natural::D),
            Pitch::E => Self::natural(Natural::E),
            Pitch::F => Self::natural(Natural::F),
            Pitch::FSharp => Self::sharp(Natural::F),
            Pitch::G => Self::natural(Natural::G),
            Pitch::GSharp => Self::sharp(Natural::G),
            Pitch::A => Self::natural(Natural::A),
            Pitch::ASharp => Self::sharp(Natural::A),
            Pitch::B => Self::natural(Natural::B),
        }
    }

    /// Return the `Note` for the given `Pitch`.
    pub const fn from_flat(pitch: Pitch) -> Self {
        match pitch {
            Pitch::C => Self::natural(Natural::C),
            Pitch::CSharp => Self::flat(Natural::D),
            Pitch::D => Self::natural(Natural::D),
            Pitch::DSharp => Self::flat(Natural::E),
            Pitch::E => Self::natural(Natural::E),
            Pitch::F => Self::natural(Natural::F),
            Pitch::FSharp => Self::flat(Natural::G),
            Pitch::G => Self::natural(Natural::G),
            Pitch::GSharp => Self::flat(Natural::A),
            Pitch::A => Self::natural(Natural::A),
            Pitch::ASharp => Self::flat(Natural::B),
            Pitch::B => Self::natural(Natural::B),
        }
    }

    /// Returns the enharmonic note for `self` in flat notation.
    ///
    /// # Examples
    ///
    /// Convert a `Note` in sharp notation to flats
    /// ```
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::sharp(Natural::G);
    /// assert_eq!(note.into_flat(), Note::flat(Natural::A))
    /// ```
    ///
    /// Find a natural enharmonic note
    /// ```
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::flat(Natural::F);
    /// assert_eq!(note.into_flat(), Note::natural(Natural::E))
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
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::flat(Natural::D);
    /// assert_eq!(note.into_sharp(), Note::sharp(Natural::C))
    /// ```
    ///
    /// Find a natural enharmonic note
    /// ```
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::sharp(Natural::B);
    /// assert_eq!(note.into_sharp(), Note::natural(Natural::C))
    /// ```
    pub const fn into_sharp(self) -> Self {
        Self::from_sharp(self.pitch())
    }

    /// Returns the pitch of the given `Note`.
    /// ```
    /// use music_theory::{Pitch, Natural};
    ///
    /// let pitch = Pitch::natural(Natural::F);
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
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::flat(Natural::D);
    /// assert!(note.is_enharmonic(Note::sharp(Natural::C)))
    /// ```
    ///
    /// This function will also return true if the notes are the same.
    /// ```
    /// use music_theory::{Natural, Note};
    ///
    /// let note = Note::natural(Natural::C);
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

use crate::{Interval, Pitch};
use core::fmt;
use core::ops::{Add, Sub};

mod octave;
pub use octave::Octave;

pub mod message;

mod midi_set;
pub use midi_set::MidiSet;

/// MIDI note represented as a byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MidiNote(u8);

impl MidiNote {
    /// Create a new `MidiNote` from a `Pitch` and `Octave`.
    /// ```
    /// use staff::midi::{Octave, MidiNote};
    /// use staff::Pitch;
    ///
    /// let note = MidiNote::new(Pitch::A, Octave::FOUR);
    /// assert_eq!(note.into_byte(), 69);
    /// assert_eq!(note.to_string(), "A4");
    /// ```
    pub const fn new(pitch: Pitch, octave: Octave) -> Self {
        Self::from_byte(
            (octave.into_i8() + 1) as u8 * (Pitch::B.into_byte() + 1) + pitch.into_byte(),
        )
    }

    /// Create a new `MidiNote` from a byte.
    pub const fn from_byte(byte: u8) -> Self {
        Self(byte)
    }

    /// ```
    /// use staff::midi::MidiNote;
    /// use staff::Pitch;
    ///
    /// let note = MidiNote::from_byte(108);
    /// assert_eq!(note.pitch(), Pitch::C);
    /// ```
    pub const fn pitch(self) -> Pitch {
        Pitch::from_byte(self.into_byte())
    }

    /// ```
    /// use staff::midi::{Octave, MidiNote};
    /// use staff::Pitch;
    ///
    /// let note = MidiNote::new(Pitch::C, Octave::EIGHT);
    /// assert_eq!(note.octave(), Octave::EIGHT);
    /// ```
    ///
    /// Midi notes start at octave -1.
    /// ```
    /// use staff::midi::{Octave, MidiNote};
    ///
    /// let note = MidiNote::from_byte(11);
    /// assert_eq!(note.octave(), Octave::NEGATIVE_ONE);
    /// ```
    pub const fn octave(self) -> Octave {
        Octave::from_midi(self)
    }

    #[cfg(feature = "std")]
    pub fn frequency(self) -> f64 {
        let a_midi = 69;
        let a_freq = 440.;
        a_freq * 2f64.powf((self.into_byte() as i8 - a_midi) as f64 / 12.)
    }

    /// Return the byte representation of `self`.
    pub const fn into_byte(self) -> u8 {
        self.0
    }
}

impl Add<Interval> for MidiNote {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        Self::from_byte(self.into_byte() + rhs.semitones())
    }
}

impl Sub for MidiNote {
    type Output = Interval;

    fn sub(self, rhs: Self) -> Self::Output {
        Interval::new((self.into_byte() as i8 - rhs.into_byte() as i8).abs() as _)
    }
}

impl From<u8> for MidiNote {
    fn from(byte: u8) -> Self {
        Self::from_byte(byte)
    }
}

impl From<MidiNote> for u8 {
    fn from(midi: MidiNote) -> Self {
        midi.into_byte()
    }
}

impl fmt::Display for MidiNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch(), self.octave())
    }
}

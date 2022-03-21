use crate::{Interval, Pitch};
use core::ops::{Add, Sub};

mod display;
pub use display::MidiNoteDisplay;

mod octave;
pub use octave::Octave;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MidiNote(u8);

impl MidiNote {
    /// ```
    /// use music::midi::{Octave, MidiNote};
    /// use music::Pitch;
    ///
    /// let note = MidiNote::new(Pitch::A, Octave::FOUR);
    /// assert_eq!(note.into_byte(), 69);
    /// ```
    pub const fn new(pitch: Pitch, octave: Octave) -> Self {
        Self::from_byte(
            (octave.into_i8() + 1) as u8 * (Pitch::B.into_byte() + 1) + pitch.into_byte(),
        )
    }

    pub const fn from_byte(byte: u8) -> Self {
        Self(byte)
    }

    /// ```
    /// use music::midi::MidiNote;
    /// use music::Pitch;
    ///
    /// let note = MidiNote::from_byte(108);
    /// assert_eq!(note.pitch(), Pitch::C);
    /// ```
    pub const fn pitch(self) -> Pitch {
        Pitch::from_byte(self.into_byte())
    }

    /// ```
    /// use music::midi::{Octave, MidiNote};
    /// use music::Pitch;
    ///
    /// let note = MidiNote::new(Pitch::C, Octave::EIGHT);
    /// assert_eq!(note.octave(), Octave::EIGHT);
    /// ```
    ///
    /// Midi notes start at octave -1.
    /// ```
    /// use music::midi::{Octave, MidiNote};
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

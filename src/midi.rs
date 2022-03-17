use crate::Pitch;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MidiNote(u8);

impl MidiNote {
    pub const fn new(byte: u8) -> Self {
        Self(byte)
    }

    /// ```
    /// use music::midi::MidiNote;
    /// use music::Pitch;
    ///
    /// let note = MidiNote::new(108);
    /// assert_eq!(note.pitch(), Pitch::C);
    /// ```
    pub const fn pitch(self) -> Pitch {
        Pitch::from_byte(self.into_byte())
    }

    /// ```
    /// use music::midi::MidiNote;
    /// use music::Pitch;
    ///
    /// let note = MidiNote::new(108);
    /// assert_eq!(note.octave(), 8);
    /// ```
    ///
    /// Midi notes start at octave -1.
    /// ```
    /// use music::midi::MidiNote;
    /// use music::Pitch;
    ///
    /// let note = MidiNote::new(11);
    /// assert_eq!(note.octave(), -1);
    /// ```
    pub const fn octave(self) -> i8 {
        (self.into_byte() / (Pitch::B.into_byte() + 1)) as i8 - 1
    }

    pub const fn into_byte(self) -> u8 {
        self.0
    }
}

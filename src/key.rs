use core::fmt;
use std::fmt::Write;

use crate::{
    chord::{Chord, ChordKind},
    midi::{MidiNote, Octave},
    note::PitchNote,
    scale::Scale,
    Interval, Pitch,
};

pub struct KeySignature {
    sharps: u8,
}

impl KeySignature {
    /// ```
    /// use music::key::KeySignature;
    /// use music::Pitch;
    ///
    /// let key = KeySignature::major(Pitch::D);
    /// assert_eq!(key.to_string(), "##")
    /// ```
    ///
    /// ```
    /// use music::key::KeySignature;
    /// use music::Pitch;
    ///
    /// let key = KeySignature::major(Pitch::F);
    /// assert_eq!(key.to_string(), "b")
    /// ```
    pub fn major(root: Pitch) -> Self {
        let mut pitch = Pitch::C;
        let mut alteration = 0;
        while pitch != root {
            pitch = pitch + Interval::PERFECT_FIFTH;
            alteration += 1;
        }

        Self { sharps: alteration }
    }

    pub fn sharps(self) -> u8 {
        self.sharps
    }

    pub fn flats(self) -> u8 {
        Pitch::B.into_byte() - self.sharps
    }
}

impl fmt::Display for KeySignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sharps <= 6 {
            for _ in 0..self.sharps {
                f.write_char('#')?;
            }
        } else {
            for _ in self.sharps..=Pitch::B.into_byte() {
                f.write_char('b')?;
            }
        }

        Ok(())
    }
}

pub struct Key {
    octave: Octave,
    scale: Scale<'static>,
    kinds: [ChordKind; 7],
}

impl Key {
    pub fn new(octave: Octave, scale: Scale<'static>, kinds: [ChordKind; 7]) -> Self {
        Self {
            octave,
            scale,
            kinds,
        }
    }

    pub fn major(root: MidiNote) -> Self {
        Self::new(
            root.octave(),
            Scale::major(PitchNote::from_sharp(root.pitch())),
            [
                ChordKind::MajorSeventh,
                ChordKind::MinorSeventh,
                ChordKind::MinorSeventh,
                ChordKind::MajorSeventh,
                ChordKind::DominantSeventh,
                ChordKind::MinorSeventh,
                ChordKind::HalfDiminished,
            ],
        )
    }

    pub fn natural_minor(root: MidiNote) -> Self {
        Self::new(
            root.octave(),
            Scale::natural_minor(PitchNote::from_sharp(root.pitch())),
            [
                ChordKind::MajorSeventh,
                ChordKind::MinorSeventh,
                ChordKind::MinorSeventh,
                ChordKind::MajorSeventh,
                ChordKind::DominantSeventh,
                ChordKind::MinorSeventh,
                ChordKind::HalfDiminished,
            ],
        )
    }

    pub fn chords(self) -> impl Iterator<Item = Chord> {
        self.scale
            .zip(self.kinds.into_iter())
            .map(move |(note, kind)| Chord::new(MidiNote::new(note.pitch(), self.octave), kind))
    }
}

#[cfg(test)]
mod tests {
    use crate::{midi::Octave, Pitch};

    use super::*;

    #[test]
    fn f() {
        let key = Key::major(MidiNote::new(Pitch::C, Octave::FOUR));
        for chord in key.chords() {
            dbg!(chord);
        }
    }
}

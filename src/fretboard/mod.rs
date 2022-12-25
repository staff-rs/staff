//! Fretboard iterator for guitar and other instruments

use crate::{
    chord::{self, Chords},
    midi::{MidiNote, Octave},
    Interval, Pitch,
};
use core::iter::Zip;

pub mod diagram;

pub const STANDARD: [MidiNote; 6] = [
    MidiNote::new(Pitch::E, Octave::FOUR),
    MidiNote::new(Pitch::A, Octave::FOUR),
    MidiNote::new(Pitch::D, Octave::FIVE),
    MidiNote::new(Pitch::G, Octave::FIVE),
    MidiNote::new(Pitch::B, Octave::FIVE),
    MidiNote::new(Pitch::E, Octave::SIX),
];

pub struct Fretboard<T, F>
where
    T: IntoIterator<Item = MidiNote>,
    F: IntoIterator<Item = Option<u8>>,
{
    iter: Zip<T::IntoIter, F::IntoIter>,
}

impl<T, F> Fretboard<T, F>
where
    T: IntoIterator<Item = MidiNote>,
    F: IntoIterator<Item = Option<u8>>,
{
    pub fn new(tuning: T, frets: F) -> Self {
        let iter = tuning.into_iter().zip(frets);
        Self { iter }
    }

    pub fn chords(self) -> Chords<Box<[MidiNote]>> {
        let midi_notes: Vec<_> = self.collect();
        chord::chords(midi_notes.into_boxed_slice())
    }
}

impl<T, F> Iterator for Fretboard<T, F>
where
    T: IntoIterator<Item = MidiNote>,
    F: IntoIterator<Item = Option<u8>>,
{
    type Item = MidiNote;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((string, fret)) = self.iter.next() {
            if let Some(fret) = fret {
                return Some(string + Interval::new(fret));
            }
        }

        None
    }
}

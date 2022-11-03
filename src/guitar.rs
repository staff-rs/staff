use core::iter::Zip;

use crate::{
    midi::{MidiNote, Octave},
    Interval, Pitch,
};

pub const STANDARD: [MidiNote; 6] = [
    MidiNote::new(Pitch::E, Octave::FOUR),
    MidiNote::new(Pitch::A, Octave::FOUR),
    MidiNote::new(Pitch::D, Octave::FIVE),
    MidiNote::new(Pitch::G, Octave::FIVE),
    MidiNote::new(Pitch::B, Octave::FIVE),
    MidiNote::new(Pitch::E, Octave::SIX),
];

pub struct MidiNotes<T, F>
where
    T: IntoIterator<Item = MidiNote>,
    F: IntoIterator<Item = Option<u8>>,
{
    iter: Zip<T::IntoIter, F::IntoIter>,
}

impl<T, F> MidiNotes<T, F>
where
    T: IntoIterator<Item = MidiNote>,
    F: IntoIterator<Item = Option<u8>>,
{
    pub fn new(tuning: T, frets: F) -> Self {
        let iter = tuning.into_iter().zip(frets);
        Self { iter }
    }
}

impl<T, F> Iterator for MidiNotes<T, F>
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

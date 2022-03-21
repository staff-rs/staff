use crate::Interval;
use core::ops::{Add, Sub};

mod display;
pub use display::ChordDisplay;

mod kind;
pub use kind::ChordKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chord<T> {
    pub root: T,
    pub kind: ChordKind,
}

impl<T> Chord<T>
where
    T: Add<Interval> + Copy,
{
    pub fn new(root: T, kind: ChordKind) -> Self {
        Self { root, kind }
    }

    /// Match chords with a given `root` from an iterator of notes.
    /// See [`ChordKind::match_notes`] for more info.
    /// ```
    /// use music::chord::{Chord, ChordKind};
    /// use music::Pitch;
    ///
    /// let root = Pitch::C;
    /// let notes = [
    ///         Pitch::E,
    ///         root,
    ///         Pitch::G
    ///     ];
    ///
    /// let mut matches = Chord::match_notes(root, notes);
    /// assert_eq!(matches.next(), Some(Chord::new(Pitch::C, ChordKind::Major)))
    /// ```
    pub fn match_notes<I>(root: T, notes: I) -> impl Iterator<Item = Self>
    where
        T: Sub<Output = Interval> + Clone,
        I: IntoIterator<Item = T>,
    {
        ChordKind::match_notes(root, notes).map(move |kind| Self::new(root, kind))
    }

    pub fn notes(self) -> impl Iterator<Item = T::Output> {
        self.kind
            .intervals()
            .map(move |interval| self.root + interval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        midi::{MidiNote, MidiNoteDisplay, Octave},
        Pitch,
    };

    #[test]
    fn f() {
        let root = MidiNote::new(Pitch::C, Octave::FOUR);
        let matches = ChordKind::match_notes(
            root,
            [
                MidiNote::new(Pitch::E, Octave::FOUR),
                root,
                MidiNote::new(Pitch::G, Octave::FOUR),
            ],
        );

        for chord in matches {
            dbg!(chord);
        }

        let root = MidiNote::new(Pitch::C, Octave::FOUR);
        let c = ChordDisplay::new(MidiNoteDisplay::from_sharp(root), ChordKind::Minor);
        println!("{}", c);
    }
}

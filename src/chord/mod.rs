use crate::Interval;
use core::ops::{Add, Sub};

mod kind;
pub use kind::ChordKind;

/// Chord with a root note and [`ChordKind`]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chord<T> {
    /// Root note
    pub root: T,
    /// Intervals kind
    pub kind: ChordKind,
}

impl<T> Chord<T>
where
    T: Add<Interval> + Copy,
{
    /// Create a new `Chord` from a root note and `ChordKind`.
    pub fn new(root: T, kind: ChordKind) -> Self {
        Self { root, kind }
    }

    /// Match chords with a given `root` from an iterator of notes.
    /// See [`ChordKind::match_notes`] for more info.
    /// ```
    /// use music_theory::chord::{Chord, ChordKind};
    /// use music_theory::Pitch;
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

    /// Returns an iterator over the notes of `self`.
    pub fn notes(self) -> impl Iterator<Item = T::Output> {
        self.kind
            .intervals()
            .map(move |interval| self.root + interval)
    }
}

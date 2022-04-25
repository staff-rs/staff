use crate::{set::Set, Interval};
use core::ops::Sub;

/// Named hord intervals
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChordKind {
    /// Major
    Major,
    /// Major seventh (maj7)
    MajorSeventh,
    /// Minor (m)
    Minor,
    /// Minor seventh (m7)
    MinorSeventh,
    /// Dominant seventh (7)
    DominantSeventh,
    /// Half diminished (m7b5)
    HalfDiminished,
}

impl ChordKind {
    /// Return an array of all `ChordKind`s.
    pub fn all() -> [Self; 6] {
        [
            Self::Major,
            Self::MajorSeventh,
            Self::Minor,
            Self::MinorSeventh,
            Self::DominantSeventh,
            Self::HalfDiminished,
        ]
    }

    /// Find a `ChordKind` from a [`Set`] of intervals.
    pub fn from_intervals(intervals: Set<Interval>) -> Option<Self> {
        Self::all()
            .into_iter()
            .find(move |kind| kind.intervals() == intervals)
    }

    /// ```
    /// use music_note::chord::ChordKind;
    /// use music_note::midi::{MidiNote, Octave};
    /// use music_note::Pitch;
    ///
    /// let root = MidiNote::new(Pitch::C, Octave::FOUR);
    /// let mut matches = ChordKind::match_notes(
    ///     root,
    ///     [
    ///         MidiNote::new(Pitch::E, Octave::FOUR),
    ///         root,
    ///         MidiNote::new(Pitch::G, Octave::FOUR),
    ///     ],
    /// );
    ///
    /// assert_eq!(matches.next(), Some(ChordKind::Major))
    /// ```
    pub fn match_notes<T, I>(root: T, notes: I) -> impl Iterator<Item = Self>
    where
        T: Sub<Output = Interval> + Clone,
        I: IntoIterator<Item = T>,
    {
        let functions: Set<Interval> = notes.into_iter().map(|note| note - root.clone()).collect();

        functions
            .modes()
            .flat_map(|intervals| Self::from_intervals(intervals).into_iter())
    }

    /// Return the `Set` of intervals for self.
    pub fn intervals(&self) -> Set<Interval> {
        let array: &[_] = match self {
            Self::Major => &[
                Interval::UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
            Self::MajorSeventh => &[
                Interval::UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SEVENTH,
            ],
            Self::Minor => &[
                Interval::UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
            Self::MinorSeventh => &[
                Interval::UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
            ],
            Self::DominantSeventh => &[
                Interval::UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
            ],
            Self::HalfDiminished => &[
                Interval::UNISON,
                Interval::MINOR_THIRD,
                Interval::TRITONE,
                Interval::MINOR_SEVENTH,
            ],
        };
        array.iter().copied().collect()
    }

    /// Return a constant `&'static str` label for self.
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Major => "",
            Self::MajorSeventh => "M7",
            Self::Minor => "m",
            Self::MinorSeventh => "m7",
            Self::DominantSeventh => "7",
            Self::HalfDiminished => "m7b5",
        }
    }
}

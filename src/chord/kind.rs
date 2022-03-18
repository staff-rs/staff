use crate::{midi::MidiNote, set::Set, Interval};

use super::functions;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChordKind {
    Major,
    MajorSeventh,
    Minor,
    MinorSeventh,
    DominantSeventh,
    HalfDiminished,
}

impl ChordKind {
    pub fn all() -> [Self; 2] {
        [Self::Major, Self::Minor]
    }

    pub fn from_intervals(intervals: Set<Interval>) -> Option<Self> {
        Self::all()
            .into_iter()
            .find(move |kind| kind.intervals() == intervals)
    }

    /// ```
    /// use music::chord::ChordKind;
    /// use music::midi::{MidiNote, Octave};
    /// use music::Pitch;
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
    pub fn match_notes<I>(root: MidiNote, notes: I) -> impl Iterator<Item = Self>
    where
        I: IntoIterator<Item = MidiNote>,
    {
        let functions: Set<Interval> = functions(notes, root).collect();
        functions
            .modes()
            .flat_map(|intervals| Self::from_intervals(intervals).into_iter())
    }

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

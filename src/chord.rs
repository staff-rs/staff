use crate::{
    midi::{MidiNote, MidiNoteDisplay},
    set::Set,
    Interval, Pitch,
};
use core::fmt;

pub fn functions<I>(notes: I, root: MidiNote) -> impl Iterator<Item = Interval>
where
    I: IntoIterator<Item = MidiNote>,
{
    notes.into_iter().map(move |note| note - root)
}

#[derive(Clone, Copy, Debug)]
pub struct Chord {
    pub root: MidiNote,
    pub kind: ChordKind,
}

impl Chord {
    pub fn new(root: MidiNote, kind: ChordKind) -> Self {
        Self { root, kind }
    }
}

pub struct ChordDisplay {
    root: MidiNoteDisplay,
    kind: ChordKind,
}

impl ChordDisplay {
    pub fn new(root: MidiNoteDisplay, kind: ChordKind) -> Self {
        Self { root, kind }
    }
}

impl fmt::Display for ChordDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.root, self.kind.to_str())
    }
}

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

    pub fn pitches(&self, root: Pitch) -> impl Iterator<Item = Pitch> {
        self.intervals().map(move |interval| root + interval)
    }

    pub fn notes(&self, root: MidiNote) -> impl Iterator<Item = MidiNote> {
        self.intervals().map(move |interval| root + interval)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{midi::Octave, Pitch};

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

use core::fmt;

use crate::{
    midi::{MidiNote, MidiNoteDisplay},
    set::Set,
    Interval, Pitch,
};

pub fn functions<I>(notes: I, root: MidiNote) -> impl Iterator<Item = Interval>
where
    I: IntoIterator<Item = MidiNote>,
{
    notes.into_iter().map(move |note| note - root)
}

pub struct Chord {
    root: MidiNoteDisplay,
    kind: ChordKind,
}

impl Chord {
    pub fn new(root: MidiNoteDisplay, kind: ChordKind) -> Self {
        Self { root, kind }
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.root, self.kind.to_str())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ChordKind {
    Major,
    Minor,
}

impl ChordKind {
    pub fn all() -> [Self; 2] {
        [Self::Major, Self::Minor]
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Self::Major => "",
            Self::Minor => "m",
        }
    }

    pub fn intervals(&self) -> Set<Interval> {
        let array = match self {
            Self::Major => [
                Interval::UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
            Self::Minor => [
                Interval::UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
        };
        array.into_iter().collect()
    }

    pub fn pitches(&self, root: Pitch) -> impl Iterator<Item = Pitch> {
        self.intervals().map(move |interval| root + interval)
    }

    pub fn notes(&self, root: MidiNote) -> impl Iterator<Item = MidiNote> {
        self.intervals().map(move |interval| root + interval)
    }

    pub fn matches(root: MidiNote, notes: &[MidiNote]) -> impl Iterator<Item = Self> {
        let functions: Set<Interval> = functions(notes.iter().copied(), root).collect();

        functions.modes().flat_map(|intervals| {
            Self::all()
                .into_iter()
                .filter(move |kind| kind.intervals() == intervals)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{midi::Octave, Pitch};

    #[test]
    fn f() {
        let matches = ChordKind::matches(
            MidiNote::new(Pitch::C, Octave::FOUR),
            &[
                MidiNote::new(Pitch::E, Octave::FOUR),
                MidiNote::new(Pitch::G, Octave::FOUR),
                MidiNote::new(Pitch::C, Octave::FOUR),
            ],
        );

        for chord in matches {
            dbg!(chord);
        }

        let root = MidiNote::new(Pitch::C, Octave::FOUR);
        let c = Chord::new(MidiNoteDisplay::from_sharp(root), ChordKind::Minor);
        println!("{}", c);
    }
}

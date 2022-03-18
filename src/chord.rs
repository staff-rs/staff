use crate::{midi::MidiNote, set::Set, Interval};

pub fn major(root: MidiNote) -> [MidiNote; 3] {
    [
        root,
        root + Interval::MAJOR_THIRD,
        root + Interval::PERFECT_FIFTH,
    ]
}

pub fn functions<I>(chord: I, root: MidiNote) -> impl Iterator<Item = Interval>
where
    I: IntoIterator<Item = MidiNote>,
{
    chord.into_iter().map(move |note| note - root)
}

#[derive(Debug)]
pub enum ChordKind {
    Major,
}

impl ChordKind {
    pub fn all() -> [Self; 1] {
        [Self::Major]
    }

    pub fn intervals(&self) -> Set<Interval> {
        match self {
            Self::Major => [
                Interval::UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ]
            .into_iter()
            .collect(),
        }
    }
}

pub fn detect(root: MidiNote, notes: &[MidiNote]) -> impl Iterator<Item = ChordKind> {
    let functions: Set<Interval> = functions(notes.iter().copied(), root).collect();

    functions.modes().flat_map(|intervals| {
        ChordKind::all()
            .into_iter()
            .filter(move |kind| kind.intervals() == intervals)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{midi::Octave, Pitch};

    #[test]
    fn f() {
        let matches = detect(
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
    }
}

use crate::{midi::MidiNote, set::Set, Interval, Pitch};

pub fn functions<I>(notes: I, root: MidiNote) -> impl Iterator<Item = Interval>
where
    I: IntoIterator<Item = MidiNote>,
{
    notes.into_iter().map(move |note| note - root)
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
    }
}

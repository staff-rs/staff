use crate::{midi::MidiNote, Interval, Pitch};

mod display;
pub use display::ChordDisplay;

mod kind;
pub use kind::ChordKind;

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

    pub fn pitches(self) -> impl Iterator<Item = Pitch> {
        self.kind
            .intervals()
            .map(move |interval| self.root.pitch() + interval)
    }

    pub fn notes(self) -> impl Iterator<Item = MidiNote> {
        self.kind
            .intervals()
            .map(move |interval| self.root + interval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        midi::{MidiNoteDisplay, Octave},
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

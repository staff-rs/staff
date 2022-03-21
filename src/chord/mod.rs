use crate::{midi::MidiNote, Interval};
use core::ops::Add;

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

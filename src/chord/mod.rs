use crate::{Interval, Pitch, Set};
use core::ops::Sub;

mod kind;
pub use kind::ChordKind;

#[derive(Clone)]
pub struct Chord {
    root: Pitch,
    bass: Option<Pitch>,
    notes: Set<Pitch>,
}

impl Chord {
    pub fn new(root: Pitch) -> Self {
        Self {
            root,
            bass: None,
            notes: Set::default(),
        }
    }

    pub fn is_rootless(&self) -> bool {
        !self.notes.contains(self.root)
    }
}

impl IntoIterator for Chord {
    type Item = Pitch;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        let (high, low) = if let Some(bass) = self.bass {
            self.notes.clone().split(bass)
        } else {
            (Set::default(), self.notes)
        };
        Iter { low, high }
    }
}

pub struct Iter {
    low: Set<Pitch>,
    high: Set<Pitch>,
}

impl Iterator for Iter {
    type Item = Pitch;

    fn next(&mut self) -> Option<Self::Item> {
        self.low.next().or_else(|| self.high.next())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        midi::{MidiNote, Octave},
        Pitch,
    };

    use super::Chord;

    #[test]
    fn f() {
        let mut c = Chord::new(Pitch::C);
        c.notes.push(Pitch::C);
        c.notes.push(Pitch::E);
        c.notes.push(Pitch::G);

        c.bass = Some(Pitch::E);

        let (a, b) = c.notes.split(Pitch::E);
        dbg!(a.collect::<Vec<_>>());
        dbg!(b.collect::<Vec<_>>());

        let notes: Vec<_> = c.into_iter().collect();
        dbg!(notes);
    }
}

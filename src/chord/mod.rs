use crate::{Interval, Pitch, Set};

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

    pub fn major(root: Pitch) -> Self {
        Self::new(root)
            .pitch(root)
            .interval(Interval::MAJOR_THIRD)
            .interval(Interval::PERFECT_FIFTH)
    }

    pub fn bass(mut self, pitch: Pitch) -> Self {
        self.bass = Some(pitch);
        self
    }

    pub fn pitch(mut self, pitch: Pitch) -> Self {
        self.notes.push(pitch);
        self
    }

    pub fn interval(self, interval: Interval) -> Self {
        let pitch = self.root + interval;
        self.pitch(pitch)
    }

    pub fn seventh(self) -> Self {
        self.interval(Interval::MINOR_SEVENTH)
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
            self.notes.split(bass)
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
    use super::Chord;
    use crate::Pitch;

    #[test]
    fn f() {
        let c = Chord::major(Pitch::C).bass(Pitch::E);
        let notes: Vec<_> = c.into_iter().collect();
        dbg!(notes);
    }
}

use crate::{Interval, Pitch, Set};

mod kind;
pub use kind::ChordKind;

#[derive(Clone, Copy)]
pub struct Chord {
    pub root: Pitch,
    pub bass: Option<Pitch>,
    pub pitches: Set<Pitch>,
}

impl Chord {
    pub fn new(root: Pitch, bass: Option<Pitch>, pitches: Set<Pitch>) -> Self {
        Self {
            root,
            bass,
            pitches,
        }
    }

    pub fn from_root(root: Pitch) -> Self {
        Self::new(root, None, Set::default())
    }

    pub fn triad(root: Pitch, third: Interval, fifth: Interval) -> Self {
        let mut me = Self::from_root(root);
        me.extend([Interval::UNISON, third, fifth]);
        me
    }

    pub fn major(root: Pitch) -> Self {
        Self::triad(root, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH)
    }

    pub fn minor(root: Pitch) -> Self {
        Self::triad(root, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH)
    }

    pub fn half_diminished(root: Pitch) -> Self {
        Self::triad(root, Interval::MINOR_THIRD, Interval::TRITONE)
    }

    /// ```
    /// use music_note::{Chord, Pitch};
    ///
    /// // C Major (1st inversion)
    /// let chord = Chord::major(Pitch::C).with_bass(Pitch::E);
    ///
    /// let notes = [Pitch::E, Pitch::G, Pitch::C];
    /// assert!(chord.into_iter().eq(notes));
    /// ```
    pub fn with_bass(mut self, pitch: Pitch) -> Self {
        self.bass = Some(pitch);
        self
    }

    pub fn with_interval(mut self, interval: Interval) -> Self {
        self.extend([interval]);
        self
    }

    pub fn with_seventh(self) -> Self {
        self.with_interval(Interval::MINOR_SEVENTH)
    }

    pub fn with_major_seventh(self) -> Self {
        self.with_interval(Interval::MAJOR_SEVENTH)
    }

    pub fn is_rootless(self) -> bool {
        !self.pitches.contains(self.root)
    }
}

impl Extend<Pitch> for Chord {
    fn extend<T: IntoIterator<Item = Pitch>>(&mut self, iter: T) {
        self.pitches.extend(iter);
    }
}

impl Extend<Interval> for Chord {
    fn extend<T: IntoIterator<Item = Interval>>(&mut self, iter: T) {
        let root = self.root;
        self.extend(iter.into_iter().map(|interval| root + interval))
    }
}

impl IntoIterator for Chord {
    type Item = Pitch;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        let (high, low) = if let Some(bass) = self.bass {
            self.pitches.split(bass)
        } else {
            (Set::default(), self.pitches)
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
        let c = Chord::major(Pitch::C).with_bass(Pitch::E);
        let notes: Vec<_> = c.into_iter().collect();
        dbg!(notes);
    }
}

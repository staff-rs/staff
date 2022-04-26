use crate::{set::IntervalSet, Interval, Pitch};

pub struct Builder {
    pub bass: Option<Pitch>,
    pub is_inversion: bool,
    pub intervals: IntervalSet,
}

impl Builder {
    pub fn interval(mut self, interval: Interval) -> Self {
        self.intervals.push(interval);
        self
    }

    pub fn root(self) -> Self {
        self.interval(Interval::UNISON)
    }

    pub fn major(self) -> Self {
        self.root()
            .interval(Interval::MAJOR_THIRD)
            .interval(Interval::PERFECT_FIFTH)
    }

    pub fn seventh(self) -> Self {
        self.major().interval(Interval::MINOR_SEVENTH)
    }

    /// ```
    /// use music_note::{Chord, Pitch};
    ///
    /// // C/B
    /// let chord = Chord::builder()
    ///     .major()
    ///     .bass(Pitch::B)
    ///     .build(Pitch::C);
    ///
    /// let notes = [Pitch::B, Pitch::C, Pitch::E, Pitch::G];
    /// assert!(chord.into_iter().eq(notes));
    /// ```
    pub fn bass(mut self, pitch: Pitch) -> Self {
        self.bass = Some(pitch);
        self
    }

    /// ```
    /// use music_note::{Chord, Pitch};
    ///
    /// // C Major (1st inversion)
    /// let chord = Chord::builder()
    ///     .major()
    ///     .inversion(Pitch::E)
    ///     .build(Pitch::C);
    ///
    /// let notes = [Pitch::E, Pitch::G, Pitch::C];
    /// assert!(chord.into_iter().eq(notes));
    /// ```
    pub fn inversion(mut self, pitch: Pitch) -> Self {
        self.is_inversion = true;
        self.bass(pitch)
    }

    pub fn build(self, root: Pitch) -> Chord {
        Chord {
            root,
            builder: self,
        }
    }
}

pub struct Chord {
    root: Pitch,
    builder: Builder,
}

impl Chord {
    pub fn builder() -> Builder {
        Builder {
            bass: None,
            is_inversion: false,
            intervals: IntervalSet::default(),
        }
    }

    pub fn intervals(self) -> Intervals {
        let (high, low) = if let Some(bass) = self.builder.bass {
            let bass_interval =
                Interval::new((self.root.into_byte() as i8 - bass.into_byte() as i8).abs() as u8);
            if self.builder.is_inversion {
                self.builder.intervals.split(bass_interval)
            } else {
                (
                    self.builder.intervals,
                    [bass_interval].into_iter().collect(),
                )
            }
        } else {
            (IntervalSet::default(), self.builder.intervals)
        };

        Intervals { low, high }
    }
}

impl IntoIterator for Chord {
    type Item = Pitch;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            root: self.root,
            intervals: self.intervals(),
        }
    }
}

pub struct Intervals {
    low: IntervalSet,
    high: IntervalSet,
}

impl Iterator for Intervals {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        self.low.next().or_else(|| self.high.next())
    }
}

pub struct Iter {
    root: Pitch,
    intervals: Intervals,
}

impl Iterator for Iter {
    type Item = Pitch;

    fn next(&mut self) -> Option<Self::Item> {
        self.intervals.next().map(|interval| self.root + interval)
    }
}

/*
#[derive(Clone, Copy)]
pub struct Chord {
    pub root: Pitch,
    pub bass: Option<Pitch>,
    pub is_inversion: bool,
    pub pitches: Set<Interval, >,
}

impl Chord {
    pub fn new(root: Pitch, bass: Option<Pitch>, is_inversion: bool, pitches: Set<Pitch>) -> Self {
        Self {
            root,
            bass,
            is_inversion,
            pitches,
        }
    }

    pub fn from_root(root: Pitch) -> Self {
        Self::new(root, None, false, Set::default())
    }

    pub fn triad(root: Pitch, third: Interval, fifth: Interval) -> Self {
        let mut me = Self::from_root(root);
        me.extend([Interval::UNISON, third, fifth]);
        me
    }

    /// ```
    /// use music_note::{Chord, Pitch};
    ///
    /// // C Major
    /// let chord = Chord::major(Pitch::C);
    ///
    /// let notes = [Pitch::C, Pitch::E, Pitch::G];
    /// assert!(chord.into_iter().eq(notes));
    /// ```
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
    /// let chord = Chord::major(Pitch::C).inversion(Pitch::E);
    ///
    /// let notes = [Pitch::E, Pitch::G, Pitch::C];
    /// assert!(chord.into_iter().eq(notes));
    /// ```
    pub fn inversion(mut self, pitch: Pitch) -> Self {
        self.is_inversion = true;
        self.with_bass(pitch)
    }

    /// ```
    /// use music_note::{Chord, Pitch};
    ///
    /// // C/B
    /// let chord = Chord::major(Pitch::C).with_bass(Pitch::B);
    ///
    /// let notes = [Pitch::B, Pitch::C, Pitch::E, Pitch::G];
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

    pub fn inversions(self) -> Inversions {
        Inversions {
            chord: self,
            pitches: self.pitches,
        }
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
            if self.is_inversion {
                self.pitches.split(bass)
            } else {
                (self.pitches, [bass].into_iter().collect())
            }
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

*/

use crate::{set::IntervalSet, Chord, Interval, Pitch};

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

    /// ```
    /// use music_note::{Chord, Pitch};
    ///
    /// // C/B
    /// let chord = Chord::major()
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
    /// let chord = Chord::major()
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

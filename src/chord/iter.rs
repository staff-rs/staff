use crate::{set::IntervalSet, Interval, Pitch};


pub struct Intervals {
    pub(super) low: IntervalSet,
    pub(super) high: IntervalSet,
}

impl Iterator for Intervals {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        self.low.next().or_else(|| self.high.next())
    }
}

pub struct Iter {
    pub(super) root: Pitch,
    pub(super) intervals: Intervals,
}

impl Iterator for Iter {
    type Item = Pitch;

    fn next(&mut self) -> Option<Self::Item> {
        self.intervals.next().map(|interval| self.root + interval)
    }
}
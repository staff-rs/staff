use crate::{set::Set, Interval};

pub type ScaleIntervals = Set<Interval, u16>;

impl ScaleIntervals {
    pub fn major() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SIXTH,
            Interval::MAJOR_SEVENTH,
        ])
    }

    pub fn natural_minor() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SIXTH,
            Interval::MINOR_SEVENTH,
        ])
    }

    pub fn harmonic_minor() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SIXTH,
            Interval::MAJOR_SEVENTH,
        ])
    }

    pub fn melodic_minor() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SIXTH,
            Interval::MAJOR_SEVENTH,
        ])
    }

    pub fn blues() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::TRITONE,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SEVENTH,
        ])
    }

    pub fn dorian() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SIXTH,
            Interval::MINOR_SEVENTH,
        ])
    }
}

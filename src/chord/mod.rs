use crate::{set::IntervalSet, Interval, Pitch};
use core::fmt::{self, Write};

mod builder;
pub use builder::Builder;

pub struct Chord {
    root: Pitch,
    builder: Builder,
}

impl Chord {
    pub fn major() -> Builder {
        Self::builder()
            .root()
            .interval(Interval::MAJOR_THIRD)
            .interval(Interval::PERFECT_FIFTH)
    }

    pub fn minor() -> Builder {
        Self::builder()
            .root()
            .interval(Interval::MAJOR_THIRD)
            .interval(Interval::PERFECT_FIFTH)
    }

    pub fn seventh() -> Builder {
        Self::major().interval(Interval::MINOR_SEVENTH)
    }

    pub fn major_seventh() -> Builder {
        Self::major().interval(Interval::MAJOR_SEVENTH)
    }

    pub fn minor_seventh() -> Builder {
        Self::minor().interval(Interval::MINOR_SEVENTH)
    }

    pub fn minor_major_seventh() -> Builder {
        Self::minor().interval(Interval::MAJOR_SEVENTH)
    }

    pub fn builder() -> Builder {
        Builder {
            bass: None,
            is_inversion: false,
            intervals: IntervalSet::default(),
        }
    }

    pub fn root(self) -> Pitch {
        self.root
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

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        f.write_char('C')?;

        if self.builder.intervals.contains(Interval::MINOR_THIRD) {
            f.write_char('m')?
        } else if self.builder.intervals.contains(Interval::MAJOR_SECOND) {
            f.write_str("sus2")?
        } else if self.builder.intervals.contains(Interval::PERFECT_FOURTH) {
            f.write_str("sus4")?
        }

        let mut has_fifth = true;
        if self.builder.intervals.contains(Interval::TRITONE) {
            f.write_str("b5")?
        } else if !self.builder.intervals.contains(Interval::PERFECT_FIFTH) {
            has_fifth = false;
        }

        if self.builder.intervals.contains(Interval::MINOR_SEVENTH) {
            f.write_char('7')?
        } else if self.builder.intervals.contains(Interval::MAJOR_SEVENTH) {
            f.write_str("maj7")?
        }

        if !self.builder.intervals.contains(Interval::UNISON) {
            f.write_str("(no root)")?
        }

        if !has_fifth {
            f.write_str("(no5)")?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Chord, Pitch};

    #[test]
    fn f() {
        let chord = Chord::seventh().build(Pitch::C);
        println!("{}", chord);
    }
}

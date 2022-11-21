//! Format trait to adjust `fmt::Display` output

use core::fmt;
use std::fmt::{Display, Write};

use crate::{midi::MidiNote, set::IntervalSet, Chord, Interval};

/// ```
/// use staff::{midi, Chord, Format};
///
/// let chord = Chord::minor(midi!(C, 4));
/// let s = format!("{}", chord.into_fmt().show_octave(true));
/// assert_eq!(s, "C4m")
/// ```
pub trait Format {
    fn into_fmt(self) -> Formatter<Self>
    where
        Self: Sized,
    {
        Formatter {
            t: self,
            show_octave: false,
        }
    }

    fn fmt_with_octave(&self, f: &mut fmt::Formatter, show_octave: bool) -> fmt::Result;
}

impl<T: Format> Format for &T {
    fn fmt_with_octave(&self, f: &mut fmt::Formatter, show_octave: bool) -> fmt::Result {
        (*self).fmt_with_octave(f, show_octave)
    }
}

pub struct Formatter<T> {
    t: T,
    show_octave: bool,
}

impl<T> Formatter<T> {
    pub fn show_octave(mut self, show_octave: bool) -> Self {
        self.show_octave = show_octave;
        self
    }
}

impl<T: Format> fmt::Display for Formatter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.t.fmt_with_octave(f, self.show_octave)
    }
}

impl Format for MidiNote {
    fn fmt_with_octave(&self, f: &mut fmt::Formatter, show_octave: bool) -> fmt::Result {
        self.pitch().fmt(f)?;

        if show_octave {
            self.octave().fmt(f)
        } else {
            Ok(())
        }
    }
}

impl Format for Chord {
    fn fmt_with_octave(&self, f: &mut fmt::Formatter, show_octave: bool) -> fmt::Result {
        self.root.fmt_with_octave(f, show_octave)?;

        let mut intervals: IntervalSet = self.clone().intervals().collect();

        if intervals.maybe_remove(Interval::MAJOR_THIRD).is_none() {
            if intervals.maybe_remove(Interval::MINOR_THIRD).is_some() {
                f.write_char('m')?
            } else if intervals.maybe_remove(Interval::MAJOR_SECOND).is_some() {
                f.write_str("sus2")?
            } else if intervals.maybe_remove(Interval::PERFECT_FOURTH).is_some() {
                f.write_str("sus4")?
            }
        }

        let mut has_fifth = true;
        if intervals.maybe_remove(Interval::TRITONE).is_some() {
            f.write_str("b5")?
        } else if intervals.maybe_remove(Interval::PERFECT_FIFTH).is_none() {
            has_fifth = false;
        }

        if intervals.maybe_remove(Interval::MINOR_SEVENTH).is_some() {
            if intervals.maybe_remove(Interval::MAJOR_NINTH).is_some() {
                if intervals.maybe_remove(Interval::MINOR_ELEVENTH).is_some() {
                    if intervals.maybe_remove(Interval::MINOR_THIRTEENTH).is_some() {
                        f.write_str("13")?
                    } else {
                        f.write_str("11")?
                    }
                } else {
                    f.write_str("9")?
                }
            } else {
                f.write_str("7")?
            }
        } else if intervals.maybe_remove(Interval::MAJOR_SEVENTH).is_some() {
            if intervals.maybe_remove(Interval::MAJOR_NINTH).is_some() {
                if intervals.maybe_remove(Interval::MAJOR_ELEVENTH).is_some() {
                    if intervals.maybe_remove(Interval::MAJOR_THIRTEENTH).is_some() {
                        f.write_str("maj13")?
                    } else {
                        f.write_str("maj11")?
                    }
                } else {
                    f.write_str("maj9")?
                }
            } else {
                f.write_str("maj7")?
            }
        }

        if let Some(bass) = self.bass {
            f.write_char('/')?;
            bass.fmt_with_octave(f, show_octave)?;
        }

        if !intervals.maybe_remove(Interval::UNISON).is_some() {
            f.write_str("(no root)")?
        }

        if !has_fifth {
            f.write_str("(no5)")?
        }

        for alteration in intervals {
            write!(f, "(add{})", alteration)?;
        }

        Ok(())
    }
}

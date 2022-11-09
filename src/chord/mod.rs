use crate::{
    midi::{MidiNote, Octave},
    set::IntervalSet,
    Interval, Natural, Note, Pitch,
};
use core::{
    fmt::{self, Write},
    str::FromStr,
};

mod builder;
pub use builder::Builder;

mod iter;
pub use self::iter::{Chords, Intervals, Iter};

/// ```
/// use staff::{chord, midi, Pitch, Chord};
///
/// let notes = [midi!(C, 4),midi!(E, 4), midi!(G, 4)];
/// let chords = chord::chords(&notes);
///
/// let names = chords.map(|chord| chord.to_string());
/// assert!(names.eq(["C", "Em/C(no5)", "Gm/C"]));
/// ```
pub fn chords<T>(midi_notes: T) -> Chords<T>
where
    T: AsRef<[MidiNote]>,
{
    Chords::new(midi_notes)
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chord {
    root: Pitch,
    #[cfg_attr(feature = "serde", serde(flatten))]
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
            .interval(Interval::MINOR_THIRD)
            .interval(Interval::PERFECT_FIFTH)
    }

    /// ```
    /// use staff::{Chord, Pitch};
    ///
    /// // D7
    /// let chord = Chord::seventh().build(Pitch::D);
    ///
    /// let notes = [Pitch::D, Pitch::FSharp, Pitch::A, Pitch::C];
    /// assert!(chord.into_iter().eq(notes));
    /// ```
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

    pub fn half_diminished() -> Builder {
        Self::builder()
            .root()
            .interval(Interval::MINOR_THIRD)
            .interval(Interval::TRITONE)
            .interval(Interval::MINOR_SEVENTH)
    }

    pub fn builder() -> Builder {
        Builder {
            bass: None,
            is_inversion: false,
            intervals: IntervalSet::default(),
        }
    }

    /// ```
    /// use staff::{midi, Chord, Pitch};
    ///
    /// let chord = Chord::from_midi(
    ///     midi!(C, 4),
    ///     [midi!(E, 3), midi!(G, 3), midi!(C, 4)]
    /// );
    ///
    /// assert_eq!(chord.to_string(), "C/E");
    ///
    /// let pitches = [Pitch::E, Pitch::G, Pitch::C];
    /// assert!(chord.into_iter().eq(pitches));
    /// ```
    pub fn from_midi<I>(root: MidiNote, iter: I) -> Self
    where
        I: IntoIterator<Item = MidiNote>,
    {
        let mut iter = iter.into_iter();
        let mut intervals = IntervalSet::default();
        let mut is_inversion = false;

        let bass_note = iter.next().unwrap();
        let root_pitch = root.pitch();
        let bass = if bass_note != root {
            is_inversion = true;

            let bass_pitch = bass_note.pitch();
            intervals.push(bass_pitch.abs_diff(root_pitch));

            Some(bass_note.pitch())
        } else {
            intervals.push(Interval::UNISON);
            None
        };

        if let Some(note) = iter.next() {
            intervals.push(note.pitch().abs_diff(root_pitch));
            intervals.extend(iter.map(|midi| midi - root));
        }

        Self {
            root: root.pitch(),
            builder: Builder {
                bass,
                is_inversion,
                intervals,
            },
        }
    }

    pub fn root(&self) -> Pitch {
        self.root
    }

    pub fn intervals(self) -> Intervals {
        // TODO maybe use rotate_right?
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

    pub fn midi_notes(self, octave: Octave) -> MidiNotes {
        MidiNotes {
            root: MidiNote::new(self.root, octave),
            intervals: self.intervals(),
        }
    }
}

pub struct MidiNotes {
    root: MidiNote,
    intervals: Intervals,
}

impl Iterator for MidiNotes {
    type Item = MidiNote;

    fn next(&mut self) -> Option<Self::Item> {
        self.intervals.next().map(|interval| self.root + interval)
    }
}

impl FromIterator<MidiNote> for Chord {
    fn from_iter<T: IntoIterator<Item = MidiNote>>(iter: T) -> Self {
        let mut notes = iter.into_iter();
        let root = notes.next().unwrap_or(MidiNote::from_byte(0));

        Self::from_midi(root, core::iter::once(root).chain(notes))
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

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.root.fmt(f)?;

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

        if let Some(bass) = self.builder.bass {
            write!(f, "/{}", bass)?;
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

impl FromStr for Chord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let natural: Natural = chars.next().unwrap().try_into().unwrap();

        let mut next = chars.next();
        let root: Pitch = match next {
            Some('b') => {
                next = chars.next();
                if next == Some('b') {
                    next = chars.next();
                    Note::double_flat(natural).into()
                } else {
                    Note::flat(natural).into()
                }
            }
            Some('#') => {
                next = chars.next();
                if next == Some('#') {
                    next = chars.next();
                    Note::double_sharp(natural).into()
                } else {
                    Note::sharp(natural).into()
                }
            }
            _ => natural.into(),
        };

        let mut builder = match next {
            Some('m') => {
                next = chars.next();
                Chord::minor()
            }
            _ => Chord::major(),
        };

        loop {
            if let Some(c) = next {
                match c {
                    '7' => builder.intervals.push(Interval::MINOR_SEVENTH),
                    _ => todo!(),
                }
                next = chars.next();
            } else {
                break;
            }
        }

        Ok(builder.build(root))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Chord, Pitch};

    #[test]
    fn it_parses_d_double_sharp_major() {
        let chord: Chord = "D##".parse().unwrap();
        assert_eq!(chord, Chord::major().build(Pitch::E));
    }

    #[test]
    fn it_parses_c_minor_seven() {
        let chord: Chord = "Cm7".parse().unwrap();
        assert_eq!(chord, Chord::minor_seventh().build(Pitch::C));
    }
}

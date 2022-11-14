use crate::{
    midi::{MidiNote, Octave},
    set::IntervalSet,
    Interval, Natural, Note, Pitch,
};
use core::{
    fmt::{self, Write},
    str::FromStr,
};

mod iter;
pub use self::iter::{Chords, Iter, Intervals};

/*
/// ```
/// use staff::{chord, midi, Pitch, Chord};
///
/// let notes = [midi!(C, 4),midi!(E, 4), midi!(G, 4)];
/// let chords = chord::chords(&notes);
///
/// let names = chords.map(|chord| chord.to_string());
/// assert!(names.eq(["C", "Em/C(no5)", "Gm/C"]));
/// ```
*/
pub fn chords<T>(midi_notes: T) -> Chords<T>
where
    T: AsRef<[MidiNote]>,
{
    Chords::new(midi_notes)
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chord {
    pub root: MidiNote,
    pub bass: Option<MidiNote>,
    pub is_inversion: bool,
    pub intervals: IntervalSet,
}

impl Chord {
    pub fn new(root: MidiNote) -> Self {
        Self {
            root,
            bass: None,
            is_inversion: false,
            intervals: IntervalSet::default(),
        }
    }

    pub fn with_bass(mut self, bass_note: MidiNote) -> Self {
        self.bass = Some(bass_note);
        self
    }

    pub fn with_inversion(mut self, bass_note: MidiNote) -> Self {
        self.is_inversion = true;
        self.with_bass(bass_note)
    }

    pub fn with_interval(mut self, interval: Interval) -> Self {
        self.intervals.push(interval);
        self
    }

    pub fn root(self) -> Self {
        self.with_interval(Interval::UNISON)
    }

    /// ```
    /// use staff::{Chord, midi};
    ///
    /// let chord = Chord::major(midi!(C, 4))
    ///     .major_seventh()
    ///     .major_ninth();
    ///
    /// let midi_notes = [
    ///     midi!(C, 4),
    ///     midi!(E, 4),
    ///     midi!(G, 4),
    ///     midi!(B, 4),
    ///     midi!(D, 5),
    /// ];
    ///
    /// assert!(chord.into_iter().eq(midi_notes));
    /// ```
    pub fn major(root: MidiNote) -> Self {
        Self::new(root)
            .root()
            .with_interval(Interval::MAJOR_THIRD)
            .with_interval(Interval::PERFECT_FIFTH)
    }

    pub fn minor(root: MidiNote) -> Self {
        Self::new(root)
            .root()
            .with_interval(Interval::MINOR_THIRD)
            .with_interval(Interval::PERFECT_FIFTH)
    }

    pub fn seventh(root: MidiNote) -> Self {
        Self::major(root).with_interval(Interval::MINOR_SEVENTH)
    }

    pub fn major_seventh(self) -> Self {
        self.with_interval(Interval::MAJOR_SEVENTH)
    }

    pub fn minor_seventh(root: MidiNote) -> Self {
        Self::minor(root).with_interval(Interval::MINOR_SEVENTH)
    }

    pub fn major_ninth(self) -> Self {
        self.with_interval(Interval::MAJOR_NINTH)
    }

    pub fn half_diminished(root: MidiNote) -> Self {
        Self::new(root)
            .root()
            .with_interval(Interval::MINOR_THIRD)
            .with_interval(Interval::TRITONE)
            .with_interval(Interval::MINOR_SEVENTH)
    }

    /// ```
    /// use staff::{midi, Chord, Pitch};
    ///
    /// let notes = [midi!(E, 3), midi!(G, 3), midi!(C, 4)];
    /// let chord = Chord::from_midi(midi!(C, 4), notes).unwrap();
    ///
    /// assert_eq!(chord.to_string(), "C4/E3");
    ///
    /// assert!(chord.into_iter().eq(notes));
    /// ```
    pub fn from_midi<I>(root: MidiNote, iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = MidiNote>,
    {
        let mut iter = iter.into_iter();
        let mut intervals = IntervalSet::default();
        let mut is_inversion = false;

        let bass_note = iter.next()?;

        let bass = if bass_note != root {
            is_inversion = true;
            Some(bass_note)
        } else {
            None
        };
        intervals.push(Interval::UNISON);

        let lowest_note = bass.unwrap_or(root);
        intervals.extend(iter.map(|midi| midi - lowest_note));

        Some(Self {
            root,
            bass,
            is_inversion,
            intervals,
        })
    }

    /// Returns the bass, or lowest, note of the chord.
    /// ```
    /// use staff::{midi, Chord};
    ///
    /// let chord = Chord::major(midi!(C, 3))
    ///     .with_bass(midi!(E, 4));
    ///
    /// assert_eq!(chord.bass(), midi!(E, 4))
    /// ```
    /// Returns the root note if no other bass note is present.
    /// ```
    /// use staff::{midi, Chord};
    ///
    /// let chord = Chord::major(midi!(G, 4));
    /// assert_eq!(chord.bass(), midi!(G, 4))
    /// ```
    pub fn bass(&self) -> MidiNote {
        self.bass.unwrap_or(self.root)
    }

    /// ```
    /// use staff::{midi, Chord, Interval};
    ///
    /// let chord = Chord::major(midi!(C, 4));
    ///
    /// let intervals = [Interval::UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH];
    /// assert!(chord.intervals().eq(intervals));
    /// ```
    pub fn intervals(self) -> Intervals {
        self.into()
    }

    pub fn midi_notes(self) -> MidiNotes {
        MidiNotes {
            root: self.bass(),
            intervals: self.intervals,
        }
    }
}

pub struct MidiNotes {
    root: MidiNote,
    intervals: IntervalSet,
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

        Self::from_midi(root, core::iter::once(root).chain(notes)).unwrap()
    }
}

impl IntoIterator for Chord {
    type Item = MidiNote;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            root: self.bass(),
            intervals: self.intervals,
        }
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.root.fmt(f)?;

        let intervals: IntervalSet = self.clone().intervals().collect();

        if intervals.contains(Interval::MINOR_THIRD) {
            f.write_char('m')?
        } else if intervals.contains(Interval::MAJOR_SECOND) {
            f.write_str("sus2")?
        } else if intervals.contains(Interval::PERFECT_FOURTH) {
            f.write_str("sus4")?
        }

        let mut has_fifth = true;
        if intervals.contains(Interval::TRITONE) {
            f.write_str("b5")?
        } else if !intervals.contains(Interval::PERFECT_FIFTH) {
            has_fifth = false;
        }

        if intervals.contains(Interval::MINOR_SEVENTH) {
            f.write_char('7')?
        } else if intervals.contains(Interval::MAJOR_SEVENTH) {
            if intervals.contains(Interval::MAJOR_NINTH) {
                if intervals.contains(Interval::MAJOR_ELEVENTH) {
                    if intervals.contains(Interval::MAJOR_THIRTEENTH) {
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
            write!(f, "/{}", bass)?;
        }

        if !intervals.contains(Interval::UNISON) {
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

        let mut chord = match next {
            Some('m') => {
                next = chars.next();
                Chord::minor(MidiNote::new(root, Octave::FOUR))
            }
            _ => Chord::major(MidiNote::new(root, Octave::FOUR)),
        };

        loop {
            if let Some(c) = next {
                match c {
                    'b' => match chars.next() {
                        Some(c) => match c {
                            '5' => chord.intervals.push(Interval::TRITONE),
                            _ => todo!(),
                        },
                        None => break,
                    },
                    '7' => chord.intervals.push(Interval::MINOR_SEVENTH),
                    _ => todo!(),
                }
                next = chars.next();
            } else {
                break;
            }
        }

        Ok(chord)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        midi::{MidiNote, Octave},
        Chord, Pitch,
    };

    #[test]
    fn it_parses_d_double_sharp_major() {
        let chord: Chord = "D##".parse().unwrap();
        assert_eq!(chord, Chord::major(MidiNote::new(Pitch::E, Octave::FOUR)));
    }

    #[test]
    fn it_parses_c_minor_seven() {
        let chord: Chord = "Cm7".parse().unwrap();
        assert_eq!(
            chord,
            Chord::minor_seventh(MidiNote::new(Pitch::C, Octave::FOUR))
        );
    }

    #[test]
    fn it_collects_from_maj_13() {
        let chord = Chord::from_midi(
            MidiNote::new(Pitch::C, Octave::FOUR),
            [
                MidiNote::new(Pitch::C, Octave::FOUR),
                MidiNote::new(Pitch::E, Octave::FOUR),
                MidiNote::new(Pitch::G, Octave::FOUR),
                MidiNote::new(Pitch::B, Octave::FOUR),
                MidiNote::new(Pitch::D, Octave::FIVE),
                MidiNote::new(Pitch::F, Octave::FIVE),
                MidiNote::new(Pitch::A, Octave::FIVE),
            ],
        )
        .unwrap();

        assert_eq!(chord.to_string(), "C4maj13");
    }
}

use crate::{midi::MidiNote, set::IntervalSet, Chord, Interval};

pub struct MidiNotes {
    root: MidiNote,
    intervals: IntervalSet,
}

impl From<Chord> for MidiNotes {
    fn from(chord: Chord) -> Self {
        Self {
            root: chord.bass(),
            intervals: chord.intervals,
        }
    }
}

impl Iterator for MidiNotes {
    type Item = MidiNote;

    fn next(&mut self) -> Option<Self::Item> {
        self.intervals.next().map(|interval| self.root + interval)
    }
}

pub struct Intervals {
    chord: Chord,
}

impl From<Chord> for Intervals {
    fn from(chord: Chord) -> Self {
        Self { chord }
    }
}

impl Iterator for Intervals {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        self.chord.intervals.next().map(|interval| {
            let midi_note = self.chord.bass.unwrap_or(self.chord.root) + interval;
            midi_note.abs_diff(self.chord.root)
        })
    }
}

pub struct Chords<T> {
    midi_notes: T,
    pos: usize,
}

impl<T> Chords<T>
where
    T: AsRef<[MidiNote]>,
{
    pub fn new(midi_notes: T) -> Self {
        Self { midi_notes, pos: 0 }
    }
}

impl<T> Iterator for Chords<T>
where
    T: AsRef<[MidiNote]>,
{
    type Item = Chord;

    fn next(&mut self) -> Option<Self::Item> {
        self.midi_notes.as_ref().get(self.pos).and_then(|root| {
            self.pos += 1;
            Chord::from_midi(*root, self.midi_notes.as_ref().iter().copied())
        })
    }
}

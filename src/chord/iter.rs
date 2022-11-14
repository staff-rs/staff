use crate::{midi::MidiNote, set::IntervalSet, Chord, Interval};

pub struct Iter {
    pub(super) root: MidiNote,
    pub(super) intervals: IntervalSet,
}

impl Iterator for Iter {
    type Item = MidiNote;

    fn next(&mut self) -> Option<Self::Item> {
        self.intervals.next().map(|interval| self.root + interval)
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

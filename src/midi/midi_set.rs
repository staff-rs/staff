use super::MidiNote;
use crate::Set;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MidiSet {
    low: Set<MidiNote, u64>,
    high: Set<MidiNote, u64>,
}

impl MidiSet {
    pub fn push(&mut self, midi: MidiNote) {
        if midi > MidiNote::from_byte(63) {
            let byte = midi.into_byte() - 63;
            self.high.push(MidiNote::from(byte))
        } else {
            self.low.push(midi)
        }
    }
}

impl Iterator for MidiSet {
    type Item = MidiNote;

    fn next(&mut self) -> Option<Self::Item> {
        self.low.next().or_else(|| {
            self.high
                .next()
                .map(|midi| MidiNote::from(midi.into_byte() + 63))
        })
    }
}

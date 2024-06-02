use super::MidiNote;
use crate::set::Set;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MidiSet {
    low: Set<MidiNote, u64>,
    high: Set<MidiNote, u64>,
}

impl MidiSet {
    pub fn contains(&self, midi: MidiNote) -> bool {
        with_midi(self.low, self.high, midi, |set, midi| set.contains(midi))
    }

    pub fn split(self, midi: MidiNote) -> (Self, Self) {
        if midi <= MidiNote::from_byte(63) {
            let (lower_low, upper_low) = self.low.split(midi);
            (
                MidiSet {
                    low: lower_low,
                    ..Default::default()
                },
                MidiSet {
                    low: upper_low,
                    high: self.high,
                },
            )
        } else {
            let (lower_high, upper_high) = self.high.split(midi);
            (
                MidiSet {
                    low: self.low,
                    high: lower_high,
                },
                MidiSet {
                    high: upper_high,
                    ..Default::default()
                },
            )
        }
    }

    pub fn push(&mut self, midi: MidiNote) {
        self.inner(midi, |set, midi| set.push(midi))
    }

    pub fn remove(&mut self, midi: MidiNote) {
        self.inner(midi, |set, midi| set.remove(midi))
    }

    fn inner<F, T>(&mut self, midi: MidiNote, f: F) -> T
    where
        F: FnOnce(&mut Set<MidiNote, u64>, MidiNote) -> T,
    {
        with_midi(&mut self.low, &mut self.high, midi, f)
    }
}

fn with_midi<T, U, F>(low: T, high: T, midi: MidiNote, f: F) -> U
where
    F: FnOnce(T, MidiNote) -> U,
{
    if midi <= MidiNote::from_byte(63) {
        f(low, midi)
    } else {
        let byte = midi.into_byte() - 64;
        f(high, MidiNote::from(byte))
    }
}

impl FromIterator<MidiNote> for MidiSet {
    fn from_iter<T: IntoIterator<Item = MidiNote>>(iter: T) -> Self {
        let mut set = Self::default();
        set.extend(iter);
        set
    }
}

impl Extend<MidiNote> for MidiSet {
    fn extend<T: IntoIterator<Item = MidiNote>>(&mut self, iter: T) {
        for midi in iter {
            self.push(midi);
        }
    }
}

impl Iterator for MidiSet {
    type Item = MidiNote;

    fn next(&mut self) -> Option<Self::Item> {
        self.low.next().or_else(|| {
            self.high
                .next()
                .map(|midi| MidiNote::from(midi.into_byte() + 64))
        })
    }
}

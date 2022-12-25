use super::Fretboard;
use crate::midi::MidiNote;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fretted {
    pub fret: u8,
    pub start: u8,
    pub end: u8,
}

impl Fretted {
    pub fn new(fret: u8, start: u8, end: u8) -> Self {
        Self { fret, start, end }
    }

    pub fn point(fret: u8, string: u8) -> Self {
        Self::new(fret, string, string + 1)
    }

    pub fn muted(fret: u8, string: u8) -> Self {
        Self::new(fret, string, string)
    }

    pub fn is_intersection(&self, other: &Self) -> bool {
        self.fret == other.fret && self.start < other.end && self.end > other.start
    }
}

pub struct Diagram {
    pub strings: u8,
    pub frets: u8,
    pub starting_fret: u8,
    pub fretted: Vec<Fretted>,
}

impl Default for Diagram {
    fn default() -> Self {
        Self {
            strings: 6,
            frets: 6,
            starting_fret: 0,
            fretted: Vec::new(),
        }
    }
}

impl Diagram {
    pub fn insert(&mut self, fret: Fretted) -> Option<usize> {
        if fret.fret >= self.frets || fret.start > self.strings || fret.end > self.strings {
            return None;
        }

        if let Some(idx) = self.intersection(&fret) {
            Some(idx)
        } else {
            self.fretted.push(fret);
            None
        }
    }

    pub fn intersection(&self, fret: &Fretted) -> Option<usize> {
        self.fretted.iter().position(|f| f.is_intersection(fret))
    }

    pub fn midi_notes<I: IntoIterator<Item = MidiNote>>(
        &self,
        tuning: I,
    ) -> Fretboard<I, Vec<Option<u8>>> {
        let mut frets: Vec<Option<u8>> = vec![None; self.strings as _];
        for fretted in &self.fretted {
            let pos = fretted.fret + self.starting_fret;
            for idx in fretted.start..fretted.end {
                if let Some(last) = &mut frets[idx as usize] {
                    *last = (*last).max(pos);
                } else {
                    frets[idx as usize] = Some(pos);
                }
            }
        }

        Fretboard::new(tuning, frets)
    }
}

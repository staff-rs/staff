use super::Fretboard;
use crate::midi::MidiNote;

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StringRange {
    pub start: u8,
    pub end: u8,
}

impl StringRange {
    pub fn new(start: u8, end: u8) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fretted {
    pub pos: u8,
    pub strings: StringRange,
}

impl Fretted {
    pub fn new(pos: u8, strings: StringRange) -> Self {
        Self { pos, strings }
    }

    pub fn barre(pos: u8, start: u8, end: u8) -> Self {
        Self::new(pos, StringRange::new(start, end))
    }

    pub fn point(pos: u8, string: u8) -> Self {
        Self::barre(pos, string, string + 1)
    }

    pub fn muted(pos: u8, string: u8) -> Self {
        Self::barre(pos, string, string)
    }

    pub fn is_intersection(&self, other: &Self) -> bool {
        self.pos == other.pos
            && self.strings.start < other.strings.end
            && self.strings.end > other.strings.start
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
        if fret.pos >= self.frets
            || fret.strings.start > self.strings
            || fret.strings.end > self.strings
        {
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
            let pos = fretted.pos + self.starting_fret;
            for idx in fretted.strings.start..fretted.strings.end {
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

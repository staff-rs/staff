use std::{iter::Enumerate, slice};

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
    pub fn insert(&mut self, fretted: Fretted) -> Option<usize> {
        if fretted.fret >= self.frets || fretted.start > self.strings || fretted.end > self.strings
        {
            return None;
        }

        if let Some(idx) = self.intersections(&fretted).next() {
            Some(idx)
        } else {
            self.fretted.push(fretted);
            None
        }
    }

    pub fn intersections<'d, 'f>(&'d self, fretted: &'f Fretted) -> Intersections<'d, 'f> {
        Intersections {
            iter: self.fretted.iter().enumerate(),
            fretted,
        }
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

impl FromIterator<Fretted> for Diagram {
    fn from_iter<T: IntoIterator<Item = Fretted>>(iter: T) -> Self {
        let mut diagram = Self::default();

        for fretted in iter {
            diagram.insert(fretted);
        }

        diagram
    }
}

pub struct Intersections<'d, 'f> {
    iter: Enumerate<slice::Iter<'d, Fretted>>,
    fretted: &'f Fretted,
}

impl Iterator for Intersections<'_, '_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for (pos, f) in &mut self.iter {
            if f.is_intersection(self.fretted) {
                return Some(pos);
            }
        }

        None
    }
}

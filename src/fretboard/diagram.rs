use std::{iter::Enumerate, slice};

use super::Fretboard;
use crate::midi::MidiNote;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Range {
    pub fret: u8,
    pub start: u8,
    pub end: u8,
}

impl Range {
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

/// Fretted instrument chord diagram
pub struct Diagram {
    pub strings: u8,
    pub frets: u8,
    pub starting_fret: u8,
    pub fretted: Vec<Range>,
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
    /// Insert a [`Range`] into the diagram, combining any intersecting ranges.
    /// If the range is out of bounds for the given diagram, this will return `Some`.
    pub fn insert(&mut self, fretted: Range) -> Option<Range> {
        if fretted.fret >= self.frets || fretted.start > self.strings || fretted.end > self.strings
        {
            return Some(fretted);
        }

        let mut start = fretted.start;
        let mut end = fretted.end;
        let mut pos = 0;

        while pos < self.fretted.len() {
            let f = &self.fretted[pos];
            if f.is_intersection(&fretted) {
                start = start.min(f.start);
                end = end.max(f.end);

                self.fretted.remove(pos);
            } else {
                pos += 1;
            }
        }

        self.fretted.push(Range::new(fretted.fret, start, end));
        None
    }

    pub fn intersections<'d, 'f>(&'d self, fretted: &'f Range) -> Intersections<'d, 'f> {
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

impl FromIterator<Range> for Diagram {
    fn from_iter<T: IntoIterator<Item = Range>>(iter: T) -> Self {
        let mut diagram = Self::default();

        for fretted in iter {
            diagram.insert(fretted);
        }

        diagram
    }
}

pub struct Intersections<'d, 'f> {
    iter: Enumerate<slice::Iter<'d, Range>>,
    fretted: &'f Range,
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

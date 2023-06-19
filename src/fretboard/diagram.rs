use std::{iter::Enumerate, slice};
use super::Fretboard;
use crate::midi::MidiNote;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    pub fn is_intersection(self, other: &Self) -> bool {
        self.fret == other.fret && self.start < other.end && self.end > other.start
    }

    pub fn into_point(self) -> Option<u8> {
        if self.start + 1 == self.end {
            Some(self.start)
        } else {
            None
        }
    }
}

/// Fretted instrument chord diagram
pub struct Diagram {
    strings: u8,
    frets: u8,
    pub starting_fret: u8,
    ranges: Vec<Range>,
}

impl Default for Diagram {
    fn default() -> Self {
        Self::new(6, 6, 0)
    }
}

impl Diagram {
    pub fn new(strings: u8, frets: u8, starting_fret: u8) -> Self {
        Self {
            strings,
            frets,
            starting_fret,
            ranges: Vec::new(),
        }
    }

    pub fn strings(&self) -> u8 {
        self.strings
    }
    pub fn frets(&self) -> u8 {
        self.frets
    }

    pub fn ranges(&self) -> &[Range] {
        &self.ranges
    }

    pub fn set_strings(&mut self, strings: u8) {
        let mut pos = 0;
        while pos < self.ranges.len() {
            let range = &self.ranges()[pos];
            if range.start > strings || range.end > strings {
                self.ranges.remove(pos);
            } else {
                pos += 1;
            }
        }
        self.strings = strings;
    }

    pub fn set_frets(&mut self, frets: u8) {
        let mut pos = 0;
        while pos < self.ranges.len() {
            let range = &self.ranges()[pos];
            if range.fret > frets {
                self.ranges.remove(pos);
            } else {
                pos += 1;
            }
        }
        self.frets = frets;
    }

    /// Insert a [`Range`] into the diagram, combining any intersecting ranges.
    /// If the range is out of bounds for the given diagram, this will return `Some`.
    pub fn insert(&mut self, range: Range) -> Option<Range> {
        if range.fret >= self.frets || range.start > self.strings || range.end > self.strings {
            return Some(range);
        }

        let mut start = range.start;
        let mut end = range.end;
        let mut pos = 0;

        while pos < self.ranges.len() {
            let f = &self.ranges[pos];
            if f.is_intersection(&range) {
                start = start.min(f.start);
                end = end.max(f.end);

                self.ranges.remove(pos);
            } else {
                pos += 1;
            }
        }

        self.ranges.push(Range::new(range.fret, start, end));
        None
    }

    pub fn remove(&mut self, index: usize) {
        self.ranges.remove(index);
    }

    pub fn remove_range(&mut self, range: Range) {
        let mut pos = 0;

        while pos < self.ranges.len() {
            let f = &self.ranges[pos];
            if f.is_intersection(&range) {
                self.ranges.remove(pos);
            } else {
                pos += 1;
            }
        }
    }

    pub fn intersections<'d, 'f>(&'d self, fretted: &'f Range) -> Intersections<'d, 'f> {
        Intersections {
            iter: self.ranges.iter().enumerate(),
            fretted,
        }
    }

    pub fn midi_notes<I: IntoIterator<Item = MidiNote>>(
        &self,
        tuning: I,
    ) -> Fretboard<I, Vec<Option<u8>>> {
        let mut frets: Vec<Option<u8>> = vec![None; self.strings as _];
        for fretted in &self.ranges {
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

impl Extend<Range> for Diagram {
    fn extend<T: IntoIterator<Item = Range>>(&mut self, iter: T) {
        for fretted in iter {
            self.insert(fretted);
        }
    }
}

impl FromIterator<Range> for Diagram {
    fn from_iter<T: IntoIterator<Item = Range>>(iter: T) -> Self {
        let mut diagram = Self::default();
        diagram.extend(iter);
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

use crate::{Interval, Pitch};
use core::marker::PhantomData;
use core::ops::Index;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set<T> {
    bits: u16,
    _marker: PhantomData<T>,
}

impl Default for Set<Interval> {
    fn default() -> Self {
        Self::empty()
    }
}

impl Default for Set<Pitch> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Set<T> {
    fn new(bits: u16) -> Self {
        Self {
            bits,
            _marker: PhantomData,
        }
    }

    fn empty() -> Self {
        Self::new(0)
    }

    /// Removes the least signifigant bit from `self` and returns its position
    pub fn pop_bit(&mut self) -> Option<u8> {
        if self.bits != 0 {
            let trailing = self.bits.trailing_zeros();
            self.bits &= self.bits - 1;
            Some(trailing as u8)
        } else {
            None
        }
    }
}

impl<T> Set<T>
where
    T: Into<u8>,
{
    pub fn push(&mut self, item: T) {
        self.bits |= 1 << item.into() as u16;
    }

    pub fn remove(&mut self, item: T) {
        self.bits |= !(1 << item.into()) as u16;
    }

    pub fn contains(&self, item: T) -> bool {
        self.bits >> item.into() & 1 == 1
    }

    pub fn split(self, item: T) -> (Self, Self) {
        let byte = item.into();
        (
            Self::new(self.bits & ((1 << byte) - 1)),
            Self::new((self.bits >> byte) << byte),
        )
    }
}

impl Set<Interval> {
    pub fn modes(self) -> impl Iterator<Item = Self> {
        self.enumerate().map(move |(index, _)| {
            let rotated = self.bits.rotate_right(index as _);
            Self::new(rotated)
        })
    }
}

impl FromIterator<Interval> for Set<Interval> {
    fn from_iter<T: IntoIterator<Item = Interval>>(iter: T) -> Self {
        let mut pitch_set = Self::default();
        for pitch in iter {
            pitch_set.push(pitch);
        }
        pitch_set
    }
}

impl FromIterator<Pitch> for Set<Pitch> {
    fn from_iter<T: IntoIterator<Item = Pitch>>(iter: T) -> Self {
        let mut pitch_set = Self::default();
        for pitch in iter {
            pitch_set.push(pitch);
        }
        pitch_set
    }
}

impl Iterator for Set<Interval> {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_bit().map(Interval::new)
    }
}

impl Iterator for Set<Pitch> {
    type Item = Pitch;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_bit().map(Pitch::from_byte)
    }
}

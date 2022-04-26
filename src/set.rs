use num_traits::{PrimInt, Zero};

use crate::{Interval, Pitch};
use core::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set<T, U> {
    bits: U,
    _marker: PhantomData<T>,
}

impl<T, U: Zero> Default for Set<T, U> {
    fn default() -> Self {
        Self::new(U::zero())
    }
}

impl<T, U> Set<T, U> {
    fn new(bits: U) -> Self {
        Self {
            bits,
            _marker: PhantomData,
        }
    }

    /// Removes the least signifigant bit from `self` and returns its position
    pub fn pop_bit(&mut self) -> Option<u8>
    where
        U: PrimInt,
    {
        if !self.bits.is_zero() {
            let trailing = self.bits.trailing_zeros();
            self.bits = self.bits & (self.bits - U::one());
            Some(trailing as u8)
        } else {
            None
        }
    }
}

impl<T, U> Set<T, U>
where
    T: Into<u8>,
    U: PrimInt,
{
    pub fn push(&mut self, item: T) {
        self.bits = self.bits | (U::one() << item.into() as usize);
    }

    pub fn remove(&mut self, item: T) {
        self.bits = self.bits | !(U::one() << item.into() as usize);
    }

    pub fn contains(&self, item: T) -> bool {
        (self.bits >> item.into() as usize & U::one()).is_one()
    }

    pub fn split(self, item: T) -> (Self, Self) {
        let byte = item.into() as usize;
        (
            Self::new(self.bits & ((U::one() << byte) - U::one())),
            Self::new((self.bits >> byte) << byte),
        )
    }
}

pub type IntervalSet = Set<Interval, u32>;

impl IntervalSet {
    pub fn modes(self) -> impl Iterator<Item = Self> {
        self.enumerate().map(move |(index, _)| {
            let rotated = self.bits.rotate_right(index as _);
            Self::new(rotated)
        })
    }
}

impl FromIterator<Interval> for IntervalSet {
    fn from_iter<T: IntoIterator<Item = Interval>>(iter: T) -> Self {
        let mut pitch_set = Self::default();
        for pitch in iter {
            pitch_set.push(pitch);
        }
        pitch_set
    }
}

impl FromIterator<Pitch> for Set<Pitch, u16> {
    fn from_iter<T: IntoIterator<Item = Pitch>>(iter: T) -> Self {
        let mut pitch_set = Self::default();
        for pitch in iter {
            pitch_set.push(pitch);
        }
        pitch_set
    }
}

impl<T, U> Extend<T> for Set<T, U>
where
    T: Into<u8>,
    U: PrimInt,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T, U> Iterator for Set<T, U>
where
    T: From<u8>,
    U: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_bit().map(Into::into)
    }
}

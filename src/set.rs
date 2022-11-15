use crate::{Interval, Pitch};
use core::marker::PhantomData;
use num_traits::{PrimInt, Zero};

pub type PitchSet = Set<Pitch, u16>;

pub type IntervalSet = Set<Interval, u32>;

impl IntervalSet {
    pub fn modes(self) -> impl Iterator<Item = Self> {
        self.enumerate().map(move |(index, _)| {
            let rotated = self.bits.rotate_right(index as _);
            Self::new(rotated)
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Set<T, U> {
    pub bits: U,
    #[cfg_attr(feature = "serde", serde(skip))]
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
}

impl<T, U> Set<T, U>
where
    U: PrimInt,
{
    pub fn all() -> Self
    where
        U: PrimInt,
    {
        Self::new(U::max_value())
    }

    /// Removes the least signifigant bit from `self` and returns its position
    pub fn pop_bit(&mut self) -> Option<u8> {
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
        self.bits = self.bits & !(U::one() << item.into() as usize);
    }

    pub fn maybe_remove(&mut self, item: T) -> Option<T>
    where
        T: Clone,
    {
        if self.contains(item.clone()) {
            self.remove(item.clone());
            Some(item)
        } else {
            None
        }
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

impl<T, U> FromIterator<T> for Set<T, U>
where
    T: Into<u8>,
    U: PrimInt,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::default();
        set.extend(iter);
        set
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

#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn push_and_remove_from_set() {
        let initial = [0, 2];
        let mut set: Set<u8, u8> = Set::from_iter(initial.into_iter());

        set.push(1);
        assert!(set.contains(1));
        set.remove(1);
        assert!(!set.contains(1));

        assert!(set.contains(0));
        assert!(set.contains(2));
    }
}

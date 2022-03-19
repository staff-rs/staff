use crate::Interval;
use core::mem::{self};
use std::array::IntoIter;

mod degree;
pub use degree::Degree;

pub const MAJOR_SCALE: [Interval; 7] = [
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
];

pub const NATURAL_MINOR_SCALE: [Interval; 7] = [
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
];

pub struct Scale<T, I> {
    degree: T,
    intervals: I,
}

impl<T, I> Scale<T, I>
where
    T: Degree,
    I: Iterator<Item = Interval>,
{
    #[inline]
    pub fn new(root: T, intervals: I) -> Self {
        Self {
            degree: root,
            intervals,
        }
    }
}

impl<T: Degree> Scale<T, IntoIter<Interval, 7>> {
    #[inline]
    pub fn major(root: T) -> Self {
        Self::new(root, MAJOR_SCALE.into_iter())
    }

    #[inline]
    pub fn natural_minor(root: T) -> Self {
        Self::new(root, NATURAL_MINOR_SCALE.into_iter())
    }
}

impl<T, I> Iterator for Scale<T, I>
where
    T: Degree,
    I: Iterator<Item = Interval>,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(interval) = self.intervals.next() {
            let next = self.degree.next_degree(interval);
            Some(mem::replace(&mut self.degree, next))
        } else {
            None
        }
    }
}

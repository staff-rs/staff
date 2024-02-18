//! Scales with iterators

use crate::Interval;
use core::ops::Add;

mod degree;
pub use degree::Degree;

mod intervals;
pub use intervals::ScaleIntervals;

mod diatonic;
pub use diatonic::{Diatonic, DiatonicScale};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scale<T, U> {
    root: T,
    intervals: U,
}

impl<T, U> Scale<T, U> {
    pub fn new(root: T, intervals: U) -> Self {
        Self { root, intervals }
    }
}

impl<T, U> Scale<T, Diatonic<T, U>>
where
    T: Degree + Clone,
{
    pub fn diatonic(root: T, intervals: U) -> Self {
        Self::new(root.clone(), Diatonic::new(root, intervals))
    }
}

impl<T> Scale<T, ScaleIntervals> {
    pub fn blues(root: T) -> Self {
        Self::new(root, ScaleIntervals::blues())
    }
}

impl<T, U> Iterator for Scale<T, U>
where
    T: Add<Interval> + Clone,
    U: Iterator<Item = Interval>,
{
    type Item = T::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.intervals
            .next()
            .map(|interval| self.root.clone() + interval)
    }
}

impl<T, U> Iterator for Scale<T, Diatonic<T, U>>
where
    T: Degree + Clone,
    U: Iterator<Item = Interval>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.intervals.intervals.next().map(|interval| {
            self.root
                .clone()
                .degree(&mut self.intervals.state, interval)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Natural, Note, Scale};

    #[test]
    fn test_g_flat_major() {
        let scale = Scale::major(Note::flat(Natural::G));

        assert!(scale.eq([
            Note::flat(Natural::G),
            Note::flat(Natural::A),
            Note::flat(Natural::B),
            Note::flat(Natural::C),
            Note::flat(Natural::D),
            Note::flat(Natural::E),
            Note::from(Natural::F),
        ]))
    }

    #[test]
    fn test_f_sharp_major() {
        let scale = Scale::major(Note::sharp(Natural::F));

        assert!(scale.eq([
            Note::sharp(Natural::F),
            Note::sharp(Natural::G),
            Note::sharp(Natural::A),
            Note::from(Natural::B),
            Note::sharp(Natural::C),
            Note::sharp(Natural::D),
            Note::sharp(Natural::E),
        ]))
    }

    #[test]
    fn test_c_sharp_major() {
        let scale = Scale::major(Note::sharp(Natural::C));

        assert!(scale.eq([
            Note::sharp(Natural::C),
            Note::sharp(Natural::D),
            Note::sharp(Natural::E),
            Note::sharp(Natural::F),
            Note::sharp(Natural::G),
            Note::sharp(Natural::A),
            Note::sharp(Natural::B),
        ]))
    }

    #[test]
    fn test_c_locrian() {
        let scale = Scale::locrian(Note::from(Natural::C));

        assert!(scale.eq([
            Note::from(Natural::C),
            Note::flat(Natural::D),
            Note::flat(Natural::E),
            Note::from(Natural::F),
            Note::flat(Natural::G),
            Note::flat(Natural::A),
            Note::flat(Natural::B),
        ]));
    }

    #[test]
    fn test_c_lydian() {
        let scale = Scale::lydian(Note::from(Natural::C));

        assert!(scale.eq([
            Note::C,
            Note::D,
            Note::E,
            Note::sharp(Natural::F),
            Note::G,
            Note::A,
            Note::B,
        ]));
    }
}

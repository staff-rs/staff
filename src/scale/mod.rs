use crate::set::Set;
use crate::Interval;
use core::ops::Add;

mod degree;
pub use degree::Degree;

pub type DiatonicScale<T> = Scale<T, Diatonic<T, Set<Interval, u16>>>;

impl<T> DiatonicScale<T>
where
    T: Degree + Clone,
{
    pub fn major(root: T) -> Self {
        Self::from_array(
            root,
            [
                Interval::UNISON,
                Interval::MAJOR_SECOND,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SIXTH,
                Interval::MAJOR_SEVENTH,
            ],
        )
    }

    pub fn natural_minor(root: T) -> Self {
        Self::from_array(
            root,
            [
                Interval::UNISON,
                Interval::MAJOR_SECOND,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SIXTH,
                Interval::MINOR_SEVENTH,
            ],
        )
    }

    /// ```
    /// use music_note::{Natural, Note, Scale};
    ///
    /// let scale = Scale::harmonic_minor(Note::from(Natural::A));
    ///
    /// assert!(scale.eq([
    ///     Note::from(Natural::A),
    ///     Note::from(Natural::B),
    ///     Note::from(Natural::C),
    ///     Note::from(Natural::D),
    ///     Note::from(Natural::E),
    ///     Note::from(Natural::F),
    ///     Note::sharp(Natural::G),
    /// ]));
    /// ```
    pub fn harmonic_minor(root: T) -> Self {
        Self::from_array(
            root,
            [
                Interval::UNISON,
                Interval::MAJOR_SECOND,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SIXTH,
                Interval::MAJOR_SEVENTH,
            ],
        )
    }

    pub fn melodic_minor(root: T) -> Self {
        Self::from_array(
            root,
            [
                Interval::UNISON,
                Interval::MAJOR_SECOND,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SIXTH,
                Interval::MAJOR_SEVENTH,
            ],
        )
    }

    fn from_array<const N: usize>(root: T, array: [Interval; N]) -> Self {
        Self::diatonic(root, array.into_iter().collect())
    }
}

pub struct Diatonic<T: Degree, U> {
    degree: T,
    state: T::State,
    intervals: U,
}

impl<T, U> Diatonic<T, U>
where
    T: Degree + Clone,
{
    pub fn new(root: T, intervals: U) -> Self {
        Self {
            degree: root.clone(),
            state: root.state(),
            intervals,
        }
    }
}

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

impl<T> Scale<T, Set<Interval, u16>> {
    pub fn blues(root: T) -> Self {
        Self::new(
            root,
            [
                Interval::UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::TRITONE,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
            ]
            .into_iter()
            .collect(),
        )
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
            self.intervals
                .degree
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
}

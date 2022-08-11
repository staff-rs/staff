use crate::set::Set;
use crate::Interval;
use core::ops::Add;

mod degree;
pub use degree::Degree;

pub type ScaleIntervals = Set<Interval, u16>;

impl ScaleIntervals {
    pub fn major() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SIXTH,
            Interval::MAJOR_SEVENTH,
        ])
    }

    pub fn natural_minor() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SIXTH,
            Interval::MINOR_SEVENTH,
        ])
    }

    pub fn harmonic_minor() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SIXTH,
            Interval::MAJOR_SEVENTH,
        ])
    }

    pub fn melodic_minor() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SIXTH,
            Interval::MAJOR_SEVENTH,
        ])
    }

    pub fn blues() -> Self {
        Self::from_iter([
            Interval::UNISON,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::TRITONE,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SEVENTH,
        ])
    }
}

pub type DiatonicScale<T> = Scale<T, Diatonic<T, ScaleIntervals>>;

impl<T> DiatonicScale<T>
where
    T: Degree + Clone,
{
    /// ```
    /// use staff::{midi, Note, Scale};
    ///
    /// // C major
    /// let scale = Scale::major(midi!(C, 4));
    ///
    /// assert!(scale.eq([
    ///     midi!(C, 4),
    ///     midi!(D, 4),
    ///     midi!(E, 4),
    ///     midi!(F, 4),
    ///     midi!(G, 4),
    ///     midi!(A, 4),
    ///     midi!(B, 4),
    /// ]));
    /// ```
    pub fn major(root: T) -> Self {
        Self::diatonic(root, ScaleIntervals::major())
    }

    /// ```
    /// use staff::{Natural, Note, Scale};
    ///
    /// // B natural minor
    /// let scale = Scale::natural_minor(Note::from(Natural::B));
    ///
    /// assert!(scale.eq([
    ///     Note::from(Natural::B),
    ///     Note::sharp(Natural::C),
    ///     Note::from(Natural::D),
    ///     Note::from(Natural::E),
    ///     Note::sharp(Natural::F),
    ///     Note::from(Natural::G),
    ///     Note::from(Natural::A),
    /// ]));
    /// ```
    pub fn natural_minor(root: T) -> Self {
        Self::diatonic(root, ScaleIntervals::natural_minor())
    }

    /// ```
    /// use staff::{Natural, Note, Scale};
    ///
    /// // A harmonic minor
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
        Self::diatonic(root, ScaleIntervals::harmonic_minor())
    }

    /// ```
    /// use staff::{Natural, Note, Scale};
    ///
    /// // E melodic minor
    /// let scale = Scale::melodic_minor(Note::from(Natural::E));
    ///
    /// assert!(scale.eq([
    ///     Note::from(Natural::E),
    ///     Note::sharp(Natural::F),
    ///     Note::from(Natural::G),
    ///     Note::from(Natural::A),
    ///     Note::from(Natural::B),
    ///     Note::sharp(Natural::C),
    ///     Note::sharp(Natural::D),
    /// ]));
    /// ```
    pub fn melodic_minor(root: T) -> Self {
        Self::diatonic(root, ScaleIntervals::melodic_minor())
    }
}

pub struct Diatonic<T: Degree, U> {
    state: T::State,
    intervals: U,
}

impl<T, U> Diatonic<T, U>
where
    T: Degree,
{
    pub fn new(root: T, intervals: U) -> Self {
        Self {
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
}

use super::{Degree, Scale, ScaleIntervals};

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

    pub fn dorian(root: T) -> Self {
        Self::diatonic(root, ScaleIntervals::dorian())
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Diatonic<T: Degree, U> {
    pub state: T::State,
    pub intervals: U,
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

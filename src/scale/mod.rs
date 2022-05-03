use crate::Interval;
use core::array::IntoIter;
use core::mem::{self};

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

/// A diatonic scale
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
    /// ```
    /// use music_note::{Natural, Note, Scale};
    ///
    /// // F major scale
    /// let scale = Scale::major(Note::from(Natural::F));
    ///  
    /// assert!(scale.eq([
    ///     Note::from(Natural::F),
    ///     Note::from(Natural::G),
    ///     Note::from(Natural::A),
    ///     Note::flat(Natural::B),
    ///     Note::from(Natural::C),
    ///     Note::from(Natural::D),
    ///     Note::from(Natural::E),
    /// ]))
    /// ```
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

use crate::set::Set;
use crate::Interval;
use core::ops::Add;

mod degree;
pub use degree::Degree;

pub struct Diatonic<T: Degree, U> {
    degree: T,
    state: T::State,
    intervals: U,
}

pub struct Scale<T, U> {
    root: T,
    intervals: U,
}

impl<T> Scale<T, Diatonic<T, Set<Interval, u16>>>
where
    T: Degree + Clone,
{
    pub fn major(root: T) -> Self {
        Self {
            root: root.clone(),
            intervals: Diatonic {
                degree: root.clone(),
                state: root.state(),
                intervals: [
                    Interval::UNISON,
                    Interval::MAJOR_SECOND,
                    Interval::MAJOR_THIRD,
                    Interval::PERFECT_FOURTH,
                    Interval::PERFECT_FIFTH,
                    Interval::MAJOR_SIXTH,
                    Interval::MAJOR_SEVENTH,
                ]
                .into_iter()
                .collect(),
            },
        }
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

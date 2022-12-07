use crate::{midi::Octave, Natural};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note {
    pub index: i64,
}

impl Note {
    pub fn new(natural: Natural, octave: Octave) -> Self {
        let index = Natural::F as u8 as i64 - natural as u8 as i64
            + 7 * (Octave::FIVE.into_i8() as i64 - octave.into_i8() as i64);

        index.into()
    }
}

impl From<i64> for Note {
    fn from(index: i64) -> Self {
        Self { index }
    }
}

#[cfg(test)]
mod test {
    use crate::{midi::Octave, Natural};

    use super::Note;

    #[test]
    fn it_works() {
        let note = Note::new(Natural::F, Octave::FIVE);
        assert_eq!(note.index, 0)
    }

    #[test]
    fn it_works_e() {
        let note = Note::new(Natural::E, Octave::FOUR);
        assert_eq!(note.index, 8);
    }
}

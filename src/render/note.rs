use crate::{midi::Octave, note::Accidental, Natural};

pub fn note_index(natural: Natural, octave: Octave) -> i64 {
    Natural::F as u8 as i64 - natural as u8 as i64
        + 7 * (Octave::FIVE.into_i8() as i64 - octave.into_i8() as i64)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    pub index: i64,
    pub accidental: Accidental,
}

impl Note {
    pub fn new(natural: Natural, octave: Octave, accidental: Accidental) -> Self {
        Self {
            index: note_index(natural, octave),
            accidental,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{midi::Octave, note::Accidental, Natural};

    use super::Note;

    #[test]
    fn it_works() {
        let note = Note::new(Natural::F, Octave::FIVE, Accidental::Natural);
        assert_eq!(note.index, 0)
    }

    #[test]
    fn it_works_e() {
        let note = Note::new(Natural::E, Octave::FOUR, Accidental::Natural);
        assert_eq!(note.index, 8);
    }
}

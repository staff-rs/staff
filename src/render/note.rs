use crate::{midi::Octave, note::Accidental, Natural};

pub fn note_index(natural: Natural, octave: Octave) -> i64 {
    let mut octave_index = (Octave::FIVE.into_i8() as i64 - octave.into_i8() as i64);
    if natural < Natural::C {
        octave_index -= 1;
    }

    Natural::F as u8 as i64 - natural as u8 as i64 + 7 * octave_index
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    pub index: i64,
    pub accidental: Option<Accidental>,
}

impl Note {
    pub fn new(natural: Natural, octave: Octave, accidental: Option<Accidental>) -> Self {
        Self {
            index: note_index(natural, octave),
            accidental,
        }
    }
}

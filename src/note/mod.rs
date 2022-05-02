use crate::{Natural, Pitch};
use core::marker::PhantomData;

mod accidental;
pub use accidental::{Accidental, AccidentalKind, Flat, Sharp};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note<A> {
    pub natural: Natural,

    pub accidental_kind: AccidentalKind,
    _accidental: PhantomData<A>,
}

impl<A> Note<A> {
    pub fn new(natural: Natural, accidental: AccidentalKind) -> Self {
        Self {
            natural,
            accidental_kind: accidental,
            _accidental: PhantomData,
        }
    }
}

impl Note<Flat> {
    pub fn flat(natural: Natural) -> Self {
        Self::new(natural, AccidentalKind::Single)
    }

    pub fn double_flat(natural: Natural) -> Self {
        Self::new(natural, AccidentalKind::Double)
    }
}

impl Note<Sharp> {
    pub fn sharp(natural: Natural) -> Self {
        Self::new(natural, AccidentalKind::Single)
    }

    pub fn double_sharp(natural: Natural) -> Self {
        Self::new(natural, AccidentalKind::Double)
    }
}

impl<A> From<Note<A>> for Pitch
where
    A: Accidental,
{
    fn from(note: Note<A>) -> Self {
        A::accidental(note.accidental_kind, note.natural)
    }
}

impl<A> From<Natural> for Note<A> {
    fn from(natural: Natural) -> Self {
        Self::new(natural, AccidentalKind::Natural)
    }
}

use crate::{Natural, Pitch};
use core::{fmt, marker::PhantomData};

mod accidental;
pub use accidental::{Accidental, AccidentalKind, Flat, Sharp};

#[derive(Debug, PartialEq, Eq)]
pub struct Note<A> {
    natural: Natural,
    accidental_kind: AccidentalKind,
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

    pub fn natural(self) -> Natural {
        self.natural
    }

    pub fn natural_mut(&mut self) -> &mut Natural {
        &mut self.natural
    }

    pub fn accidental(self) -> AccidentalKind {
        self.accidental_kind
    }

    pub fn accidental_mut(&mut self) -> &mut AccidentalKind {
        &mut self.accidental_kind
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

impl<A> Clone for Note<A> {
    fn clone(&self) -> Self {
        Self::new(self.natural, self.accidental_kind)
    }
}

impl<A> Copy for Note<A> {}

impl<A> From<Note<A>> for Pitch
where
    A: Accidental,
{
    fn from(note: Note<A>) -> Self {
        A::into_pitch(note.accidental_kind, note.natural)
    }
}

impl<A> From<Natural> for Note<A> {
    fn from(natural: Natural) -> Self {
        Self::new(natural, AccidentalKind::Natural)
    }
}

impl<A> fmt::Display for Note<A>
where
    A: Accidental,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.natural.fmt(f)?;
        A::write_fmt(self.accidental_kind, f)
    }
}

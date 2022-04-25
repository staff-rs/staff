use super::Chord;

#[derive(Clone, Copy, Debug)]
pub enum ChordAccidental {
    Natural,
    Sharp,
    Flat,
}

#[derive(Clone, Copy, Debug)]
pub enum Seventh {
    Major,
    Minor,
}

#[derive(Clone, Copy, Debug)]
pub enum Third {
    Major,
    Minor,
    Sus2,
    Sus4,
}

#[derive(Clone, Copy, Debug)]
pub struct Builder<T> {
    pub(super) bass: Option<T>,
    pub(super) no_root: bool,
    pub(super) third: Third,
    pub(super) fifth: Option<ChordAccidental>,
    pub(super)sixth: bool,
    pub(super)seventh: Option<ChordAccidental>,
    pub(super) ninth: Option<ChordAccidental>,
    pub(super) eleventh: Option<ChordAccidental>,
    pub(super) thirteenth: Option<ChordAccidental>,
}

impl<T> Default for Builder<T> {
    fn default() -> Self {
        Self {
            bass: None,
            no_root: true,
            third: Third::Major,
            fifth: None,
            sixth: false,
            seventh: None,
            ninth: None,
            eleventh: None,
            thirteenth: None,
        }
    }
}

impl<T> Builder<T> {
    pub fn bass(mut self, bass: T) -> Self {
        self.bass = Some(bass);
        self
    }

    pub fn no_root(mut self) -> Self {
        self.no_root = true;
        self
    }

    pub fn third(mut self, third: Third) -> Self {
        self.third = third;
        self
    }

    pub fn major(self) -> Self {
        self.third(Third::Major)
    }

    pub fn minor(self) -> Self {
        self.third(Third::Minor)
    }

    pub fn sus2(self) -> Self {
        self.third(Third::Sus2)
    }

    pub fn sus4(self) -> Self {
        self.third(Third::Sus4)
    }

    pub fn build(self, root: T) -> Chord<T> {
        Chord {
            root,
            builder: self,
        }
    }
}
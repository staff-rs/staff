mod note;
pub use note::Note;

#[derive(Clone, PartialEq, Eq)]
pub enum Element {
    Br,
    Hr,
    Clef(Clef),
    Note(Note),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Clef {}

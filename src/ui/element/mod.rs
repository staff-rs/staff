mod note;
pub use note::Note;

pub enum Element {
    Br(Br),
    Clef(Clef),
    Note(Note),
}

pub struct Br {}

pub struct Clef {}

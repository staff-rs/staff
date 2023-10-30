mod note;
pub use note::Note;

pub enum Element {
    Br,
    Hr,
    Clef(Clef),
    Note(Note),
}


pub struct Clef {}

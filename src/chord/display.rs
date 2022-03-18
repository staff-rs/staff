use super::ChordKind;
use crate::midi::MidiNoteDisplay;
use core::fmt;

pub struct ChordDisplay {
    root: MidiNoteDisplay,
    kind: ChordKind,
}

impl ChordDisplay {
    pub fn new(root: MidiNoteDisplay, kind: ChordKind) -> Self {
        Self { root, kind }
    }
}

impl fmt::Display for ChordDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.root, self.kind.to_str())
    }
}

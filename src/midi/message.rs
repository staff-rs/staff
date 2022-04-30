use super::MidiNote;

pub struct NoteMessage {
    pub note: MidiNote,
    pub velocity: u8,
}

pub enum Message {
    NoteOff(NoteMessage),
    NoteOn(NoteMessage),
}

impl FromIterator<u8> for Message {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let status = iter.next().unwrap();
        match status >> 4 {
            0b1000 => {
                let note = MidiNote::from_byte(iter.next().unwrap());
                let velocity = iter.next().unwrap();
                Message::NoteOff(NoteMessage { note, velocity })
            }
            0b1001 => {
                let note = MidiNote::from_byte(iter.next().unwrap());
                let velocity = iter.next().unwrap();
                Message::NoteOn(NoteMessage { note, velocity })
            }
            _ => todo!(),
        }
    }
}

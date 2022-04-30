use super::MidiNote;

pub struct NoteMessage {
    pub note: MidiNote,
    pub velocity: u8,
}

pub enum Message {
    NoteOff(NoteMessage),
    NoteOn(NoteMessage),
}

pub struct Messages<T> {
    iter: T,
}

impl<T> Iterator for Messages<T>
where
    T: Iterator<Item = u8>,
{
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        let status = self.iter.next().unwrap();
        let msg = match status >> 4 {
            0b1000 => {
                let note = MidiNote::from_byte(self.iter.next().unwrap());
                let velocity = self.iter.next().unwrap();
                Message::NoteOff(NoteMessage { note, velocity })
            }
            0b1001 => {
                let note = MidiNote::from_byte(self.iter.next().unwrap());
                let velocity = self.iter.next().unwrap();
                Message::NoteOn(NoteMessage { note, velocity })
            }
            _ => todo!(),
        };
        Some(msg)
    }
}

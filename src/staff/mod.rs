use crate::{note::Flat, Interval, Natural, Note, Pitch};

mod key;
pub use key::Key;

pub struct Staff<T> {
    key: Key,
    notes: T,
}

impl<T> Staff<T> {
    pub fn new(key: Key, notes: T) -> Self {
        Self { key, notes }
    }
}

impl<T> Iterator for Staff<T>
where
    T: Iterator<Item = Note<Flat>>,
{
    type Item = Pitch;

    fn next(&mut self) -> Option<Self::Item> {
        self.notes.next().map(move |note| {
            let pitch = Pitch::from(note);
            if note.natural() >= Natural::from(self.key.flats()) {
                pitch + Interval::MINOR_SECOND
            } else {
                pitch
            }
        })
    }
}

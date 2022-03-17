use crate::{midi::MidiNote, Interval};

pub fn major(root: MidiNote) -> [MidiNote; 3] {
    [
        root,
        root + Interval::MAJOR_THIRD,
        root + Interval::PERFECT_FIFTH,
    ]
}

pub fn functions<I>(chord: I, root: MidiNote) -> impl Iterator
where
    I: IntoIterator<Item = MidiNote>,
{
    chord.into_iter().map(move |note| note - root)
}

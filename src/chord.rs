use crate::Pitch;

pub fn functions<I>(chord: I, root: Pitch) -> impl Iterator
where
    I: IntoIterator<Item = Pitch>,
{
    chord.into_iter().map(move |pitch: Pitch| pitch - root)
}

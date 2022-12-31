#[derive(Clone, Debug)]
pub struct Index {
    pub pos: usize,
    pub len: usize,
}

/// A sound [`Source`](rodio::Source) to play a guitar note.
pub struct GuitarSource<T> {
    pub frequencies: T,
    pub index: Index,
}

impl<T> GuitarSource<T>
where
    T: AsMut<[f32]>,
{
    pub fn new(frequencies: T, index: Index) -> Self {
        Self { frequencies, index }
    }
}

impl<T> Iterator for GuitarSource<T>
where
    T: AsMut<[f32]>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let frequencies = self.frequencies.as_mut();
        let output = (frequencies[self.index.pos]
            + frequencies[(self.index.pos + 1) % frequencies.len()])
            / 2.;
        frequencies[self.index.pos] = output;

        self.index.pos += 1;
        if self.index.pos >= frequencies.len() {
            self.index.pos = 0;
        }

        Some(output)
    }
}

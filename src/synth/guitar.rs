/// A sound [`Source`](rodio::Source) to play a guitar note.
pub struct GuitarSource<T> {
    pub frequencies: T,
    pub pos: usize,
}

impl<T> GuitarSource<T>
where
    T: AsMut<[f32]>,
{
    pub fn new(frequencies: T, pos: usize) -> Self {
        Self { frequencies, pos }
    }
}

impl<T> Iterator for GuitarSource<T>
where
    T: AsMut<[f32]>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let frequencies = self.frequencies.as_mut();
        let output = (frequencies[self.pos] + frequencies[(self.pos + 1) % frequencies.len()]) / 2.;
        frequencies[self.pos] = output;

        self.pos += 1;
        if self.pos >= frequencies.len() {
            self.pos = 0;
        }

        Some(output)
    }
}

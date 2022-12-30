use rand::{thread_rng, Rng};
use rodio::Source;
use std::time::Duration;

struct String {
    pos: usize,
    len: usize,
}

pub struct GuitarChord {
    frequencies: Vec<f32>,
    strings: Vec<String>,
    sample_rate: u32,
    num_sample: usize,
    num_spacing_samples: usize,
}

impl GuitarChord {
    pub fn new(sample_rate: u32, spacing_duration: Duration) -> Self {
        let spacing_nanos = spacing_duration.as_secs() as u32 * 1_000_000_000
            + spacing_duration.subsec_nanos() as u32;
        let num_spacing_samples = spacing_nanos / 1_000_000 * (sample_rate / 1_000);

        Self {
            frequencies: Vec::new(),
            strings: Vec::new(),
            sample_rate,
            num_sample: 0,
            num_spacing_samples: num_spacing_samples as _,
        }
    }

    pub fn set_frequencies(&mut self, freqs: impl IntoIterator<Item = f32>) {
        self.strings.clear();

        for freq in freqs {
            let period = (self.sample_rate as f32 / freq).round() as usize;
            self.strings.push(String {
                pos: 0,
                len: period,
            });

            let mut rng = thread_rng();
            for _ in 0..period {
                let noise = rng.gen_range(-1.0..1.0);
                self.frequencies.push(noise);
            }
        }
    }
}

impl Iterator for GuitarChord {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self
            .num_sample
            .checked_div(self.num_spacing_samples)
            .unwrap_or_default();
        self.num_sample += 1;

        let mut start = 0;
        let sum = self
            .strings
            .iter_mut()
            .take(count)
            .map(|string| {
                let frequencies = &mut self.frequencies[start..start + string.len];
                let output = (frequencies[string.pos]
                    + frequencies[(string.pos + 1) % frequencies.len()])
                    / 2.;
                frequencies[string.pos] = output;

                start += string.len;
                string.pos += 1;
                if string.pos >= frequencies.len() {
                    string.pos = 0;
                }

                output
            })
            .sum();
        Some(sum)
    }
}

impl Source for GuitarChord {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        // TODO
        None
    }
}

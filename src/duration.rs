#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DurationKind {
    Quarter,
    Half,
    Whole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Duration {
    kind: DurationKind,
    is_dotted: bool,
}

impl Duration {
    pub fn beats(self, unit: u8) -> f64 {
        let mut n = match self.kind {
            DurationKind::Quarter => 4.,
            DurationKind::Half => 2.,
            DurationKind::Whole => 1.,
        };
        if self.is_dotted {
            n *= 1.5;
        }

        unit as f64 / n
    }
}

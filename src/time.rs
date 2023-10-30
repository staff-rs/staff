#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DurationKind {
    Eigth,
    Quarter,
    Half,
    Whole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Duration {
    pub kind: DurationKind,
    pub is_dotted: bool,
}

impl Duration {
    pub fn new(kind: DurationKind, is_dotted: bool) -> Self {
        Self { kind, is_dotted }
    }

    pub fn beats(self, unit: u8) -> f64 {
        let mut n = match self.kind {
            DurationKind::Eigth => 8.,
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

impl From<DurationKind> for Duration {
    fn from(value: DurationKind) -> Self {
        Self::new(value, false)
    }
}

#[cfg(feature = "ui")]
impl<'a> dioxus::prelude::IntoAttributeValue<'a> for Duration {
    fn into_value(
        self,
        _bump: &'a dioxus::core::exports::bumpalo::Bump,
    ) -> dioxus::core::AttributeValue<'a> {
        dioxus::core::AttributeValue::Int(((self.kind as u8) << 1 | self.is_dotted as u8) as _)
    }
}

pub struct TimeSignature {
    pub unit: DurationKind,
    pub beats: u8,
}

impl TimeSignature {
    pub fn new(unit: DurationKind, beats: u8) -> Self {
        Self { unit, beats }
    }
}

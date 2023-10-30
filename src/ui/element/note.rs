use crate::{
    midi::Octave,
    note::Accidental,
    time::{Duration, DurationKind},
    Natural,
};
use dioxus::{core::AttributeValue, prelude::*};
use std::mem;

#[derive(Clone, PartialEq, Eq)]
pub struct Note {
    pub natural: Natural,
    pub octave: Octave,
    pub accidental: Option<Accidental>,
    pub duration: Duration,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            natural: Natural::C,
            octave: Octave::FOUR,
            accidental: None,
            duration: Duration::from(DurationKind::Quarter),
        }
    }
}

impl Note {
    pub fn with_natural(mut self, natural: Natural) -> Self {
        self.natural = natural;
        self
    }

    pub fn with_octave(mut self, octave: Octave) -> Self {
        self.octave = octave;
        self
    }

    pub fn with_accidental(mut self, accidental: Option<Accidental>) -> Self {
        self.accidental = accidental;
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn from_attrs(node: &VNode, attrs: &[TemplateAttribute]) -> Note {
        let mut natural = None;
        let mut octave = None;
        let mut accidental = None;
        let mut duration = None;

        for attr in attrs {
            match attr {
                TemplateAttribute::Static {
                    name: _,
                    value: _,
                    namespace: _,
                } => todo!(),
                TemplateAttribute::Dynamic { id } => {
                    let attr = &node.dynamic_attrs[*id];
                    match attr.name {
                        "accidental" => {
                            if let AttributeValue::Int(n) = attr.value {
                                if n < 0 || n > Accidental::DoubleSharp as u8 as _ {
                                    todo!()
                                }
                                let acc: Accidental = unsafe { mem::transmute(n as u8) };
                                accidental = Some(acc);
                            }
                        }
                        "natural" => {
                            if let AttributeValue::Int(n) = attr.value {
                                if n < 0 || n > Natural::G as u8 as _ {
                                    todo!()
                                }
                                let nat: Natural = unsafe { mem::transmute(n as u8) };
                                natural = Some(nat);
                            }
                        }
                        "octave" => {
                            if let AttributeValue::Int(n) = attr.value {
                                octave = Some(Octave::new_unchecked(n as _));
                            }
                        }
                        "duration" => {
                            if let AttributeValue::Int(n) = attr.value {
                                let kind = unsafe { mem::transmute((n >> 1) as u8) };
                                let is_dotted = n & 1 == 1;
                                duration = Some(Duration::new(kind, is_dotted));
                            }
                        }
                        _ => todo!(),
                    }
                }
            }
        }

        Self {
            natural: natural.unwrap(),
            octave: octave.unwrap_or(Octave::FOUR),
            accidental,
            duration: duration.unwrap_or_else(|| Duration::new(DurationKind::Quarter, false)),
        }
    }

    pub fn index(&self) -> i64 {
        let mut octave_index = Octave::FIVE.into_i8() as i64 - self.octave.into_i8() as i64;
        if self.natural < Natural::C {
            octave_index -= 1;
        }

        Natural::F as u8 as i64 - self.natural as u8 as i64 + 7 * octave_index
    }
}

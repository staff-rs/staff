use crate::{midi::Octave, note::Accidental, Natural};
use core::mem;
use dioxus::{
    core::AttributeValue,
    prelude::{TemplateAttribute, VNode},
};

pub struct Note {
    pub natural: Natural,
    pub octave: Octave,
    pub accidental: Option<Accidental>,
}

impl Note {
    pub fn from_attrs(node: &VNode, attrs: &[TemplateAttribute]) -> Note {
        let mut natural = None;
        let mut octave = None;
        let mut accidental = None;

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
                        _ => todo!(),
                    }
                }
            }
        }

        Self {
            natural: natural.unwrap(),
            octave: octave.unwrap_or(Octave::FOUR),
            accidental,
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

use super::prelude::*;
use crate::{midi::Octave, Natural};
use core::mem;
use dioxus::core::AttributeValue;

pub struct Note {
    natural: Natural,
    octave: Octave,
}

impl Note {
    pub fn from_attrs(node: &VNode, attrs: &[TemplateAttribute]) -> Note {
        let mut natural = None;
        let mut octave = None;

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

#[component]
pub fn Staff<'a>(
    cx: Scope<'a>,
    children: Element<'a>,

    /// Line height of the staff.
    #[props(default = 15.)]
    line_height: f64,

    /// Width of the staff.
    #[props(default = 400.)]
    width: f64,

    /// Stroke width of the items in the staff.
    #[props(default = 2.)]
    stroke_width: f64,
) -> Element<'a> {
    let node = children.as_ref().unwrap();

    let mut left = 10.;
    let top = *stroke_width;

    let elements = node.template.get().roots.iter().map(|root| match root {
        TemplateNode::Element {
            tag,
            namespace: _,
            attrs,
            children: _,
        } => match *tag {
            "note" => {
                let note = Note::from_attrs(node, attrs);
                let y = note.index() as f64 * (line_height / 2.) + top;
                let x = left;
                left += 30.;

                let stem_x = x + line_height / 2. - stroke_width / 2.;
                render!(
                    circle { cx: x, cy: y, r: line_height / 2. }
                    path {
                        d: "M{stem_x} {y - line_height * 3.} L{stem_x} {y}",
                        stroke: "#000",
                        stroke_width: *stroke_width
                    }
                )
            }
            "clef" => {
                let x = left;
                left += 70.;

                render!(text { x: x, y: line_height * 4. + stroke_width, font_family: "Noto Music", font_size: "{line_height * 3.}px", "𝄞" })
            }
            _ => todo!(),
        },
        _ => todo!(),
    });

    let mut d = String::new();
    for i in 0..5 {
        let y = i as f64 * line_height + top;
        d.push_str(&format!("M0 {y} L {width} {y} "));
    }

    render!(
        svg { width: "500px", height: "500px", xmlns: "http://www.w3.org/2000/svg",
            elements,
            path { d: "{d}", stroke: "#000", stroke_width: *stroke_width }
        }
    )
}

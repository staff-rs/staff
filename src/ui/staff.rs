use super::prelude::*;
use crate::{midi::Octave, Natural};
use core::mem;
use dioxus::core::AttributeValue;

#[component]
pub fn Staff<'a>(
    cx: Scope<'a>,
    children: Element<'a>,

    /// Line height of the staff.
    #[props(default = 20.)]
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
            tag: _,
            namespace: _,
            attrs,
            children: _,
        } => {
            let mut natural = None;
            for attr in *attrs {
                match attr {
                    TemplateAttribute::Static {
                        name: _,
                        value: _,
                        namespace: _,
                    } => todo!(),
                    TemplateAttribute::Dynamic { id } => {
                        let attr = &node.dynamic_attrs[*id];
                        if attr.name == "natural" {
                            if let AttributeValue::Int(n) = attr.value {
                                if n < 0 || n > Natural::G as u8 as _ {
                                    todo!()
                                }
                                let nat: Natural = unsafe { mem::transmute(n as u8) };
                                natural = Some(nat);
                            }
                        }
                    }
                }
            }

            let natural = natural.unwrap();
            let y = note_index(natural, Octave::FOUR) as f64 * (line_height / 2.) + top;
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

pub fn note_index(natural: Natural, octave: Octave) -> i64 {
    let mut octave_index = Octave::FIVE.into_i8() as i64 - octave.into_i8() as i64;
    if natural < Natural::C {
        octave_index -= 1;
    }

    Natural::F as u8 as i64 - natural as u8 as i64 + 7 * octave_index
}

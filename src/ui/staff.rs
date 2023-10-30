use super::prelude::*;
use crate::{
    time::DurationKind,
    ui::{
        element::{self, Br, Clef},
        items, Note,
    },
};

#[component]
fn Br(cx: Scope, x: f64, y: f64, top: f64, line_height: f64, stroke_width: f64) -> Element {
    render!(path {
        d: "M{x} {top + y}L{x} {top + y + line_height * 4.}",
        stroke: "#000",
        stroke_width: *stroke_width
    })
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
    let top = *stroke_width + 100.;

    let elements = items(node, *width).map(|(item, is_newline)| {
        let mut x = item.x;
        let lines = if is_newline {
            let mut d = String::new();
            for i in 0..5 {
                let y = i as f64 * line_height + top + item.y;
                d.push_str(&format!("M0 {y} L {width} {y} "));
            }
            render!(
                path { d: "{d}", stroke: "#000", stroke_width: *stroke_width }
                Br {
                    x: x + stroke_width / 2.,
                    y: item.y,
                    top: top,
                    line_height: *line_height,
                    stroke_width: *stroke_width
                }
                Br {
                    x: width - stroke_width / 2.,
                    y: item.y,
                    top: top,
                    line_height: *line_height,
                    stroke_width: *stroke_width
                }
            )
        } else {
            None
        };
        x += 20.;

        let elem =match item.element {
            element::Element::Note(note) => {
                let y = item.y + note.index() as f64 * (line_height / 2.) + top;
                let acc =  if let Some(accidental) = note.accidental {
                    let acc_x = x;
                    x += 30.;
                    render!(text { font_family: "Noto Music", x: acc_x, y: y + line_height / 2., font_size: "48px", "{accidental}" })
                } else {
                    None
                };

                let stem_x = x + line_height / 2. - stroke_width / 2.;
                let head_and_stem = match note.duration.kind {
                    DurationKind::Quarter => {
                        render! {
                            circle { cx: x, cy: y, r: line_height / 2., fill: "#000" }
                            path {
                                d: "M{stem_x} {y - line_height * 3.} L{stem_x} {y}",
                                stroke: "#000",
                                stroke_width: *stroke_width
                            }
                        }
                    }
                    DurationKind::Half => {
                        render! {
                            circle { cx: x, cy: y, r: line_height / 2. - stroke_width / 2., stroke: "#000", stroke_width: *stroke_width, fill: "none" }
                            path {
                                d: "M{stem_x} {y - line_height * 3.} L{stem_x} {y}",
                                stroke: "#000",
                                stroke_width: *stroke_width
                            }
                        }
                    }
                    DurationKind::Whole => {
                        render! (circle { cx: x, cy: y, r: line_height / 2. - stroke_width / 2., stroke: "#000", stroke_width: *stroke_width, fill: "none" })
                    }        
                };

                render!( acc, head_and_stem )
            }
            element::Element::Br(_) => render!(
                Br {
                    x: x,
                    y: item.y,
                    top: top,
                    line_height: *line_height,
                    stroke_width: *stroke_width
                }
            ),
             element::Element::Clef(_) => {
                render!(text { x: x, y: top + line_height * 4. + stroke_width, font_family: "Noto Music", font_size: "{line_height * 3.}px", "𝄞" })
            }
    
        };
        render!( lines, elem )
    });

    render!(svg {
        width: "{width}px",
        height: "500px",
        xmlns: "http://www.w3.org/2000/svg",
        elements
    })
}

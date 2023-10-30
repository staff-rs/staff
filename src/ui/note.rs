use dioxus::prelude::*;
use crate::{
    time::{Duration, DurationKind},
    ui::layout::{Layout, NoteLayout},
};


#[component]
fn Hr(cx: Scope, x: f64, y: f64, top: f64, line_height: f64, stroke_width: f64) -> Element {
    render!(path {
        d: "M{x} {top + y}L{x} {top + y + line_height * 4.}",
        stroke: "#000",
        stroke_width: *stroke_width
    })
}

#[component]
pub fn Note<'a>(
    cx: Scope<'a>,
    duration: Duration,
    x: f64,
    y: f64,
    layout: Layout,
    head_size: f64,
    font_size: f64,
    stroke_width: f64,
    line_height: f64,
    onlayout: EventHandler<'a, Layout>,
) -> Element<'a> {
    let mut x = *x;

    let accidental_elem = if let Some((accidental, size)) = layout.accidental {
        let text_x = x;
        x += size[0];

        render!(text { x: text_x, y: *y, font_family: "Noto Music", font_size: *font_size, "{accidental}" })
    } else {
        None
    };

    x += line_height / 2.;

    let stem_x = x + head_size - stroke_width / 2.;
    let head_and_stem = match duration.kind {
        DurationKind::Quarter => {
            render! {
                circle { cx: x, cy: *y, r: line_height / 2., fill: "#000" }
                path {
                    d: "M{stem_x} {y - line_height * 3.} L{stem_x} {y}",
                    stroke: "#000",
                    stroke_width: *stroke_width
                }
            }
        }
        DurationKind::Half => {
            render! {
                circle { cx: x, cy: *y, r: line_height / 2. - stroke_width / 2., stroke: "#000", stroke_width: *stroke_width, fill: "none" }
                path {
                    d: "M{stem_x} {y - line_height * 3.} L{stem_x} {y}",
                    stroke: "#000",
                    stroke_width: *stroke_width
                }
            }
        }
        DurationKind::Whole => {
            render!(circle {
                cx: x,
                cy: *y,
                r: line_height / 2. - stroke_width / 2.,
                stroke: "#000",
                stroke_width: *stroke_width,
                fill: "none"
            })
        }
    };

    render! {
        NoteLayout {
            font_size: *font_size,
            accidental: layout.accidental.map(|(acc, _)| acc),
            onlayout: |layout: Layout| onlayout.call(layout)
        }
        accidental_elem,
        head_and_stem,
    }
}

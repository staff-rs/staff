use crate::{
    time::{Duration, DurationKind},
    ui::layout::{Layout, NoteLayout},
};
use dioxus::prelude::*;
use std::rc::Rc;

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
    last: Rc<RefCell<Option<[f64; 2]>>>,
    onlayout: EventHandler<'a, Layout>,
    onclick: EventHandler<'a, MouseEvent>,
) -> Element<'a> {
    let mut x = *x;
    let accidental_elem = if let Some((accidental, size)) = layout.accidental {
        let text_x = x;
        x += size[0];

        render!(text { x: text_x, y: *y + line_height / 2., font_family: "Noto Music", font_size: *font_size, "{accidental}" })
    } else {
        None
    };

    let note_x = x + line_height / 2.;
    let render_stem = || {
        let stem_x = note_x + head_size - stroke_width / 2.;
        render!(path {
            d: "M{stem_x} {y - line_height * 3.} L{stem_x} {y}",
            stroke: "#000",
            stroke_width: *stroke_width
        })
    };
    let head_and_stem_elem = match duration.kind {
        DurationKind::Eigth => {
            let mut last_ref = last.borrow_mut();
            let half_stroke_width = stroke_width / 2.;
            let stem_x = note_x + head_size - stroke_width / 2.;
            let tie_height = 8.;

            let tie = if let Some(last) = last_ref.take() {
                let x1 = last[0] - half_stroke_width;
                let x2 = stem_x + half_stroke_width;

                render!(path {
                    d: r"
                        M{x1} {last[1]} L{stem_x} {y - line_height * 3.}
                        L{x2} {y - line_height * 3.}
                        L{x2} {y - line_height * 3. - tie_height}
                        L{x1} {last[1] - tie_height}
                        Z",
                    fill: "#000"
                })
            } else {
                let stem_x = note_x + head_size - stroke_width / 2.;

                *last_ref = Some([stem_x, y - line_height * 3.]);
                None
            };

            render! {
                circle {
                    cx: note_x,
                    cy: *y,
                    r: line_height / 2.,
                    fill: "#000",
                    cursor: "pointer",
                    onclick: |event| onclick.call(event)
                }
                render_stem(),
                tie
            }
        }
        DurationKind::Quarter => {
            last.borrow_mut().take();
            render! {
                circle {
                    cx: note_x,
                    cy: *y,
                    r: line_height / 2.,
                    fill: "#000",
                    cursor: "pointer",
                    onclick: |event| onclick.call(event)
                }
                render_stem()
            }
        }
        DurationKind::Half => {
            last.borrow_mut().take();
            render! {
                circle {
                    cx: note_x,
                    cy: *y,
                    r: line_height / 2. - stroke_width / 2.,
                    stroke: "#000",
                    stroke_width: *stroke_width,
                    fill: "rgba(0,0,0,0)",
                    cursor: "pointer",
                    onclick: |event| onclick.call(event)
                }
                render_stem()
            }
        }
        DurationKind::Whole => {
            last.borrow_mut().take();
            render!(circle {
                cx: note_x,
                cy: *y,
                r: line_height / 2. - stroke_width / 2.,
                stroke: "#000",
                stroke_width: *stroke_width,
                fill: "rgba(0,0,0,0)",
                cursor: "pointer",
                onclick: |event| onclick.call(event)
            })
        }
    };

    render! {
        NoteLayout {
            font_size: *font_size,
            accidental: layout.accidental.map(|(acc, _)| acc),
            duration: *duration,
            onlayout: |layout: Layout| onlayout.call(layout)
        }
        accidental_elem,
        head_and_stem_elem
    }
}

use super::prelude::*;
use crate::{
    note::Accidental,
    ui::layout::{Layout, NoteLayout},
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
pub fn Note<'a>(
    cx: Scope<'a>,
    x: f64,
    y: f64,
    layout: Layout,
    font_size: f64,
    onlayout: EventHandler<'a, Layout>,
) -> Element<'a> {
    let mut x = *x;

    let accidental_elem = if let Some((accidental, size)) = layout.accidental {
        let text_x = x;
        x += size[0];

        render!( text { x: text_x, y: *y,font_family: "Noto Music", font_size: *font_size,  "{accidental}" } )
    } else {
        None
    };

    render! {
        NoteLayout { font_size: *font_size, accidental: Accidental::Sharp, onlayout: |layout: Layout| onlayout.call(layout) }
        accidental_elem,
        circle { cx: x + 10., cy: *y, r: 5., fill: "#000" }
    }
}

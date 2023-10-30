use super::prelude::*;
use crate::ui::{
    element::{self, Br, Clef},
    layout::Layout,
    Note,
};
use dioxus_signals::use_signal;

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

    let layouts = use_signal(cx, || {
        items(node, *width)
            .map(|(elem, is_newline)| match &elem {
                element::Element::Note(note) => (
                    Layout {
                        accidental: note.accidental.map(|acc| (acc, [0.; 2])),
                    },
                    elem,
                    is_newline,
                ),
                _ => todo!(),
            })
            .collect::<Vec<_>>()
    });

    let layouts_ref = layouts.read();

    let y = 0.;
    let mut left = 0.;

    let elems = layouts_ref
        .iter()
        .enumerate()
        .map(move |(idx, (layout, element, is_newline))| {
            let lines = if *is_newline {
                let mut d = String::new();
                for i in 0..5 {
                    let y = i as f64 * line_height + top + y;
                    d.push_str(&format!("M0 {y} L {width} {y} "));
                }

                let elem = render!(
                    path { d: "{d}", stroke: "#000", stroke_width: *stroke_width }
                    Br {
                        x: left + stroke_width / 2.,
                        y: y,
                        top: top,
                        line_height: *line_height,
                        stroke_width: *stroke_width
                    }
                    Br {
                        x: width - stroke_width / 2.,
                        y: y,
                        top: top,
                        line_height: *line_height,
                        stroke_width: *stroke_width
                    }
                );

                left += 20.;
                elem
            } else {
                None
            };

            let elem = match element {
                element::Element::Note(note) => {
                    let x = left;
                    left += layout.width();

                    render!(Note {
                        duration: note.duration,
                        x: x,
                        y: top + note.index() as f64 * (line_height / 2.),
                        layout: layout.clone(),
                        head_size: line_height / 2.,
                        font_size: 48.,
                        stroke_width: *stroke_width,
                        line_height: *line_height,
                        onlayout: move |layout| layouts.write()[idx].0 = layout
                    })
                }
                _ => todo!(),
            };

            render! { lines, elem }
        });

    render!(svg {
        width: "{width}px",
        height: "500px",
        xmlns: "http://www.w3.org/2000/svg",
        elems
    })
}

fn items<'a>(
    node: &'a VNode<'a>,
    width: f64,
) -> impl Iterator<Item = (element::Element, bool)> + 'a {
    let mut x = 0.;
    let mut y = 0.;
    let mut is_newline = true;

    node.template
        .get()
        .roots
        .iter()
        .map(move |root| match root {
            TemplateNode::Element {
                tag,
                namespace: _,
                attrs,
                children: _,
            } => {
                let (elem, elem_width) = match *tag {
                    "note" => (
                        element::Element::Note(element::Note::from_attrs(&node, attrs)),
                        80.,
                    ),
                    "br" => (element::Element::Br(Br {}), 30.),
                    "clef" => (element::Element::Clef(Clef {}), 60.),
                    _ => todo!(),
                };

                x += elem_width;

                let old_is_newline = is_newline;
                is_newline = x > width;
                if is_newline {
                    x = 0.;
                    y += 120.;
                }

                (elem, old_is_newline)
            }
            _ => todo!(),
        })
}

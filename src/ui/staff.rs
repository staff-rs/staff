use super::prelude::*;
use crate::ui::{
    element::{self, Clef},
    layout::Layout,
    Note,
};
use dioxus_signals::use_signal;

#[component]
fn Hr(cx: Scope, x: f64, y: f64, top: f64, line_height: f64, stroke_width: f64) -> Element {
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
        items(node)
            .map(|elem| match &elem {
                element::Element::Note(note) => (
                    Some(Layout {
                        accidental: note.accidental.map(|acc| (acc, [0.; 2])),
                        duration: note.duration,
                    }),
                    elem,
                ),
                _ => (None, elem),
            })
            .collect::<Vec<_>>()
    });

    let layouts_ref = layouts.read();

    let mut y = 0.;
    let mut left = 0.;
    let mut is_newline = true;

    let elems = layouts_ref
        .iter()
        .enumerate()
        .map(move |(idx, (layout, element))| {
            let lines = if is_newline {
                let mut d = String::new();
                for i in 0..5 {
                    let y = i as f64 * line_height + top + y;
                    d.push_str(&format!("M0 {y} L {width} {y} "));
                }

                let elem = render!(
                    path { d: "{d}", stroke: "#000", stroke_width: *stroke_width }
                    Hr {
                        x: left + stroke_width / 2.,
                        y: y,
                        top: top,
                        line_height: *line_height,
                        stroke_width: *stroke_width
                    }
                    Hr {
                        x: width - stroke_width / 2.,
                        y: y,
                        top: top,
                        line_height: *line_height,
                        stroke_width: *stroke_width
                    }
                );

               
                is_newline = false;

                elem
            } else {
                None
            };

            

            let elem = match element {
                element::Element::Note(note) => {
                    let layout = layout.as_ref().unwrap();
                    let x = left;
                    left += layout.width();
                    render!(Note {
                        duration: note.duration,
                        x: x,
                        y: top + y + note.index() as f64 * (line_height / 2.),
                        layout: layout.clone(),
                        head_size: line_height / 2.,
                        font_size: 48.,
                        stroke_width: *stroke_width,
                        line_height: *line_height,
                        onlayout: move |layout| layouts.write()[idx].0 = Some(layout)
                    })
                }
                element::Element::Hr => {
                    let x = left;
                    left += 20.;
                    if left >= *width {
                        left = 0.;
                        is_newline = true;
                    }
                    
                    render!(Hr {
                        x: x - stroke_width / 2.,
                        y: y,
                        top: top,
                        line_height: *line_height,
                        stroke_width: *stroke_width
                    })
                }
                element::Element::Br => {
                    left = 0.;
                    y += 100.;
                    is_newline = true;
                    None
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

fn items<'a>(node: &'a VNode<'a>) -> impl Iterator<Item = element::Element> + 'a {
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
            } => match *tag {
                "note" => element::Element::Note(element::Note::from_attrs(&node, attrs)),
                "br" => element::Element::Br,
                "hr" => element::Element::Hr,
                "clef" => element::Element::Clef(Clef {}),
                _ => todo!(),
            },
            _ => todo!(),
        })
}

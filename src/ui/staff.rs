use super::{element::Note, prelude::*};
use crate::{
    note::Accidental,
    ui::{
        element::{self, Clef},
        layout::Layout,
        Note,
    },
    Natural,
};
use dioxus_signals::{use_selector, use_signal, Signal};
use std::rc::Rc;

#[component]
fn Hr(cx: Scope, x: f64, y: f64, top: f64, line_height: f64, stroke_width: f64) -> Element {
    render!(path {
        d: "M{x} {top + y}L{x} {top + y + line_height * 4.}",
        stroke: "#000",
        stroke_width: *stroke_width
    })
}

pub struct NoteEvent {
    pub idx: usize,
    pub natural: Natural,
    pub accidental: Option<Accidental>,
}

#[derive(Clone, PartialEq)]
enum ItemKind {
    Br,
    Hr,
    Note { layout: Layout, note: Note },
}

#[derive(Clone, PartialEq)]
struct Item {
    x: f64,
    y: f64,
    kind: ItemKind,
}

#[component]
pub fn Staff<'a>(
    cx: Scope<'a>,

    elements: Signal<Vec<element::Element>>,

    /// Line height of the staff.
    #[props(default = 15.)]
    line_height: f64,

    /// Width of the staff.
    #[props(default = 400.)]
    width: f64,

    /// Stroke width of the items in the staff.
    #[props(default = 2.)]
    stroke_width: f64,

    onclick: EventHandler<'a, NoteEvent>,
) -> Element<'a> {
    let layouts = use_signal(cx, move || Vec::new());
    let top = *stroke_width + 100.;

    to_owned![elements];
    dioxus_signals::use_effect(cx, move || {
        let elements_ref = elements.read();
        layouts.set(
            elements_ref
                .clone()
                .into_iter()
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
                .collect::<Vec<_>>(),
        );
    });

    to_owned![width];
    let items = use_selector(cx, move || {
        let mut y = 0.;
        let mut left = 0.;
        let mut is_newline = true;

        let layouts_ref = layouts.read();
        layouts_ref
            .iter()
            .map(|(layout_cell, element)| {
                let old_is_newline = is_newline;
                is_newline = false;

                if left >= width && width > 0. {
                    left = 0.;
                    y += 140.;
                    is_newline = true;
                }

                let item = match element {
                    element::Element::Br => {
                        left = 0.;
                        y += 140.;
                        is_newline = true;

                        Item {
                            x: left,
                            y,
                            kind: ItemKind::Hr,
                        }
                    }
                    element::Element::Hr => {
                        let x = left;
                        left += 30.;

                        Item {
                            x,
                            y,
                            kind: ItemKind::Hr,
                        }
                    }
                    element::Element::Note(note) => {
                        let layout = layout_cell.as_ref().unwrap();
                        let x = left;
                        left += layout.width();

                        Item {
                            x,
                            y,
                            kind: ItemKind::Note {
                                note: note.clone(),
                                layout: layout.clone(),
                            },
                        }
                    }
                    _ => todo!(),
                };
                (item, old_is_newline)
            })
            .collect::<Vec<_>>()
    });

    let items_ref = items.read();
    let last = Rc::new(RefCell::new(None));
    let elems = items_ref
        .iter()
        .enumerate()
        .map(|(idx, (item, is_newline))| {
            let lines = if *is_newline {
                let mut d = String::new();
                for i in 0..5 {
                    let y = i as f64 * line_height + top + item.y;
                    d.push_str(&format!("M0 {y} L {width} {y} "));
                }

                render!(
                    path { d: "{d}", stroke: "#000", stroke_width: *stroke_width }
                    Hr {
                        x: item.x + stroke_width / 2.,
                        y: item.y,
                        top: top,
                        line_height: *line_height,
                        stroke_width: *stroke_width
                    }
                    Hr {
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

            let elem = match &item.kind {
                ItemKind::Br => None,
                ItemKind::Hr => {
                    render!(Hr {
                        x: item.x - stroke_width / 2.,
                        y: item.y,
                        top: top,
                        line_height: *line_height,
                        stroke_width: *stroke_width
                    })
                }
                ItemKind::Note { layout, note } => {
                    let natural = note.natural;
                    let accidental = note.accidental;

                    render!(Note {
                        duration: note.duration,
                        x: item.x,
                        y: top + item.y + note.index() as f64 * (line_height / 2.),
                        layout: layout.clone(),
                        head_size: line_height / 2.,
                        font_size: 48.,
                        stroke_width: *stroke_width,
                        line_height: *line_height,
                        last: last.clone(),
                        onlayout: move |layout| layouts.write()[idx].0 = Some(layout),
                        onclick: move |event| {
                            onclick.call(NoteEvent {
                                idx,
                                natural: natural,
                                accidental: accidental,
                            })
                        }
                    })
                }
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

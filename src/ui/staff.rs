use super::{element::StaffElement, prelude::*};
use crate::{
    note::Accidental,
    ui::{
        element::elements,
        layout::{use_layouts, LayoutElement},
        use_items, ItemKind, Note,
    },
    Natural,
};
use dioxus_signals::{use_signal, Signal};
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

/// Staff component.
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

    onclick: EventHandler<'a, NoteEvent>,
) -> Element<'a> {
    let items = elements(children.as_ref().unwrap());
    let elements = use_signal(cx, || Vec::new());

    use_effect(cx, &items, move |items| {
        elements.set(items);
        async {}
    });

    render!(StaffElements {
        elements: elements,
        line_height: *line_height,
        width: *width,
        stroke_width: *stroke_width,
        onclick: |event| onclick.call(event)
    })
}

/// Staff component.
#[component]
pub fn StaffElements<'a>(
    cx: Scope<'a>,

    /// Staff elements signal.
    elements: Signal<Vec<StaffElement>>,

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
    let layouts = use_layouts(cx, *elements);
    let items = use_items(cx, layouts, *width);

    let items_ref = items.read();
    let last = Rc::new(RefCell::new(None));
    let top = *stroke_width + 100.;
    let mut bottom = top;
    let elems = items_ref
        .iter()
        .enumerate()
        .map(|(idx, (item, is_newline))| {
            let lines = if *is_newline {
                bottom = item.y;
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
                        onlayout: move |new_layout| {
                            if let LayoutElement::Note { ref mut layout, .. } =
                                &mut *layouts.read()[idx].write()
                            {
                                *layout = new_layout;
                            }
                        },
                        onclick: move |_event| {
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
        height: "{bottom}px",
        xmlns: "http://www.w3.org/2000/svg",
        elems
    })
}

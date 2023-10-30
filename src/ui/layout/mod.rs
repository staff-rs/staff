use super::element::{Note, StaffElement};
use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};

mod note;
pub use note::{Layout, NoteLayout};

pub enum LayoutElement {
    Br,
    Hr,
    Clef,
    Note { layout: Layout, element: Note },
}

pub fn use_layouts<T>(
    cx: Scope<T>,
    elements: Signal<Vec<StaffElement>>,
) -> Signal<Vec<Signal<LayoutElement>>> {
    let layouts = use_signal(cx, move || Vec::new());

    to_owned![elements];
    dioxus_signals::use_effect(cx, move || {
        let elements_ref = elements.read();
        layouts.set(
            elements_ref
                .clone()
                .into_iter()
                .map(|elem| {
                    Signal::new(match &elem {
                        StaffElement::Note(note) => LayoutElement::Note {
                            layout: Layout {
                                accidental: note.accidental.map(|acc| (acc, [0.; 2])),
                                duration: note.duration,
                            },
                            element: note.clone(),
                        },
                        StaffElement::Br => LayoutElement::Br,
                        StaffElement::Hr => LayoutElement::Hr,
                        StaffElement::Clef(_) => LayoutElement::Clef,
                    })
                })
                .collect::<Vec<_>>(),
        );
    });

    layouts
}

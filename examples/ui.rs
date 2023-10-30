use dioxus_resize_observer::use_size;
use dioxus_signals::use_signal;
use dioxus_use_mounted::use_mounted;
use staff::{
    midi::Octave,
    note::Accidental,
    time::{Duration, DurationKind},
    ui::{
        element::{self, Note},
        prelude::*,
        Font, NoteEvent, Staff,
    },
    Natural,
};

fn app(cx: Scope) -> Element {
    let mounted = use_mounted(cx);
    let size = use_size(cx, mounted);
    let selected: &UseState<Option<NoteEvent>> = use_state(cx, || None);
    let elements = use_signal(cx, || {
        vec![
            element::Element::Note(Note {
                natural: Natural::C,
                octave: Octave::FOUR,
                accidental: Some(Accidental::Sharp),
                duration: Duration::from(DurationKind::Quarter),
            }),
            element::Element::Br,
            element::Element::Note(Note {
                natural: Natural::F,
                octave: Octave::FOUR,
                accidental: Some(Accidental::Sharp),
                duration: Duration::from(DurationKind::Quarter),
            }),
        ]
    });

    render!(
        div {
            position: "absolute",
            top: 0,
            left: 0,
            display: "flex",
            width: "100vw",
            height: "100vh",
            align_items: "center",
            justify_content: "center",
            Font {}
            if let Some(selected) = &**selected {
                let mut selected_str = format!("Selected: {}", selected.natural);
                if let Some(accidental) = selected.accidental {
                    selected_str.push_str(&accidental.to_string());
                }
                render!(h4 {"{selected_str}"})
            }
            div {
                flex: 1,
                max_width: "800px",
                margin: "50px",
                overflow: "hidden",
                onmounted: move |event| mounted.onmounted(event),
                Staff {
                    width: size.width(),
                    elements: elements,
                    onclick: |event| selected.set(Some(event))
                }
            }
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}

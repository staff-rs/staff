use dioxus_resize_observer::use_size;

use dioxus_use_mounted::use_mounted;
use staff::{
    note::Accidental,
    time::{Duration, DurationKind},
    ui::{prelude::*, Font, NoteEvent, Staff},
    Natural,
};

fn app(cx: Scope) -> Element {
    let mounted = use_mounted(cx);
    let size = use_size(cx, mounted);
    let selected: &UseState<Option<NoteEvent>> = use_state(cx, || None);

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
                Staff { width: size.width(), onclick: |event| selected.set(Some(event)),
                    for _ in 0..1000 {
                        note { natural: Natural::E }
                        note { natural: Natural::B, accidental: Accidental::Sharp, duration: Duration::from(DurationKind::Eigth) }
                        note { natural: Natural::G, duration: Duration::from(DurationKind::Eigth) }
                    }
                }
            }
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}

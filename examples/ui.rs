use staff::{
    midi::Octave,
    note::Accidental,
    time::{Duration, DurationKind},
    ui::{prelude::*, Font, Staff},
    Natural,
};

fn app(cx: Scope) -> Element {
    render!(
        div { display: "flex", width: "100vw", height: "100vh", align_items: "center", justify_content: "center",
            Font {}
            Staff {
                clef { kind: ClefKind::Treble }
                note { natural: Natural::F }
                note { natural: Natural::G, accidental: Accidental::Sharp, duration: Duration::new(DurationKind::Half, false) }
                note { natural: Natural::C, accidental: Accidental::Flat, octave: Octave::FIVE }
                br {}

                note { natural: Natural::A, duration: Duration::new(DurationKind::Whole, false) }
            }
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}

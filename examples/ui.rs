use staff::{
    note::Accidental,
    time::{Duration, DurationKind},
    ui::{prelude::*, Font, Staff},
    Natural, midi::Octave,
};

fn app(cx: Scope) -> Element {
    render!(
        div { display: "flex", width: "100vw", height: "100vh", align_items: "center", justify_content: "center",
            Font {}
            Staff { 
                note { natural: Natural::F, accidental: Accidental::Sharp }
                note { natural: Natural::G, accidental: Accidental::Flat, duration: Duration::from(DurationKind::Half) }
                note { natural: Natural::A }
                hr {}
                note { natural: Natural::C, octave: Octave::FIVE, duration: Duration::from(DurationKind::Whole) }
            }
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}

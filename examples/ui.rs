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
                note { natural: Natural::E, duration: Duration::from(DurationKind::Eigth) }
                note { natural: Natural::B, duration: Duration::from(DurationKind::Eigth) }
                note { natural: Natural::G, duration: Duration::from(DurationKind::Half) }
                hr {}
                note { natural: Natural::E, duration: Duration::from(DurationKind::Eigth) }
                note { natural: Natural::B, duration: Duration::from(DurationKind::Eigth) }
                note { natural: Natural::G, duration: Duration::from(DurationKind::Half) }

                hr {}

                note { natural: Natural::E, duration: Duration::from(DurationKind::Eigth) }
                note { natural: Natural::B, duration: Duration::from(DurationKind::Eigth) }
                note { natural: Natural::G, duration: Duration::from(DurationKind::Half) }

                br {}

                note { natural: Natural::G, duration: Duration::from(DurationKind::Quarter) }
                note { natural: Natural::E, duration: Duration::new(DurationKind::Quarter, true) }
                note { natural: Natural::D, duration: Duration::from(DurationKind::Eigth) }

                hr {}

                note { natural: Natural::C, duration: Duration::from(DurationKind::Whole) }
            }
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}

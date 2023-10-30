use staff::{
    note::Accidental,
    ui::{prelude::*, Font, Staff},
    Natural, time::{Duration, DurationKind},
};

fn app(cx: Scope) -> Element {
    render!(
        div { display: "flex", width: "100vw", height: "100vh", align_items: "center", justify_content: "center",
            Font {}
            Staff { 
                note { natural: Natural::F, accidental: Accidental::Sharp }
                note { natural: Natural::A }
                note { natural: Natural::G, accidental: Accidental::Flat , duration: Duration::new(DurationKind::Half, false) }
            }
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}

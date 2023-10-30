use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;
use staff::{
    time::{Duration, DurationKind},
    ui::{prelude::*, Font, Staff},
    Natural,
};

fn app(cx: Scope) -> Element {
    let mounted = use_mounted(cx);
    let size = use_size(cx, mounted);

    render!(
        div { display: "flex", width: "100vw", height: "100vh", align_items: "center", justify_content: "center",
            Font {}
            div {
                flex: 1,
                max_width: "800px",
                overflow: "hidden",
                onmounted: move |event| mounted.onmounted(event),
                Staff { width: size.width(),
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
                    hr {}
                    note { natural: Natural::G, duration: Duration::from(DurationKind::Quarter) }
                    note { natural: Natural::E, duration: Duration::new(DurationKind::Quarter, true) }
                    note { natural: Natural::D, duration: Duration::from(DurationKind::Eigth) }
                    hr {}
                    note { natural: Natural::C, duration: Duration::from(DurationKind::Whole) }
                }
            }
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}

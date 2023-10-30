use staff::ui::{prelude::*, FretDiagram};

fn app(cx: Scope) -> Element {
    render!(
        svg { width: "500px", height: "500px", xmlns: "http://www.w3.org/2000/svg",
            FretDiagram { 
                fret { index: 1, string: 0 }
                frets { from: 2, to: 4, string: 0 }
                fret { index: 5, string: 0, is_muted: true }
            }
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}

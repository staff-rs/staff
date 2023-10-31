use staff::ui::{prelude::*, FretDiagram};

fn app(cx: Scope) -> Element {
    render!(
        svg { width: "500px", height: "500px", xmlns: "http://www.w3.org/2000/svg",
            FretDiagram { 
                fret { index: 1, string: 0 }
                frets { index: 2, from: 2, to: 4 }
            }
        }
    )
}

fn main() {
    dioxus_web::launch(app);
}

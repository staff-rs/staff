use dioxus::prelude::*;

#[component]
pub fn FretDiagram<'a>(
    cx: Scope<'a>,

    children: Element<'a>,

    /// Amount of strings in the diagram.
    #[props(default = 6)]
    string_count: u8,

    /// Amount of frets in the diagram.
    #[props(default = 8)]
    fret_count: u8,

    /// Top-most position of the diagram.
    #[props(default = 0.)]
    x: f64,

    /// Left-most position of the diagram.
    #[props(default = 0.)]
    y: f64,

    /// Width of the diagram.
    #[props(default = 150.)]
    width: f64,

    /// Height of the diagram.
    #[props(default = 200.)]
    height: f64,

    /// Stroke width of items in the diagram.
    #[props(default = 2.)]
    stroke_width: f64,
) -> Element<'a> {
    let fret_width = *width / (*string_count - 1) as f64;
    let fret_height = (*height - stroke_width * 6.) / (*fret_count - 1) as f64;

    let fret_lines = (1..*fret_count).map(|i| {
        let line_y = fret_height * i as f64 + stroke_width * 5.5;
        render!(path { stroke: "#000", stroke_width: *stroke_width, d: "M{x} {line_y}L{width} {line_y}" })
    });

    let string_lines = (0..*string_count).map(|i| {
        let line_x = fret_width * i as f64 + stroke_width / 2.;
        render!(path { stroke: "#000", stroke_width: *stroke_width, d: "M{line_x} {y}L{line_x} {height}" })
    });

    render! {
        path {
            stroke: "#000",
            stroke_width: *stroke_width * 4.,
            d: "M{x} {*stroke_width * 2.}L{width} {*stroke_width * 2.}"
        }
        fret_lines,
        string_lines
    }
}

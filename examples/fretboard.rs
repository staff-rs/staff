use staff::{
    fretboard::{Diagram, Fretted, Renderer},
    render::font,
};

fn main() {
    let fretted = [
        Fretted::muted(0, 0),
        Fretted::new(1, 1, 6),
        Fretted::point(2, 3),
        Fretted::point(3, 2),
        Fretted::point(3, 4),
    ];

    let mut diagram: Diagram = fretted.into_iter().collect();
    diagram.starting_fret = 3;

    let renderer = Renderer::new(diagram, 150., 200.);
    let font = font();
    let svg = renderer.svg(0., 0., 10., 2., 72., &font);
    svg::save("./fretboard.svg", &svg).unwrap();
}

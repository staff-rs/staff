use staff::render::{
    font,
    fret_diagram::{Diagram, Range, Renderer},
};

fn main() {
    let fretted = [
        Range::muted(0, 0),
        Range::new(1, 1, 6),
        Range::point(2, 3),
        Range::point(3, 2),
        Range::point(3, 4),
    ];

    let mut diagram: Diagram = fretted.into_iter().collect();
    diagram.starting_fret = 3;

    let renderer = Renderer::new(diagram, 150., 200.);
    let font = font();
    let svg = renderer.svg(0., 0., 10., 2., 72., &font);
    svg::save("./fretboard.svg", &svg).unwrap();
}

use staff::{
    fretboard::{Diagram, Fretted, Renderer},
    render::font,
};

fn main() {
    let mut diagram = Diagram::default();
    diagram.starting_fret = 3;
    diagram.insert(Fretted::muted(0, 0));
    diagram.insert(Fretted::barre(1, 1, 6));
    diagram.insert(Fretted::point(2, 3));
    diagram.insert(Fretted::point(3, 2));
    diagram.insert(Fretted::point(3, 4));

    let fretboard = Renderer::new(diagram, 150., 200.);

    let font = font();
    let svg = fretboard.svg(0., 0., 10., 2., 72., &font);
    svg::save("./fretboard.svg", &svg).unwrap();
}

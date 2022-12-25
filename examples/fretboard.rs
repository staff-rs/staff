use staff::render::{
    font,
    fretboard::{Fretted, Fretboard},
};

fn main() {
    let mut fretboard = Fretboard::builder().build(150., 200.);
    fretboard.insert(Fretted::new(0, 3..3));
    fretboard.insert(Fretted::new(2, 0..1));
    fretboard.insert(Fretted::new(1, 0..3));

    let font = font();
    let svg = fretboard.svg(0., 0., &font);
    svg::save("./fretboard.svg", &svg).unwrap();
}

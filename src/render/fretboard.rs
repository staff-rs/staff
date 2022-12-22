use super::Draw;
use std::ops::Range;
use svg::node::element::Rectangle;

pub struct Fret {
    pos: usize,
    strings: Range<u8>,
}

impl Fret {
    pub fn new(pos: usize, strings: Range<u8>) -> Self {
        Self { pos, strings }
    }
}

pub struct Fretboard {
    frets: Vec<Fret>,
}

impl Draw for Fretboard {
    fn draw(&self, x: f64, y: f64, renderer: &super::Renderer, node: &mut impl svg::Node) {
        let padding = 20.;

        let width = renderer.width - padding * 2.;
        let height = renderer.height - padding * 2.;
        let fret_width = width / 5.;
        let fret_height = height / 6.;

        // TODO
        let x = x + padding;
        let y = y + padding;

        for idx in 0..6 {
            let line_x = x + fret_width * idx as f64;
            renderer.draw_line(node, line_x, y, line_x, height);
        }

        for idx in 0..6 {
            let line_y = y + fret_height * idx as f64;
            renderer.draw_line(node, x, line_y, width + padding, line_y);
        }

        for fret in &self.frets {
            let circle_height = fret_height / 1.5;
            let rect = Rectangle::new()
                .set(
                    "x",
                    x + fret_width * fret.strings.start as f64 - circle_height / 2.,
                )
                .set("y", fret.pos as f64 * fret_height + y + circle_height / 4.)
                .set(
                    "width",
                    fret_width * (fret.strings.end - fret.strings.start) as f64 + circle_height,
                )
                .set("height", circle_height)
                .set("rx", circle_height / 2.);
            node.append(rect);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Fret, Fretboard};
    use crate::render::Renderer;

    #[test]
    fn f() {
        let frets = vec![Fret::new(0, 1..5), Fret::new(2, 0..2), Fret::new(3, 1..1)];
        let fretboard = Fretboard { frets };

        let mut renderer = Renderer::default();
        renderer.width = 200.;
        renderer.height = 250.;
        let svg = renderer.render(&fretboard);
        svg::save("./fretboard.svg", &svg).unwrap();
    }
}

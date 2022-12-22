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
        let fret_height = height / 7.;

        // TODO
        let x = x + padding;
        let y = y + padding;

        for idx in 0..6 {
            let line_x = x + fret_width * idx as f64;
            renderer.draw_line(node, line_x, y + fret_height, line_x, height);
        }

        for idx in 0..6 {
            let line_y = y + fret_height * idx as f64 + fret_height;
            renderer.draw_line(node, x, line_y, width + padding, line_y);
        }

        for fret in &self.frets {
            let draw_height = fret_height / 1.5;

            if fret.strings.start >= fret.strings.end {
                let x = x + fret_width * fret.strings.start as f64 - fret_width / 4.;
                renderer.draw_line(
                    node,
                    x,
                    y + fret_height / 4.,
                    x + fret_width / 2.,
                    y + fret_height * 0.75,
                );
                renderer.draw_line(
                    node,
                    x + fret_width / 2.,
                    y + fret_height / 4.,
                    x,
                    y + fret_height * 0.75,
                );
            } else {
                let mut rect = Rectangle::new()
                    .set(
                        "x",
                        x + fret_width * fret.strings.start as f64 - draw_height / 2.,
                    )
                    .set("y", fret.pos as f64 * fret_height + y + draw_height / 4.)
                    .set(
                        "width",
                        fret_width * (fret.strings.end - 1 - fret.strings.start) as f64
                            + draw_height,
                    )
                    .set("height", draw_height)
                    .set("rx", draw_height / 2.);

                if fret.pos == 0 {
                    rect = rect.set("stroke", "#000").set("fill", "transparent")
                }

                node.append(rect);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Fret, Fretboard};
    use crate::render::Renderer;

    #[test]
    fn f() {
        let frets = vec![Fret::new(0, 3..3), Fret::new(0, 0..1), Fret::new(3, 1..2)];
        let fretboard = Fretboard { frets };

        let mut renderer = Renderer::default();
        renderer.width = 200.;
        renderer.height = 250.;
        let svg = renderer.render(&fretboard);
        svg::save("./fretboard.svg", &svg).unwrap();
    }
}

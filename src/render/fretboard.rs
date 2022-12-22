use super::Draw;
use std::ops::Range;
use svg::node::element::Rectangle;
use text_svg::Glpyh;

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
    starting_fret: u8,
    frets: Vec<Fret>,
}

impl Draw for Fretboard {
    fn draw(&self, x: f64, y: f64, renderer: &super::Renderer, node: &mut impl svg::Node) {
        let font_size = 48.;
        let letter_spacing = 2.;

        let padding = 10.;
        let margin = (padding + (font_size + letter_spacing) * 2.) * 2.;

        let width = renderer.width - margin;
        let height = renderer.height - padding * 2.;
        let fret_width = width / 6.;
        let fret_height = height / 7.;

        // TODO
        let mut x = x + padding;
        let mut y = y + padding;

        if self.starting_fret > 0 {
            x += (font_size + letter_spacing) * 2.;
            let mut glyph_x = x;

            let glyph_y = y + fret_height;
            let s = self.starting_fret.to_string();

            for c in s.chars().rev() {
                let glyph = Glpyh::new(&renderer.font, c, font_size as _);
                glyph_x -= glyph.bounding_box.width() as f64 + letter_spacing;
                node.append(glyph.path(glyph_x as _, glyph_y as _));
            }
        }

        x += fret_width / 2.;

        for idx in 0..6 {
            let line_x = x + fret_width * idx as f64;
            renderer.draw_line(node, line_x, y + fret_height, line_x, height);
        }

        let line_y = y + fret_height;
        renderer.draw_line_with_stroke_width(
            node,
            x - renderer.stroke_width / 2.,
            line_y,
            x + fret_width * 5. + renderer.stroke_width / 2.,
            line_y,
            renderer.stroke_width * 2.,
        );
        y += renderer.stroke_width;

        for idx in 1..6 {
            let line_y = line_y + fret_height * idx as f64;
            renderer.draw_line(
                node,
                x - renderer.stroke_width / 2.,
                line_y,
                x + fret_width * 5. + renderer.stroke_width / 2.,
                line_y,
            );
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
        let fretboard = Fretboard {
            starting_fret: 12,
            frets,
        };

        let mut renderer = Renderer::default();
        renderer.width = 400.;
        renderer.height = 250.;
        let svg = renderer.render(&fretboard);
        svg::save("./fretboard.svg", &svg).unwrap();
    }
}

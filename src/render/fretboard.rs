use super::Draw;
use std::{mem, ops::Range};
use svg::node::element::Rectangle;
use text_svg::Glpyh;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fret {
    pos: u8,
    strings: Range<u8>,
}

impl Fret {
    pub fn new(pos: u8, strings: Range<u8>) -> Self {
        Self { pos, strings }
    }

    pub fn point(pos: u8, string: u8) -> Self {
        Self::new(pos, string..string + 1)
    }
}

pub struct Fretboard {
    frets: Vec<Fret>,
    starting_fret: u8,
    fret_count: u8,
    strings: u8,
    font_size: f64,
    letter_spacing: f64,
    padding: f64,
}

impl Default for Fretboard {
    fn default() -> Self {
        Self {
            frets: Vec::new(),
            starting_fret: 12,
            fret_count: 5,
            strings: 6,
            font_size: 48.,
            letter_spacing: 2.,
            padding: 10.,
        }
    }
}

impl Fretboard {
    pub fn push(&mut self, fret: Fret) -> Result<(), Fret> {
        if fret.pos >= self.strings {
            return Err(fret);
        }

        if fret.strings.start >= self.fret_count || fret.strings.end >= self.fret_count {
            return Err(fret);
        }

        let is_intersection =
            self.frets.iter().filter(|f| f.pos == fret.pos).any(|f| {
                f.strings.start <= fret.strings.end && fret.strings.start <= f.strings.end
            });

        if !is_intersection {
            self.frets.push(fret);
            Ok(())
        } else {
            Err(fret)
        }
    }

    pub fn set_strings(&mut self, strings: u8) {
        self.strings = strings;

        let frets = mem::replace(&mut self.frets, Vec::new());
        let iter = frets.into_iter().filter(|fret| fret.pos < strings);
        self.frets.extend(iter);
    }
}

impl Draw for Fretboard {
    fn draw(&self, x: f64, y: f64, renderer: &super::Renderer, node: &mut impl svg::Node) {
        let margin = (self.padding + (self.font_size + self.letter_spacing) * 2.) * 2.;

        let width = renderer.width - margin;
        let height = renderer.height - self.padding * 2.;
        let fret_width = width / (self.strings) as f64;
        let fret_height = height / (self.fret_count + 1) as f64;

        let mut x = x + self.padding;
        let mut y = y + self.padding;

        x += (self.font_size + self.letter_spacing) * 2.;
        if self.starting_fret > 0 {
            let mut glyph_x = x;

            let glyph_y = y + fret_height;
            let s = self.starting_fret.to_string();

            for c in s.chars().rev() {
                let glyph = Glpyh::new(&renderer.font, c, self.font_size as _);
                glyph_x -= glyph.bounding_box.width() as f64 + self.letter_spacing;
                node.append(glyph.path(glyph_x as _, glyph_y as _));
            }
        }

        x += fret_width / 2.;

        for idx in 0..self.strings {
            let line_x = x + fret_width * idx as f64;
            renderer.draw_line(node, line_x, y + fret_height, line_x, height - self.padding);
        }

        let line_y = y + fret_height;
        renderer.draw_line_with_stroke_width(
            node,
            x - renderer.stroke_width / 2.,
            line_y,
            x + (fret_width * (self.strings - 1) as f64) + renderer.stroke_width / 2.,
            line_y,
            renderer.stroke_width * 2.,
        );
        y += renderer.stroke_width;

        for idx in 1..self.fret_count {
            let line_y = line_y + fret_height * idx as f64;
            renderer.draw_line(
                node,
                x - renderer.stroke_width / 2.,
                line_y,
                x + fret_width * (self.strings - 1) as f64 + renderer.stroke_width / 2.,
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
        let mut fretboard = Fretboard::default();
        fretboard.push(Fret::new(0, 3..3)).unwrap();
        fretboard.push(Fret::new(2, 0..1)).unwrap();
        fretboard.push(Fret::new(1, 0..3)).unwrap();

        let mut renderer = Renderer::default();
        renderer.width = 400.;
        renderer.height = 250.;
        let svg = renderer.render(&fretboard);
        svg::save("./fretboard.svg", &svg).unwrap();
    }
}

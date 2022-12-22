use crate::{fretboard::STANDARD, midi::MidiNote};

use super::{Draw, Renderer};
use std::{mem, ops::Range};
use svg::node::element::Rectangle;
use text_svg::Glpyh;

pub type Iter = crate::fretboard::Fretboard<[MidiNote; 6], Vec<Option<u8>>>;

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

pub struct Builder {
    pub starting_fret: u8,
    pub fret_count: u8,
    pub strings: u8,
    pub font_size: f64,
    pub letter_spacing: f64,
    pub padding: f64,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            starting_fret: 12,
            fret_count: 5,
            strings: 6,
            font_size: 48.,
            letter_spacing: 2.,
            padding: 10.,
        }
    }
}

impl Builder {
    pub fn build(self, renderer: &Renderer) -> Fretboard {
        let margin = (self.padding + (self.font_size + self.letter_spacing) * 2.) * 2.;
        let width = renderer.width - margin;
        let height = renderer.height - self.padding * 2.;
        let fret_width = width / (self.strings) as f64;
        let fret_height = height / (self.fret_count + 1) as f64;

        Fretboard {
            margin,
            width,
            height,
            fret_width,
            fret_height,
            frets: Vec::new(),
            builder: self,
        }
    }
}

pub struct Fretboard {
    pub builder: Builder,
    pub frets: Vec<Fret>,
    pub margin: f64,
    pub width: f64,
    pub height: f64,
    pub fret_width: f64,
    pub fret_height: f64,
}

impl Fretboard {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn push(&mut self, fret: Fret) -> Result<(), Fret> {
        if fret.pos >= self.builder.strings {
            return Err(fret);
        }

        if fret.strings.start >= self.builder.fret_count
            || fret.strings.end >= self.builder.fret_count
        {
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

    pub fn shrink_strings(&mut self, strings: u8) {
        self.builder.strings = strings;

        let frets = mem::replace(&mut self.frets, Vec::new());
        let iter = frets.into_iter().filter(|fret| fret.pos < strings);
        self.frets.extend(iter);
    }

    pub fn shrink_fret_count(&mut self, strings: u8) {
        self.builder.fret_count = strings;

        let frets = mem::replace(&mut self.frets, Vec::new());
        let iter = frets.into_iter().filter(|fret| {
            fret.strings.start >= self.builder.fret_count
                || fret.strings.end >= self.builder.fret_count
        });
        self.frets.extend(iter);
    }

    pub fn pos(&self, x: f64, y: f64) -> Option<(u8, u8)> {
        if x > self.margin && y > self.builder.padding {
            let string = (x - self.margin) / self.fret_width;
            let fret = (y - self.builder.padding) / self.fret_height;
            Some((string.round() as _, fret.round() as _))
        } else {
            None
        }
    }

    pub fn midi_notes(&self) -> Iter {
        let mut frets: Vec<Option<u8>> = vec![None; self.builder.strings as _];
        for fret in &self.frets {
            let pos = fret.pos + self.builder.starting_fret;
            for idx in fret.strings.clone() {
                if let Some(last) = &mut frets[idx as usize] {
                    *last = (*last).max(pos);
                } else {
                    frets[idx as usize] = Some(pos);
                }
            }
        }

        Iter::new(STANDARD, frets)
    }
}

impl Draw for Fretboard {
    fn draw(&self, x: f64, y: f64, renderer: &super::Renderer, node: &mut impl svg::Node) {
        let mut x = x + self.builder.padding;
        let mut y = y + self.builder.padding;

        x += (self.builder.font_size + self.builder.letter_spacing) * 2.;
        if self.builder.starting_fret > 0 {
            let mut glyph_x = x;

            let glyph_y = y + self.fret_height;
            let s = self.builder.starting_fret.to_string();

            for c in s.chars().rev() {
                let glyph = Glpyh::new(&renderer.font, c, self.builder.font_size as _);
                glyph_x -= glyph.bounding_box.width() as f64 + self.builder.letter_spacing;
                node.append(glyph.path(glyph_x as _, glyph_y as _));
            }
        }

        x += self.fret_width / 2.;

        for idx in 0..self.builder.strings {
            let line_x = x + self.fret_width * idx as f64;
            renderer.draw_line(
                node,
                line_x,
                y + self.fret_height,
                line_x,
                self.height - self.builder.padding,
            );
        }

        let line_y = y + self.fret_height;
        renderer.draw_line_with_stroke_width(
            node,
            x - renderer.stroke_width / 2.,
            line_y,
            x + (self.fret_width * (self.builder.strings - 1) as f64) + renderer.stroke_width / 2.,
            line_y,
            renderer.stroke_width * 2.,
        );
        y += renderer.stroke_width;

        for idx in 1..self.builder.fret_count {
            let line_y = line_y + self.fret_height * idx as f64;
            renderer.draw_line(
                node,
                x - renderer.stroke_width / 2.,
                line_y,
                x + self.fret_width * (self.builder.strings - 1) as f64
                    + renderer.stroke_width / 2.,
                line_y,
            );
        }

        for fret in &self.frets {
            let draw_height = self.fret_height / 1.5;

            if fret.strings.start >= fret.strings.end {
                let x = x + self.fret_width * fret.strings.start as f64 - self.fret_width / 4.;
                renderer.draw_line(
                    node,
                    x,
                    y + self.fret_height / 4.,
                    x + self.fret_width / 2.,
                    y + self.fret_height * 0.75,
                );
                renderer.draw_line(
                    node,
                    x + self.fret_width / 2.,
                    y + self.fret_height / 4.,
                    x,
                    y + self.fret_height * 0.75,
                );
            } else {
                let mut rect = Rectangle::new()
                    .set(
                        "x",
                        x + self.fret_width * fret.strings.start as f64 - draw_height / 2.,
                    )
                    .set(
                        "y",
                        fret.pos as f64 * self.fret_height + y + draw_height / 4.,
                    )
                    .set(
                        "width",
                        self.fret_width * (fret.strings.end - 1 - fret.strings.start) as f64
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
        let mut renderer = Renderer::default();
        let mut fretboard = Fretboard::builder().build(&renderer);
        fretboard.push(Fret::new(0, 3..3)).unwrap();
        fretboard.push(Fret::new(2, 0..1)).unwrap();
        fretboard.push(Fret::new(1, 0..3)).unwrap();

        renderer.width = 400.;
        renderer.height = 250.;
        let svg = renderer.render(&fretboard);
        svg::save("./fretboard.svg", &svg).unwrap();
    }
}

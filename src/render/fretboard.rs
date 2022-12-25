use crate::{fretboard::STANDARD, midi::MidiNote};
use std::{mem, ops::Range};

#[cfg(feature = "svg")]
use svg::node::{element, Node};

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

pub type Iter = crate::fretboard::Fretboard<[MidiNote; 6], Vec<Option<u8>>>;

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fretted {
    pub pos: u8,

    strings: Range<u8>,
}

impl Fretted {
    pub fn new(pos: u8, strings: Range<u8>) -> Self {
        Self { pos, strings }
    }

    pub fn point(pos: u8, string: u8) -> Self {
        Self::new(pos, string..string + 1)
    }

    pub fn is_intersection(&self, other: &Self) -> bool {
        self.pos == other.pos
            && self.strings.start < other.strings.end
            && self.strings.end > other.strings.start
    }

    // TODO for wasm-bindgen
    pub fn strings(&self) -> Range<u8> {
        self.strings.clone()
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
            font_size: 72.,
            letter_spacing: 2.,
            padding: 10.,
        }
    }
}

impl Builder {
    pub fn build(self, width: f64, height: f64) -> Fretboard {
        let height = height - self.padding * 2.;
        let fret_width = fret_width(width, self.strings);
        let fret_height = height / (self.fret_count + 1) as f64;

        Fretboard {
            width,
            height,
            fret_width,
            fret_height,
            frets: Vec::new(),
            builder: self,
        }
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub stroke_width: f64,
}

impl Line {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, stroke_width: f64) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            stroke_width,
        }
    }

    #[cfg(feature = "svg")]
    pub fn svg<T: Node>(&self, x: f64, node: &mut T) {
        node.append(
            element::Line::new()
                .set("stroke", "#000")
                .set("stroke-width", self.stroke_width)
                .set("x1", x + self.x1)
                .set("y1", self.y1)
                .set("x2", x + self.x2)
                .set("y2", self.y2),
        )
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub stroke_width: f64,
    pub is_filled: bool,
}

impl Rectangle {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        stroke_width: f64,
        is_filled: bool,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            stroke_width,
            is_filled,
        }
    }

    #[cfg(feature = "svg")]
    pub fn svg(&self, _x: f64, node: &mut impl svg::Node) {
        let element = element::Rectangle::new()
            .set("fill", "#000")
            .set("stroke-width", self.stroke_width)
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("rx", self.height / 2.);

        let styled = if self.is_filled {
            element.set("fill", "#000")
        } else {
            element
        };

        node.append(styled)
    }
}

pub enum Marker {
    Rectangle(Rectangle),
    Cross { lines: [Line; 2] },
}

pub struct Fretboard {
    pub builder: Builder,
    pub frets: Vec<Fretted>,
    pub width: f64,
    pub height: f64,
    pub fret_width: f64,
    pub fret_height: f64,
}

impl Fretboard {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn insert(&mut self, fret: Fretted) -> Option<usize> {
        if fret.pos >= self.builder.fret_count
            || fret.strings.start > self.builder.strings
            || fret.strings.end > self.builder.strings
        {
            return None;
        }

        if let Some(idx) = self.intersection(&fret) {
            Some(idx)
        } else {
            self.frets.push(fret);
            None
        }
    }

    pub fn shrink_strings(&mut self, strings: u8) {
        self.builder.strings = strings;
        self.fret_width = fret_width(self.width, strings);

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
        let x = x - self.fret_width / 2.;
        let string = x / self.fret_width;
        let fret = (y - self.builder.padding) / self.fret_height;
        Some((string.round() as _, fret.round() as _))
    }

    pub fn intersection(&self, fret: &Fretted) -> Option<usize> {
        self.frets.iter().position(|f| f.is_intersection(fret))
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

    pub fn render_grid(&self, mut draw_line: impl FnMut(Line)) {
        let x = self.fret_width / 2.;
        let mut y = 0.;
        let stroke_width = 2.;
        for idx in 0..self.builder.strings {
            let line_x = x + self.fret_width * idx as f64;
            draw_line(Line::new(
                line_x,
                y + self.fret_height,
                line_x,
                self.height - self.builder.padding,
                stroke_width,
            ));
        }

        let line_y = y + self.fret_height;
        draw_line(Line::new(
            x - stroke_width / 2.,
            line_y,
            x + (self.fret_width * (self.builder.strings - 1) as f64) + stroke_width / 2.,
            line_y,
            stroke_width * 2.,
        ));
        y += stroke_width;

        for idx in 1..self.builder.fret_count {
            let line_y = line_y + self.fret_height * idx as f64;
            draw_line(Line::new(
                x - stroke_width / 2.,
                line_y,
                x + self.fret_width * (self.builder.strings - 1) as f64 + stroke_width / 2.,
                line_y,
                stroke_width,
            ));
        }
    }

    pub fn render_single_fretted(
        &self,
        x: f64,
        y: f64,
        stroke_width: f64,
        fret: &Fretted,
        mut draw_fretted: impl FnMut(Marker),
    ) {
        let x = x + self.fret_width / 2.;
        let draw_height = self.fret_height / 1.5;

        if fret.strings.start >= fret.strings.end {
            let x = x + self.fret_width * fret.strings.start as f64 - self.fret_width / 4.;
            let lines = [
                Line::new(
                    x,
                    y + self.fret_height / 4.,
                    x + self.fret_width / 2.,
                    y + self.fret_height * 0.75,
                    stroke_width,
                ),
                Line::new(
                    x + self.fret_width / 2.,
                    y + self.fret_height / 4.,
                    x,
                    y + self.fret_height * 0.75,
                    stroke_width,
                ),
            ];
            draw_fretted(Marker::Cross { lines })
        } else {
            let rect = Rectangle::new(
                x + self.fret_width * fret.strings.start as f64 - draw_height / 2.,
                fret.pos as f64 * self.fret_height + y + draw_height / 4.,
                self.fret_width * (fret.strings.end - 1 - fret.strings.start) as f64 + draw_height,
                draw_height,
                draw_height / 2.,
                true,
            );

            if fret.pos == 0 {
                //rect = rect.set("stroke", "#000").set("fill", "transparent")
            }

            draw_fretted(Marker::Rectangle(rect))
        }
    }

    pub fn render_fretted(
        &self,
        x: f64,
        y: f64,
        stroke_width: f64,
        mut draw_fretted: impl FnMut(Marker),
    ) {
        for fret in &self.frets {
            self.render_single_fretted(x, y, stroke_width, fret, &mut draw_fretted)
        }
    }

    #[cfg(feature = "svg")]
    pub fn svg(&self, x: f64, y: f64, font: &rusttype::Font) -> svg::Document {
        use svg::Node;

        let glyphs_width = (self.builder.font_size + self.builder.letter_spacing) * 2.;
        let mut document = svg::Document::new()
            .set("width", self.width + glyphs_width)
            .set("height", self.height);

        document.append(
            element::Rectangle::new()
                .set("fill", "#fff")
                .set("x", 0)
                .set("y", 0)
                .set("width", self.width + glyphs_width)
                .set("height", self.height),
        );

        let mut x = x + self.builder.padding;
        let y = y + self.builder.padding;

        x += glyphs_width;

        if self.builder.starting_fret > 0 {
            let mut glyph_x = x;

            let glyph_y = y + self.fret_height;
            let s = self.builder.starting_fret.to_string();

            for c in s.chars().rev() {
                let glyph = text_svg::Glpyh::new(font, c, self.builder.font_size as _);
                glyph_x -= glyph.bounding_box.width() as f64 + self.builder.letter_spacing;
                document.append(glyph.path(glyph_x as _, glyph_y as _));
            }
        }

        self.render_grid(|line| line.svg(x, &mut document));

        self.render_fretted(x, 0., 2., |fretted| match fretted {
            Marker::Cross { lines } => {
                for line in lines {
                    line.svg(0., &mut document)
                }
            }
            Marker::Rectangle(rect) => {
                rect.svg(0., &mut document);
            }
        });

        document
    }
}

fn fret_width(width: f64, strings: u8) -> f64 {
    width / strings as f64
}

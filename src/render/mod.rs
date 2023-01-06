//! Sheet music engraving

#[cfg(feature = "svg")]
pub mod staff;
#[cfg(feature = "svg")]
pub use self::staff::Staff;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(tag = "kind", content = "element", rename_all = "kebab-case")
)]
#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    Line(Line),
    Path(String),
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
    pub fn svg<T: svg::Node>(&self, x: f64, node: &mut T) {
        use svg::node::element;

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
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
    pub fn svg(&self, node: &mut impl svg::Node) {
        use svg::node::element;

        let element = element::Rectangle::new()
            .set("stroke", "#000")
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

#[cfg(feature = "svg")]
pub fn font() -> rusttype::Font<'static> {
    use std::{fs::File, io::Read};

    use font_kit::{
        family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
    };
    use rusttype::Font;

    let handle = SystemSource::new()
        .select_best_match(
            &[
                FamilyName::Title("Noto Music".to_owned()),
                FamilyName::Serif,
            ],
            &Properties::new(),
        )
        .unwrap();

    match handle {
        Handle::Path { path, font_index } => {
            let mut file = File::open(path).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            Font::try_from_vec_and_index(buf, font_index).unwrap()
        }
        Handle::Memory { bytes, font_index } => {
            Font::try_from_vec_and_index(bytes.to_vec(), font_index).unwrap()
        }
    }
}

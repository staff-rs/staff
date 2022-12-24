//! Sheet music engraving

pub mod fretboard;

#[cfg(feature = "svg")]
pub mod measure;

#[cfg(feature = "svg")]
mod note;
#[cfg(feature = "svg")]
pub use self::note::Note;

#[cfg(feature = "svg")]
pub mod staff;
#[cfg(feature = "svg")]
pub use self::staff::Staff;

#[cfg(feature = "svg")]
pub mod renderer;
#[cfg(feature = "svg")]
pub use renderer::{Draw, Renderer};

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

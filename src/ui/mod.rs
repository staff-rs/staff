pub mod element;
use dioxus::prelude::*;
use dioxus_resize_observer::{use_size, Rect};
use dioxus_use_mounted::use_mounted;

mod fret_diagram;
pub use fret_diagram::FretDiagram;

mod font;
pub use font::Font;

mod item;
pub use item::{use_items, Item, ItemKind};

pub mod layout;

mod note;
pub use note::Note;

mod staff;
pub use staff::{NoteEvent, Staff, StaffElements};

pub mod prelude {
    pub use dioxus::prelude::*;

    #[repr(u8)]
    pub enum ClefKind {
        Treble,
        Bass,
    }

    impl<'a> IntoAttributeValue<'a> for ClefKind {
        fn into_value(self, _bump: &'a bumpalo::Bump) -> dioxus::core::AttributeValue<'a> {
            dioxus::core::AttributeValue::Int(self as u8 as _)
        }
    }

    pub mod dioxus_elements {
        pub use dioxus::prelude::dioxus_elements::*;

        pub type AttributeDiscription = (&'static str, Option<&'static str>, bool);

        #[allow(non_camel_case_types)]
        pub struct note {}

        impl note {
            pub const TAG_NAME: &'static str = "note";
            pub const NAME_SPACE: Option<&'static str> = None;

            #[allow(non_upper_case_globals)]
            pub const natural: AttributeDiscription = ("natural", None, true);

            #[allow(non_upper_case_globals)]
            pub const octave: AttributeDiscription = ("octave", None, false);

            #[allow(non_upper_case_globals)]
            pub const accidental: AttributeDiscription = ("accidental", None, false);

            #[allow(non_upper_case_globals)]
            pub const duration: AttributeDiscription = ("duration", None, false);
        }

        #[allow(non_camel_case_types)]
        pub struct clef {}

        impl clef {
            pub const TAG_NAME: &'static str = "clef";
            pub const NAME_SPACE: Option<&'static str> = None;

            #[allow(non_upper_case_globals)]
            pub const kind: AttributeDiscription = ("kind", None, true);
        }

        #[allow(non_camel_case_types)]
        pub struct br {}

        impl br {
            pub const TAG_NAME: &'static str = "br";
            pub const NAME_SPACE: Option<&'static str> = None;
        }

        #[allow(non_camel_case_types)]
        pub struct hr {}

        impl hr {
            pub const TAG_NAME: &'static str = "hr";
            pub const NAME_SPACE: Option<&'static str> = None;
        }

        #[allow(non_camel_case_types)]
        pub struct chord {}

        impl chord {
            pub const TAG_NAME: &'static str = "chord";
            pub const NAME_SPACE: Option<&'static str> = None;
        }

        #[allow(non_camel_case_types)]
        pub struct fret {}

        impl fret {
            pub const TAG_NAME: &'static str = "fret";
            pub const NAME_SPACE: Option<&'static str> = None;

            #[allow(non_upper_case_globals)]
            pub const index: AttributeDiscription = ("index", None, true);

            #[allow(non_upper_case_globals)]
            pub const string: AttributeDiscription = ("string", None, true);

            #[allow(non_upper_case_globals)]
            pub const is_muted: AttributeDiscription = ("is_muted", None, true);
        }

        #[allow(non_camel_case_types)]
        pub struct frets {}

        impl frets {
            pub const TAG_NAME: &'static str = "frets";
            pub const NAME_SPACE: Option<&'static str> = None;

            #[allow(non_upper_case_globals)]
            pub const index: AttributeDiscription = ("index", None, true);

            #[allow(non_upper_case_globals)]
            pub const from: AttributeDiscription = ("from", None, true);

            #[allow(non_upper_case_globals)]
            pub const to: AttributeDiscription = ("to", None, true);
        }
    }
}

#[component]
fn Text<'a>(
    cx: Scope<'a>,
    content: &'a str,
    font_family: &'a str,
    font_size: f64,
    onresize: EventHandler<'a, Rect>,
) -> Element<'a> {
    let mounted = use_mounted(cx);
    let size = use_size(cx, mounted);

    use_effect(cx, (&size.width(), &size.height()), |_| {
        onresize.call(size);
        async {}
    });

    render!(
        text { font_family: *font_family, font_size: *font_size, onmounted: move |event| mounted.onmounted(event), opacity: 0., content }
    )
}

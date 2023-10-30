mod element;
pub use element::Note;

mod font;
pub use font::Font;

mod staff;
pub use staff::{Staff, StaffProps};

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
        pub struct chord {}

        impl chord {
            pub const TAG_NAME: &'static str = "chord";
            pub const NAME_SPACE: Option<&'static str> = None;
        }
    }
}

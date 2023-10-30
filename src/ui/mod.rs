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
        fn into_value(self, bump: &'a bumpalo::Bump) -> dioxus::core::AttributeValue<'a> {
            dioxus::core::AttributeValue::Int(self as u8 as _)
        }
    }

    pub mod dioxus_elements {
        pub use dioxus::prelude::dioxus_elements::*;

        pub type AttributeDiscription = (&'static str, Option<&'static str>, bool);

        pub struct note {}

        impl note {
            pub const TAG_NAME: &'static str = "note";
            pub const NAME_SPACE: Option<&'static str> = None;

            pub const natural: AttributeDiscription = ("natural", None, true);
        }

        pub struct clef {}

        impl clef {
            pub const TAG_NAME: &'static str = "clef";
            pub const NAME_SPACE: Option<&'static str> = None;

            pub const kind: AttributeDiscription = ("kind", None, true);
        }

        pub struct chord {}

        impl chord {
            pub const TAG_NAME: &'static str = "chord";
            pub const NAME_SPACE: Option<&'static str> = None;
        }
    }
}

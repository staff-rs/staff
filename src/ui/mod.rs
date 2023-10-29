pub mod prelude {
    pub use dioxus::prelude::*;

    pub mod dioxus_elements {
        pub use dioxus::prelude::dioxus_elements::*;

        pub type AttributeDiscription = (&'static str, Option<&'static str>, bool);

        pub struct note {}

        impl note {
            pub const TAG_NAME: &'static str = "note";
            pub const NAME_SPACE: Option<&'static str> = None;

            pub const natural: AttributeDiscription = ("natural", None, true);
        }

        pub struct chord {}

        impl chord {
            pub const TAG_NAME: &'static str = "chord";
            pub const NAME_SPACE: Option<&'static str> = None;
        }
    }
}

mod staff;
pub use staff::{Staff, StaffProps};

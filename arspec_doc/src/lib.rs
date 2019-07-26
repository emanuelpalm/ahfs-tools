pub mod html;
pub mod svg;

mod font;

pub use self::font::{Font, FontStyle, FontWeight, GlyphIndex};

macro_rules! load_manifest_file {
    ($path:tt) => (include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), $path)))
}

pub mod scripts {
    pub const MAIN: &'static [u8] = load_manifest_file!("/assets/scripts/main.js");
}

pub mod styles {
    pub const PRINT: &'static [u8] = load_manifest_file!("/assets/styles/print.css");
    pub const SCREEN: &'static [u8] = load_manifest_file!("/assets/styles/screen.css");
}

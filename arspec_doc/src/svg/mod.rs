pub mod record;
pub mod service;

use crate::Font;

/// Represents some type that can be encoded as an SVG image.
pub trait Element {
    /// Encodes this as SVG image and writes it to `target`.
    ///
    /// The given `measurement` must have been retrieved via an earlier call to
    /// `measure()` on the same object.
    fn encode(&self, measurement: (f32, f32), target: &mut String);

    /// Calculates bounds of this SVG image, in pixels.
    fn measure(&self) -> (f32, f32);
}

/// Creates complete XML/SVG file for this SVG element.
pub fn render<E>(element: &E) -> String
    where E: Element,
{
    let (width, height) = element.measure();

    // Render XML version specifier and outer SVG element.
    let mut target = format!(concat!(
        "<?xml version=\"1.0\" standalone=\"no\"?>\n",
        "<svg width=\"{0}px\" height=\"{1}px\" viewBox=\"0 0 {0} {1}\"",
        " xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">\n",
    ), width, height);

    // Render CSS style definitions.
    target.push_str("<defs><style type=\"text/css\">");
    for font in Font::all() {
        target.push_str(&format!(concat!(
            "@font-face{{",
            "font-family:\"{}\";",
            "font-style:{};",
            "font-weight:{};",
            "src:\"{}\" format('truetype');",
            "}}",
        ), font.name(), font.style(), font.weight(), font.source_name()));
    }
    target.push_str("</style></defs>\n");

    element.encode((width, height), &mut target);

    target.push_str("\n</svg>\n");
    target
}

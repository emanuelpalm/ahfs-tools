pub mod record;

use crate::Font;

pub trait Write {
    fn write(&self, width: f32, height: f32, target: &mut String);
}

pub struct Element<'a> {
    pub source: &'a dyn Write,
    pub width: f32,
    pub height: f32,
}

pub fn render<'a, E>(element: E) -> String
    where E: Into<Element<'a>>,
{
    let element = element.into();

    // Render XML version specifier and outer SVG element.
    let mut target = format!(concat!(
        "<?xml version=\"1.0\" standalone=\"no\"?>\n",
        "<svg width=\"{0}px\" height=\"{1}px\" viewBox=\"0 0 {0} {1}\"",
        " xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">\n",
    ), element.width, element.height);

    // Render definitions.
    target.push_str("<defs><style type=\"text/css\">");
    for font in Font::all() {
        target.push_str(&format!(concat!(
            "@font-face {{\n",
            " font-family: \"{}\";\n",
            " font-style: {};\n",
            " font-weight: {};\n",
            " src: \"{}\" format('truetype');\n",
            "}}\n",
        ), font.name(), font.style(), font.weight(), font.source_name()));
    }
    target.push_str("</style></defs>\n");

    element.source.write(element.width, element.height, &mut target);

    target.push_str("\n</svg>\n");
    target
}

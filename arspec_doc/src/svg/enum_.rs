use arspec::spec::Enum;
use crate::Font;
use std::io;
use super::{color, Encode, Vector};

impl<'a: 'b, 'b> Encode for &'b Enum<'a> {
    fn encode<W>(&self, offset: Vector, size: Vector, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let mut offset_y = offset.y + 78.0;
        write!(
            w,
            concat!(
                "<rect x=\"{x_rect0}\" y=\"{y_rect0}\" width=\"{width0}\" height=\"{height0}\"",
                " rx=\"9\" ry=\"9\" fill=\"{color_ruler}\" />",
                "<rect x=\"{x_rect1}\" y=\"{y_rect1}\" width=\"{width1}\" height=\"{height1}\"",
                " rx=\"7\" ry=\"7\" fill=\"#fff\" />",
                "<rect x=\"{x_rect2}\" y=\"{y_rect2}\" width=\"{width1}\" height=\"1\"",
                " fill=\"{color_ruler}\" />",
                "",
                "<g text-anchor=\"middle\">",
                "<text x=\"{x_middle}\" y=\"{y_meta}\" fill=\"{color_meta}\"",
                " font-size=\"15\">«enum»</text>",
                "<text x=\"{x_middle}\" y=\"{y_name}\" fill=\"{color_name}\" font-size=\"18\"",
                " font-weight=\"bold\" class=\"enum-name\">{name}</text>",
                "</g>",
                "",
                "<g fill=\"{color_name}\" font-size=\"16\" font-style=\"italic\">",
            ),
            color_meta = color::META,
            color_name = color::GAMMA,
            color_ruler = color::RULER,
            height0 = size.y,
            height1 = size.y - 6.0,
            name = self.name.as_str(),
            width0 = size.x,
            width1 = size.x - 6.0,
            x_middle = offset.x + size.x / 2.0,
            x_rect0 = offset.x,
            x_rect1 = offset.x + 3.0,
            x_rect2 = offset.x + 3.0,
            y_meta = offset.y + 24.0,
            y_name = offset.y + 43.0,
            y_rect0 = offset.y,
            y_rect1 = offset.y + 3.0,
            y_rect2 = offset.y + 53.0,
        )?;
        for variant in &self.variants {
            write!(
                w,
                "<text x=\"{}\" y=\"{}\" class=\"enum-variant\">{}</text>",
                offset.x + 10.0,
                offset_y as usize,
                variant.name.as_str(),
            )?;
            offset_y += Font::sans().line_height() * 16.0;
        }
        write!(w, "</g>")
    }

    fn measure(&self) -> Vector {
        Vector {
            x: {
                let variant_width_max = self.variants.iter()
                    .map(|variant| {
                        Font::sans_italic().line_width_of(variant.name.as_str()) * 16.0 * 1000.0
                    } as usize)
                    .max()
                    .unwrap_or(0) as f32 / 1000.0;

                let name_width = Font::sans_bold()
                    .line_width_of(self.name.as_str()) * 18.0;

                (variant_width_max.max(name_width) + 20.0).round()
            },
            y: (self.variants.len() as f32 * Font::sans_italic()
                .line_height() * 16.0 + 71.0).round(),
        }
    }
}

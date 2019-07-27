use arspec::spec::Record;
use crate::fonts;
use std::io;
use super::{color, Encode, Vector};

impl<'a: 'b, 'b> Encode for &'b Record<'a> {
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
                " font-size=\"15\">«record»</text>",
                "<text x=\"{x_middle}\" y=\"{y_name}\" fill=\"{color_name}\" font-size=\"18\"",
                " font-weight=\"bold\" class=\"record-name\">{name}</text>",
                "</g>",
                "",
                "<g fill=\"{color_text}\" font-size=\"16\">",
            ),
            color_meta = color::META,
            color_name = color::GAMMA,
            color_ruler = color::RULER,
            color_text = color::TEXT,
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
        let entry_height = fonts::SANS.line_height() * 16.0;
        for entry in &self.entries {
            write!(
                w,
                concat!(
                    "<text x=\"{}\" y=\"{}\">",
                    "<tspan class=\"record-field-name\">{}</tspan>",
                    "<tspan>: </tspan>",
                    "<tspan fill=\"{}\" font-weight=\"bold\" class=\"type-ref\">{}</tspan>",
                    "</text>",
                ),
                offset.x + 10.0,
                offset_y as usize,
                entry.name.as_str(),
                color::GAMMA,
                entry.type_ref.as_str()
                    .chars()
                    .fold(String::new(), |mut acc, ch| {
                        match ch {
                            '<' => acc.push_str("&lt;"),
                            '>' => acc.push_str("&gt;"),
                            _ => acc.push(ch),
                        }
                        acc
                    }),
            )?;
            offset_y += entry_height;
        }
        write!(w, "</g>")
    }

    fn measure(&self) -> Vector {
        Vector {
            x: {
                let colon_space_width = fonts::SANS.line_width_of(": ");
                let entry_width_max = self.entries.iter()
                    .map(|entry| {
                        let name_width = fonts::SANS
                            .line_width_of(entry.name.as_str());
                        let type_ref_width = fonts::SANS_BOLD
                            .line_width_of(entry.type_ref.as_str());

                        (name_width + colon_space_width + type_ref_width)
                            * 16.0 * 1000.0
                    } as usize)
                    .max()
                    .unwrap_or(0) as f32 / 1000.0;

                let name_width = fonts::SANS_BOLD
                    .line_width_of(self.name.as_str()) * 18.0;

                (entry_width_max.max(name_width) + 20.0).round()
            },
            y: (self.entries.len() as f32 * fonts::SANS
                .line_height() * 16.0 + 71.0).round(),
        }
    }
}

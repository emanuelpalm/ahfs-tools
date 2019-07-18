use arspec::spec::Record;
use crate::Font;
use std::io;
use super::{Encode, Size};

impl<'a> Encode for Record<'a> {
    fn encode<W>(&self, size: Size, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let mut offset = 78.0;
        write!(
            w,
            concat!(
                "<rect x=\"0\" y=\"0\" width=\"{width0}\" height=\"{height0}\"",
                " rx=\"9\" ry=\"9\" fill=\"#aaa\" />",
                "<rect x=\"3\" y=\"3\" width=\"{width1}\" height=\"{height1}\"",
                " rx=\"7\" ry=\"7\" fill=\"#fff\" />",
                "<rect x=\"3\" y=\"53\" width=\"{width1}\" height=\"1\" fill=\"#ccc\" />",
                "",
                "<g text-anchor=\"middle\">",
                "<text x=\"50%\" y=\"24\" fill=\"#444\" font-size=\"15\">«record»</text>",
                "<text x=\"50%\" y=\"43\" fill=\"#3E7EFF\" font-size=\"18\"",
                " font-weight=\"bold\">{name}</text>",
                "</g>",
                "",
                "<g fill=\"#333\" font-size=\"16\">",
            ),
            width0 = size.width,
            height0 = size.height,
            width1 = size.width - 6.0,
            height1 = size.height - 6.0,
            name = self.name.as_str(),
        )?;
        for entry in &self.entries {
            write!(
                w,
                concat!(
                    "<text x=\"10\" y=\"{}\">",
                    "<tspan font-style=\"italic\">{}</tspan>",
                    "<tspan>: </tspan>",
                    "<tspan fill=\"#170591\" font-weight=\"bold\">{}</tspan>",
                    "</text>",
                ),
                offset as usize,
                entry.name.as_str(),
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
            offset += Font::sans().line_height() * 16.0;
        }
        write!(w, "</g>")
    }

    fn measure(&self) -> Size {
        Size {
            width: {
                let colon_space_width = Font::sans().line_width_of(": ");
                let entry_width_max = self.entries.iter()
                    .map(|entry| {
                        let name_width = Font::sans_italic()
                            .line_width_of(entry.name.as_str());
                        let type_ref_width = Font::sans_bold()
                            .line_width_of(entry.type_ref.as_str());

                        (name_width + colon_space_width + type_ref_width)
                            * 16.0 * 1000.0
                    } as usize)
                    .max()
                    .unwrap_or(0) as f32 / 1000.0;

                let name_width = Font::sans_bold()
                    .line_width_of(self.name.as_str()) * 18.0;

                (entry_width_max.max(name_width) + 20.0).round()
            },
            height: (self.entries.len() as f32 * Font::sans()
                .line_height() * 16.0 + 71.0).round(),
        }
    }
}

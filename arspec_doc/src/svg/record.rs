use arspec::spec::Record;
use crate::Font;
use super::{Element, Write};

fn calculate_record_height(record: &Record) -> f32 {
    record.entries.len() as f32 * Font::sans().line_height() * 16.0 + 71.0
}

fn calculate_record_width(record: &Record) -> f32 {
    let colon_space_width = Font::sans().line_width_of(": ");
    let width = record.entries.iter()
        .map(|entry| (Font::sans_italic().line_width_of(entry.name.as_str())
            + colon_space_width
            + Font::sans_bold().line_width_of(entry.type_ref.as_str())) as usize)
        .max()
        .unwrap_or(0) * 16 + 20;
    width as f32
}

impl<'a> From<&'a Record<'a>> for Element<'a> {
    fn from(source: &'a Record<'a>) -> Self {
        Element {
            source,
            width: calculate_record_width(source),
            height: calculate_record_height(source),
        }
    }
}

impl<'a> Write for Record<'a> {
    fn write(&self, width: f32, height: f32, target: &mut String) {
        let mut offset = 78;
        target.push_str(&format!(
            concat!(
                "<rect x=\"0\" y=\"0\" width=\"{width0}\" height=\"{height0}\"",
                " rx=\"9\" ry=\"9\" fill=\"#aaa\" />",
                "<rect x=\"3\" y=\"3\" width=\"{width1}\" height=\"{height1}\"",
                " rx=\"7\" ry=\"7\" fill=\"#fff\" />",
                "<rect x=\"3\" y=\"53\" width=\"{width1}\" height=\"1\" fill=\"#ccc\" />",
                "",
                "<g font-family=\"Noto Sans\">",
                "",
                "<g text-anchor=\"middle\">",
                "<text x=\"50%\" y=\"24\" fill=\"#444\" font-size=\"15\">«record»</text>",
                "<text x=\"50%\" y=\"43\" fill=\"#3E7EFF\" font-size=\"18\" font-weight=\"bold\">{name}</text>",
                "</g>",
                "",
                "<g fill=\"#333\" font-size=\"16\">",
                "{entries}",
                "</g>",
                "",
                "</g>",
            ),
            width0 = width,
            height0 = height,
            width1 = width - 6.0,
            height1 = height - 6.0,
            name = self.name.as_str(),
            entries = self.entries.iter()
                .fold(String::new(), |mut acc, entry| {
                    let x = entry.type_ref.as_str()
                        .replace('<', "&lt;")
                        .replace('>', "&gt;");

                    acc.push_str(&format!(
                        concat!(
                            "<text x=\"10\" y=\"{}\">",
                            "<tspan font-style=\"italic\">{}</tspan>: ",
                            "<tspan fill=\"#170591\" font-weight=\"bold\">{}</tspan>",
                            "</text>",
                        ),
                        offset,
                        entry.name.as_str(),
                        x,
                    ));
                    offset += 20;
                    acc
                }),
        ));
    }
}
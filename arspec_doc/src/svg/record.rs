use arspec::spec::Record;
use crate::Font;
use super::Element;

impl<'a> Element for Record<'a> {
    fn encode(&self, measurement: (f32, f32), target: &mut String) {
        let (width, height) = measurement;
        let mut offset = 78.0;
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
                "<text x=\"50%\" y=\"43\" fill=\"#3E7EFF\" font-size=\"18\"",
                " font-weight=\"bold\">{name}</text>",
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
                    let escaped_type_ref_str = entry.type_ref.as_str()
                        .replace('<', "&lt;")
                        .replace('>', "&gt;");

                    acc.push_str(&format!(
                        concat!(
                            "<text x=\"10\" y=\"{}\">",
                            "<tspan font-style=\"italic\">{}</tspan>",
                            "<tspan>: </tspan>",
                            "<tspan fill=\"#170591\" font-weight=\"bold\">{}</tspan>",
                            "</text>",
                        ),
                        offset as usize,
                        entry.name.as_str(),
                        escaped_type_ref_str,
                    ));
                    offset += Font::sans().line_height() * 16.0;
                    acc
                }),
        ));
    }

    fn measure(&self) -> (f32, f32) {
        (
            // Width.
            {
                let colon_space = Font::sans().line_width_of(": ");
                let entry_width_max = self.entries.iter()
                    .map(|entry| {
                        let name = Font::sans_italic().line_width_of(entry.name.as_str());
                        let type_ref = Font::sans_bold().line_width_of(entry.type_ref.as_str());
                        (name + colon_space + type_ref) * 16.0
                    } as usize)
                    .max()
                    .unwrap_or(0);

                let name_width = Font::sans_bold().line_width_of(self.name.as_str()) * 18.0;

                (entry_width_max as f32).max(name_width) + 20.0
            },
            // Height.
            {
                (self.entries.len() as f32 * Font::sans().line_height() * 16.0 + 71.0).round()
            }
        )
    }
}

use crate::parser;

pub trait WriteSVG {
    fn write_svg(&self, target: &mut String);
    fn size(&self) -> (f32, f32);
}

impl<'a> WriteSVG for parser::Record<'a> {
    fn write_svg(&self, target: &mut String) {
        let (width, height) = self.size();
        let mut offset = 78;
        target.push_str(&format!(
            concat!(
                "<rect x=\"0\" y=\"0\" width=\"{width0}\" height=\"{height0}\"",
                " rx=\"9\" ry=\"9\" fill=\"#aaa\" />",
                "<rect x=\"3\" y=\"3\" width=\"{width1}\" height=\"{height1}\"",
                " rx=\"7\" ry=\"7\" fill=\"#fff\" />",
                "<rect x=\"3\" y=\"53\" width=\"{width1}\" height=\"1\" fill=\"#ccc\" />",
                "",
                "<g font-family=\"Cousine, Courier New, monospaced\">",
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

    fn size(&self) -> (f32, f32) {
        let width = {
            let width_entries = self.entries.iter()
                .map(|entry| ((entry.name.as_str().len() + entry.type_ref.as_str().len() + 2) as f64 * (8.8203125 + 1.2)) as usize)
                .max()
                .map(|len| len + 20)
                .unwrap_or(0);

            let width_name = (self.name.as_str().len() as f64 * (9.9228515625 + 1.2) + 20.0) as usize;

            width_name.max(width_entries).max(20)
        };
        let height = 71 + self.entries.len() * 20;

        (width as f32, height as f32)
    }
}

pub fn render(element: &dyn WriteSVG) -> String {
    let (width, height) = element.size();
    let mut target = format!(concat!(
        "<?xml version=\"1.0\" standalone=\"no\"?>\n",
        "<svg width=\"{0}px\" height=\"{1}px\" viewBox=\"0 0 {0} {1}\"",
        " xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">\n",
    ), width, height);
    element.write_svg(&mut target);
    target.push_str("\n</svg>\n");
    target
}

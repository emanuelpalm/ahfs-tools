use arspec::spec::Service;
use crate::Font;
use super::Element;

impl<'a> Element for Service<'a> {
    fn encode(&self, measurement: (f32, f32), target: &mut String) {
        let (width, height) = measurement;
        let mut offset: f32 = 53.0;
        target.push_str(&format!(
            concat!(
                "<rect x=\"0\" y=\"0\" width=\"{width0}\" height=\"{height0}\"",
                " rx=\"9\" ry=\"9\" fill=\"#aaa\" />",
                "<rect x=\"3\" y=\"3\" width=\"{width1}\" height=\"{height1}\"",
                " rx=\"7\" ry=\"7\" fill=\"#fff\" />",
                "",
                "<g font-family=\"Noto Sans\">",
                "",
                "<g text-anchor=\"middle\">",
                "<text x=\"50%\" y=\"24\" fill=\"#444\" font-size=\"15\">«service»</text>",
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
            entries = self.interfaces.iter()
                .fold(String::new(), |mut acc, interface| {
                    acc.push_str(&format!(
                        concat!(
                            "<rect x=\"3\" y=\"{}\" width=\"{}\" height=\"1\" fill=\"#ccc\" />",
                            "<text x=\"10\" y=\"{}\" font-size=\"14\">",
                            "<tspan>interface </tspan>",
                            "<tspan font-size=\"17\" fill=\"#3E7EFF\"",
                            " font-weight=\"bold\">{}</tspan>",
                            "<tspan>:</tspan>",
                            "</text>",
                        ),
                        offset.round(),
                        width - 6.0,
                        offset.round() + 25.0,
                        interface.name.as_str(),
                    ));
                    offset += Font::sans().line_height() * 17.0 + 25.0;
                    for method in &interface.methods {
                        let input = method.input.as_ref().map(|input| input.as_str());
                        let output = method.output.as_ref().map(|output| output.as_str());
                        acc.push_str(&format!(
                            concat!(
                                "<text x=\"20\" y=\"{}\">",
                                "<tspan font-style=\"italic\">{}</tspan>",
                                "<tspan>(</tspan>",
                                "<tspan fill=\"#170591\" font-weight=\"bold\">{}</tspan>",
                                "<tspan>){}</tspan>",
                                "<tspan fill=\"#170591\" font-weight=\"bold\">{}</tspan>",
                                "</text>",
                            ),
                            offset.round(),
                            method.name.as_str(),
                            input.unwrap_or(""),
                            if output.is_some() { ": " } else { "" },
                            output.unwrap_or(""),
                        ));
                        offset += Font::sans().line_height() * 16.0;
                    }
                    acc
                }),
        ));
    }

    fn measure(&self) -> (f32, f32) {
        (
            // Width.
            {
                let interface_width = Font::sans().line_width_of("interface: ") * 14.0;
                let interface_width_max = self.interfaces.iter()
                    .map(|interface| {
                        let name = interface_width + Font::sans_bold()
                            .line_width_of(interface.name.as_str()) * 17.0;
                        let method_max = interface.methods.iter()
                            .map(|method| {
                                let parens = Font::sans().line_width_of("()");
                                let name = Font::sans_italic().line_width_of(method.name.as_str());
                                let input = method.input
                                    .as_ref()
                                    .map(|input| Font::sans_bold().line_width_of(input.as_str()))
                                    .unwrap_or(0.0);
                                let output = method.output
                                    .as_ref()
                                    .map(|output| Font::sans().line_width_of(": ")
                                        + Font::sans_bold().line_width_of(output.as_str()))
                                    .unwrap_or(0.0);
                                ((parens + name + input + output) * 16.0 + 15.0) as usize
                            })
                            .max()
                            .unwrap_or(0) as f32;

                        name.max(method_max)
                    } as usize)
                    .max()
                    .unwrap_or(0);

                let name_width = Font::sans_bold().line_width_of(self.name.as_str()) * 18.0;

                (interface_width_max as f32).max(name_width) + 20.0
            },
            // Height.
            {
                let line_height = Font::sans().line_height();
                self.interfaces.iter().fold(53.0, |acc, interface| {
                    acc + line_height * 17.0 + interface.methods.len() as f32 * line_height * 16.0
                }) + self.interfaces.len() as f32 * 25.0
            }
        )
    }
}

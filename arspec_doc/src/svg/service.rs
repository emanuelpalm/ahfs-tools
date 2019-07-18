use arspec::spec::Service;
use crate::Font;
use std::io;
use super::{color, Encode, Size};

impl<'a> Encode for Service<'a> {
    fn encode<W>(&self, size: Size, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let mut offset: f32 = 53.0;
        write!(
            w,
            concat!(
                "<rect x=\"0\" y=\"0\" width=\"{width0}\" height=\"{height0}\"",
                " rx=\"9\" ry=\"9\" fill=\"{color_ruler}\" />",
                "<rect x=\"3\" y=\"3\" width=\"{width1}\" height=\"{height1}\"",
                " rx=\"7\" ry=\"7\" fill=\"#fff\" />",
                "",
                "<g text-anchor=\"middle\">",
                "<text x=\"50%\" y=\"24\" fill=\"{color_meta}\"",
                " font-size=\"15\">«service»</text>",
                "<text x=\"50%\" y=\"43\" fill=\"{color_beta}\" font-size=\"18\"",
                " font-weight=\"bold\">{name}</text>",
                "</g>",
                "",
                "<g fill=\"#333\" font-size=\"16\">",
            ),
            color_beta = color::BETA,
            color_meta = color::META,
            color_ruler = color::RULER,
            width0 = size.width,
            height0 = size.height,
            width1 = size.width - 6.0,
            height1 = size.height - 6.0,
            name = self.name.as_str(),
        )?;
        for interface in &self.interfaces {
            write!(
                w,
                concat!(
                    "<rect x=\"3\" y=\"{0}\" width=\"{1}\" height=\"1\"",
                    " fill=\"{2}\" />",
                    "<text x=\"10\" y=\"{3}\" font-size=\"14\">",
                    "<tspan fill=\"{4}\">interface </tspan>",
                    "<tspan font-size=\"17\" fill=\"{5}\"",
                    " font-style=\"italic\">{6}</tspan>",
                    "<tspan fill=\"{4}\">:</tspan>",
                    "</text>",
                ),
                offset.round(),
                size.width - 6.0,
                color::RULER,
                offset.round() + 25.0,
                color::META,
                color::BETA,
                interface.name.as_str(),
            )?;
            offset += Font::sans().line_height() * 17.0 + 25.0;
            for method in &interface.methods {
                let input = method.input.as_ref().map(|input| input.as_str());
                let output = method.output.as_ref().map(|output| output.as_str());
                write!(
                    w,
                    concat!(
                        "<text x=\"20\" y=\"{0}\">",
                        "<tspan>{1}</tspan>",
                        "<tspan fill=\"{2}\">(</tspan>",
                        "<tspan fill=\"{3}\" font-weight=\"bold\">{4}</tspan>",
                        "<tspan fill=\"{2}\">){5}</tspan>",
                        "<tspan fill=\"{3}\" font-weight=\"bold\">{6}</tspan>",
                        "</text>",
                    ),
                    offset.round(),
                    method.name.as_str(),
                    color::META,
                    color::GAMMA,
                    input.unwrap_or(""),
                    if output.is_some() { ": " } else { "" },
                    output.unwrap_or(""),
                )?;
                offset += Font::sans().line_height() * 16.0;
            }
        }
        write!(w, "</g>")
    }

    fn measure(&self) -> Size {
        Size {
            width: {
                let label_width = Font::sans()
                    .line_width_of("interface :") * 14.0;
                let interface_width_max = self.interfaces.iter()
                    .map(|interface| {
                        let name_width = label_width + Font::sans_italic()
                            .line_width_of(interface.name.as_str()) * 17.0;

                        let method_width_max = interface.methods.iter()
                            .map(|method| {
                                let mut width = 0.0;

                                width += Font::sans()
                                    .line_width_of(method.name.as_str());
                                width += Font::sans().line_width_of("(");
                                width += method.input.as_ref()
                                    .map(|input| Font::sans_bold()
                                        .line_width_of(input.as_str()))
                                    .unwrap_or(0.0);
                                width += Font::sans().line_width_of(")");
                                width += method.output.as_ref()
                                    .map(|output| Font::sans()
                                        .line_width_of(": ") + Font::sans_bold()
                                        .line_width_of(output.as_str()))
                                    .unwrap_or(0.0);

                                ((width * 16.0 + 15.0) * 1000.0) as usize
                            })
                            .max()
                            .unwrap_or(0) as f32 / 1000.0;

                        (name_width.max(method_width_max) * 1000.0) as usize
                    })
                    .max()
                    .unwrap_or(0) as f32 / 1000.0;

                let name_width = Font::sans_bold()
                    .line_width_of(self.name.as_str()) * 18.0;

                interface_width_max.max(name_width) + 20.0
            },
            height: {
                let line_height = Font::sans().line_height();
                self.interfaces.iter().fold(53.0, |acc, interface| {
                    acc + line_height * 17.0
                        + interface.methods.len() as f32 * line_height * 16.0
                        + 25.0
                })
            },
        }
    }
}

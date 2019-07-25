use arspec::spec::Service;
use crate::Font;
use std::io;
use super::{color, Encode, Vector};

impl<'a: 'b, 'b> Encode for &'b Service<'a> {
    fn encode<W>(&self, offset: Vector, size: Vector, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let mut offset_y = offset.y + 53.0;
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
                " font-size=\"15\">«service»</text>",
                "<text x=\"{x_middle}\" y=\"{y_name}\" fill=\"{color_name}\" font-size=\"18\"",
                " font-weight=\"bold\">{name}</text>",
                "</g>",
                "",
                "<g fill=\"#333\" font-size=\"16\">",
            ),
            color_meta = color::META,
            color_name = color::BETA,
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
        for interface in &self.interfaces {
            write!(
                w,
                concat!(
                    "<rect x=\"{x_rect}\" y=\"{y_rect}\" width=\"{width_rect}\" height=\"1\"",
                    " fill=\"{color_rect}\" />",
                    "<text x=\"{x_text}\" y=\"{y_text}\" font-size=\"14\">",
                    "<tspan fill=\"{color_meta}\">interface </tspan>",
                    "<tspan font-size=\"17\" fill=\"{color_name}\"",
                    " font-style=\"italic\">{name}</tspan>",
                    "<tspan fill=\"{color_meta}\">:</tspan>",
                    "</text>",
                ),
                color_meta = color::META,
                color_name = color::BETA,
                color_rect = color::RULER,
                name = interface.name.as_str(),
                width_rect = size.x - 6.0,
                x_text = offset.x + 10.0,
                x_rect = offset.x + 3.0,
                y_text = offset_y.round() + 25.0,
                y_rect = offset_y.round(),
            )?;
            offset_y += Font::sans().line_height() * 17.0 + 25.0;
            for method in &interface.methods {
                let input = method.input.as_ref().map(|input| input.as_str());
                let output = method.output.as_ref().map(|output| output.as_str());
                write!(
                    w,
                    concat!(
                        "<text x=\"{x_text}\" y=\"{y_text}\">",
                        "<tspan>{method}</tspan>",
                        "<tspan fill=\"{color_meta}\">(</tspan>",
                        "<tspan fill=\"{color_type_ref}\" font-weight=\"bold\">{input}</tspan>",
                        "<tspan fill=\"{color_meta}\">){colon}</tspan>",
                        "<tspan fill=\"{color_type_ref}\" font-weight=\"bold\">{output}</tspan>",
                        "</text>",
                    ),
                    colon = if output.is_some() { ": " } else { "" },
                    color_meta = color::META,
                    color_type_ref = color::GAMMA,
                    input = input.unwrap_or(""),
                    method = method.name.as_str(),
                    output = output.unwrap_or(""),
                    x_text = offset.x + 20.0,
                    y_text = offset_y.round(),
                )?;
                offset_y += Font::sans().line_height() * 16.0;
            }
        }
        write!(w, "</g>")
    }

    fn measure(&self) -> Vector {
        Vector {
            x: {
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
            y: {
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

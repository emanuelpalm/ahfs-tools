use arspec::spec::System;
use crate::Font;
use std::io;
use super::{color, Encode, Size};

impl<'a> Encode for System<'a> {
    fn encode<W>(&self, size: Size, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        write!(
            w,
            concat!(
                "<rect x=\"0\" y=\"0\" width=\"{width0}\" height=\"{height0}\"",
                " rx=\"9\" ry=\"9\" fill=\"{color_ruler}\" />",
                "<rect x=\"3\" y=\"3\" width=\"{width1}\" height=\"{height1}\"",
                " rx=\"7\" ry=\"7\" fill=\"#fff\" />",
                "",
                "<g text-anchor=\"middle\">",
                "<text x=\"50%\" y=\"24\" fill=\"{color_meta}\" font-size=\"15\">«system»</text>",
                "<text x=\"50%\" y=\"44\" fill=\"{color_alpha}\" font-size=\"18\"",
                " font-weight=\"bold\">{name}</text>",
                "</g>",
            ),
            color_alpha = color::ALPHA,
            color_meta = color::META,
            color_ruler = color::RULER,
            width0 = size.width,
            height0 = size.height,
            width1 = size.width - 6.0,
            height1 = size.height - 6.0,
            name = self.name.as_str(),
        )
    }

    fn measure(&self) -> Size {
        Size {
            width: Font::sans_bold()
                .line_width_of(self.name.as_str()) * 18.0 + 30.0,
            height: 60.0,
        }
    }
}

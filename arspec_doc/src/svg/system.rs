use arspec::spec::System;
use crate::Font;
use std::io;
use super::{color, Encode, Size, Vector};

const TYPE_REF_HEIGHT: f32 = 24.0;

pub struct SystemMeasurements {
    pub system: Vector,
    pub consumed_services: Vec<f32>,
    pub produced_services: Vec<f32>,
    pub produced_service_name_x: f32,
}

impl Size for SystemMeasurements {
    #[inline]
    fn size(&self) -> Vector {
        Vector {
            x: (self.system.x + self.consumed_services.iter().fold(0.0_f32, |a, s| a.max(*s)) + 36.0)
                .max(self.produced_service_name_x
                    + self.produced_services.iter().fold(0.0_f32, |a, s| a.max(*s)) + 16.0),
            y: self.system.y + self.produced_services.len() as f32 * TYPE_REF_HEIGHT + 16.0,
        }
    }
}

impl<'a: 'b, 'b> Encode<SystemMeasurements> for &'b System<'a> {
    fn encode<W>(&self, offset: Vector, measurements: SystemMeasurements, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        // Encode system.
        write!(
            w,
            concat!(
                "<rect x=\"{x_rect0}\" y=\"{y_rect0}\" width=\"{width0}\" height=\"{height0}\"",
                " rx=\"9\" ry=\"9\" fill=\"{color_ruler}\" />",
                "<rect x=\"{x_rect1}\" y=\"{y_rect1}\" width=\"{width1}\" height=\"{height1}\"",
                " rx=\"7\" ry=\"7\" fill=\"#fff\" />",
                "",
                "<g text-anchor=\"middle\">",
                "<text x=\"{x_middle}\" y=\"{y_meta}\" fill=\"{color_meta}\"",
                " font-size=\"15\">«system»</text>",
                "<text x=\"{x_middle}\" y=\"{y_name}\" fill=\"{color_name}\" font-size=\"18\"",
                " font-weight=\"bold\">{name}</text>",
                "</g>",
            ),
            color_meta = color::META,
            color_name = color::ALPHA,
            color_ruler = color::RULER,
            height0 = measurements.system.y,
            height1 = measurements.system.y - 6.0,
            name = self.name.as_str(),
            width0 = measurements.system.x,
            width1 = measurements.system.x - 6.0,
            x_middle = measurements.system.x / 2.0,
            x_rect0 = offset.x,
            x_rect1 = offset.x + 3.0,
            y_meta = offset.y + measurements.system.y / 2.0 - 6.0,
            y_name = offset.y + measurements.system.y / 2.0 + 13.0,
            y_rect0 = offset.y,
            y_rect1 = offset.y + 3.0,
        )?;

        // Encode consumed services.
        {
            let x0 = offset.x + measurements.system.x;
            let mut y = offset.y + 16.0;
            for service in &self.consumes {
                write!(
                    w,
                    concat!(
                        "<path stroke=\"{color_line}\" stroke-width=\"2.3\" fill=\"none\"",
                        " d=\"M{x_origin} {y_origin} h18\" />",
                        "<rect x=\"{x_base}\" y=\"{y_base}\" width=\"12\" height=\"12\"",
                        " fill=\"{color_base}\" stroke=\"{color_line}\" stroke-width=\"2.3\" />",
                        "<path stroke=\"{color_line}\" stroke-width=\"2.3\" fill=\"none\"",
                        " d=\"M {x_grip},{y_grip} m 13,-8 c -16,-3 -16,20.3 0,17.5\" />",
                        "<text x=\"{x_name}\" y=\"{y_name}\" fill=\"{color_name}\" font-size=\"16\"",
                        " font-weight=\"bold\">{name}</text>",
                    ),
                    color_base = color::RULER,
                    color_line = color::META,
                    color_name = color::BETA,
                    name = service.name.as_str(),
                    x_base = x0 - 7.0,
                    x_grip = x0 + 18.0,
                    x_name = x0 + 36.0,
                    x_origin = x0,
                    y_base = y - 6.0,
                    y_grip = y,
                    y_name = y - Font::sans_bold().line_height() * 16.0 / 2.0
                        + Font::sans_bold().ascender() * 16.0,
                    y_origin = y,
                )?;
                y += 24.0;
            }
        }

        // Encode produced services.
        {
            let mut x = offset.x + measurements.produced_service_name_x + 4.0;
            let x0 = x;
            let mut y = offset.y + measurements.system.y;
            let y0 = y;
            for service in &self.produces {
                x -= 24.0;
                y += 24.0;
                write!(
                    w,
                    concat!(
                        "<path stroke=\"{color_line}\" stroke-width=\"2.3\" fill=\"none\"",
                        " d=\"M{x_origin} {y_origin} v{y_line0} h{x_line1}\" />",
                        "<rect x=\"{x_base}\" y=\"{y_base}\" width=\"12\" height=\"12\"",
                        " fill=\"{color_base}\" stroke=\"{color_line}\" stroke-width=\"2.3\" />",
                        "<circle cx=\"{cx_circle}\" cy=\"{cy_circle}\" r=\"6.5\" fill=\"{color_line}\" />",
                        "<text x=\"{x_name}\" y=\"{y_name}\" fill=\"{color_name}\" font-size=\"16\"",
                        " font-weight=\"bold\">{name}</text>",
                    ),
                    color_base = color::RULER,
                    color_line = color::META,
                    color_name = color::BETA,
                    cx_circle = x0,
                    cy_circle = y,
                    name = service.name.as_str(),
                    x_base = x - 6.0,
                    x_line1 = x0 - x,
                    x_name = x0 + 12.0,
                    x_origin = x,
                    y_base = y0 - 7.0,
                    y_line0 = y - y0,
                    y_name = y - Font::sans_bold().line_height() * 16.0 / 2.0
                        + Font::sans_bold().ascender() * 16.0,
                    y_origin = y0,
                )?;
            }
        }

        Ok(())
    }

    fn measure(&self) -> SystemMeasurements {
        SystemMeasurements {
            system: Vector {
                x: (Font::sans_bold().line_width_of(self.name.as_str()) * 18.0 + 60.0)
                    .max(self.produces.len() as f32 * 24.0 + 10.0),
                y: (self.consumes.len() as f32 * 24.0 + 6.0).max(90.0),
            },
            consumed_services: self.consumes.iter()
                .map(|s| Font::sans_bold().line_width_of(s.name.as_str()) * 16.0)
                .collect(),
            produced_services: self.produces.iter()
                .map(|service| Font::sans_bold().line_width_of(service.name.as_str()) * 16.0)
                .collect(),
            produced_service_name_x: self.produces.len() as f32 * 24.0 + 12.0,
        }
    }
}

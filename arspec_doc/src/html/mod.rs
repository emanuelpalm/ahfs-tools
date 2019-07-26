pub mod system;

use std::{fmt, io};
use std::path::Path;

pub trait Encode {
    fn encode<W>(&self, w: &mut W) -> io::Result<()>
        where W: io::Write;

    fn name(&self) -> &str;
}

/// Creates HTML documentation file for given `element`.
pub fn render<E, W>(
    element: &E,
    scripts: &[&'_ Path],
    styles: &[Style<'_>],
    w: &mut W,
) -> io::Result<()>
    where E: Encode,
          W: io::Write,
{
    write!(w, concat!(
        "<!DOCTYPE html>\n",
        "<html lang=\"en-US\">\n",
        "<head>\n",
        "  <meta charset=\"utf-8\">\n",
        "  <title>{name}</title>\n",
    ), name = element.name())?;

    for script in scripts {
        write!(w, "  <script src=\"{}\"></script>", script.display())?;
    }

    for style in styles {
        write!(w, "  {}\n", style)?;
    }

    write!(w, concat!(
        "</head>\n",
        "<body>\n",
    ))?;

    element.encode(w)?;

    write!(w, concat!(
        "</body>\n",
        "</html>",
    ))
}

pub struct Style<'a> {
    pub path: &'a Path,
    pub media: StyleMedia,
}

impl<'a> fmt::Display for Style<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\"{} />",
            self.path.display(),
            match self.media {
                StyleMedia::ALL => "",
                StyleMedia::PRINT => " media=\"print\"",
                StyleMedia::SCREEN => " media=\"screen\"",
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StyleMedia {
    ALL,
    PRINT,
    SCREEN,
}

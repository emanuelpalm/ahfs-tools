use crate::parser;
use crate::source::Span;

pub trait Element {
    fn render(&self, target: &mut String);
    fn size(&self) -> (f32, f32);
}


impl<'a> Element for parser::Record<'a> {
    fn render(&self, target: &mut String) {
        let mut offset = 0;
        target.push_str(&format!(
            concat!(
                "<g>",
                "<text x=\"50%\" y=\"5\" font-size=\"9\" text-anchor=\"middle\">«record»</text>",
                "<text x=\"5\" y=\"15\" font-size=\"12\">{}</text>",
                "{}",
                "</g>",
            ),
            self.name.as_str(),
            self.entries.iter()
                .fold(String::new(), |mut acc, entry| {
                    acc.push_str(&format!(
                        "<text x=\"5\" y=\"{}\" font-size=\"11\">{}: <tspan font-style=\"italic\">{}</tspan></text>",
                        (30 + (offset * 10)),
                        entry.name.as_str(),
                        entry.type_ref.as_str()
                            .replace('<', "&lt;")
                            .replace('>', "&gt;"),
                    ));
                    offset += 1;
                    acc
                }),
        ));
    }

    fn size(&self) -> (f32, f32) {
        (50.0, 50.0 + (self.entries.len() as f32) * 10.0)
    }
}

pub fn render(element: &dyn Element) -> String {
    let (width, height) = element.size();
    let mut target = format!(concat!(
        "<?xml version=\"1.0\" standalone=\"no\"?>\n",
        "<svg width=\"{0}mm\" height=\"{1}mm\" viewBox=\"0 0 {0} {1}\"",
        " xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">\n",
    ), width, height);
    element.render(&mut target);
    target.push_str("\n</svg>\n");
    target
}

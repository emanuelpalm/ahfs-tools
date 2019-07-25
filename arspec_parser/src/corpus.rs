use super::Text;
use std::io;
use std::path::Path;

/// A collection of related source code texts.
#[derive(Debug, Default)]
pub struct Corpus {
    pub texts: Vec<Text>,
}

impl Corpus {
    pub fn read_from<I, E>(iter: I) -> io::Result<Corpus>
        where I: IntoIterator<Item=E>,
              E: AsRef<Path>
    {
        let mut corpus = Corpus::default();
        for path in iter.into_iter() {
            corpus.texts.push(Text::read_at(path)?);
        }
        Ok(corpus)
    }
}

impl From<Text> for Corpus {
    #[inline]
    fn from(text: Text) -> Self {
        Corpus {
            texts: vec![text],
        }
    }
}
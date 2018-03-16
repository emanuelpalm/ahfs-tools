use ::pest::Span;

pub struct Lexeme<'a>(Span<'a>);

impl<'a> Lexeme<'a> {
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.0.as_str()
    }
}

impl<'a> From<Lexeme<'a>> for Span<'a> {
    #[inline]
    fn from(lexeme: Lexeme<'a>) -> Self {
        lexeme.0
    }
}

impl<'a> From<Lexeme<'a>> for &'a str {
    #[inline]
    fn from(lexeme: Lexeme<'a>) -> Self {
        lexeme.as_str()
    }
}

impl<'a> From<Span<'a>> for Lexeme<'a> {
    #[inline]
    fn from(span: Span<'a>) -> Self {
        Lexeme(span)
    }
}
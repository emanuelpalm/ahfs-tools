mod lexeme;
mod lexer;

pub use self::lexeme::Lexeme;
pub use self::lexer::Lexer;

// use std::result::Result;
// use super::Error;

// TODO: Implement analyze function.

//pub fn analyze<'a>(source: &'a str) -> Result<Vec<Lexeme<'a>>, Error<'a>> {
//    let mut lexemes = Vec::new();
//    Ok(lexemes)
//}


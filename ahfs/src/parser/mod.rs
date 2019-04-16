//! Arrowhead specification parsing utilities.
//!
//! This module contains tools useful for parsing specification source texts.

mod error;
mod lexer;
mod matcher;
mod name;
mod scanner;
mod token;
mod tree;

pub use self::error::Error;
pub use self::tree::Tree;

use self::matcher::Matcher;
use self::name::Name;
use self::scanner::Scanner;
use self::token::Token;
use source::Source;
use std::result;

/// The `Result` of parsing.
pub type Result<T> = result::Result<T, Error>;

/// Parses given source code texts.
pub fn parse(source: &Source) -> Result<Tree> {
    let tokens = lexer::analyze(source);
    let mut m = Matcher::new(&tokens);
    let mut t = Tree::new();

    while !m.at_end() {
        match_root(&mut m, &mut t)?;
    }

    Ok(t)
}

fn match_root<'a, 'b>(m: &mut Matcher<'a, 'b>, t: &mut Tree<'b>) -> Result<()>
    where 'b: 'a,
{
    let c = m.try_one(Name::Comment);
    let token = m.any(&[Name::Import, Name::System])?;
    match *token.name() {
        Name::Import => match_import(m, t),
        Name::System => match_system(m, t, c),
        _ => unreachable!(),
    }
}

fn match_import<'a, 'b>(m: &mut Matcher<'a, 'b>, t: &mut Tree<'b>) -> Result<()>
    where 'b: 'a,
{
    let head = m.all(&[Name::String, Name::Semicolon])?;
    let name = head[0].region();

    t.imports.push(tree::Import { name: name.clone() });

    Ok(())
}

fn match_system<'a, 'b: 'a>(
    m: &mut Matcher<'a, 'b>,
    t: &mut Tree<'b>,
    comment: Option<Token<'b>>,
) -> Result<()> {
    let head = m.all(&[Name::Identifier, Name::BraceLeft])?;
    let mut s = tree::System::new(head[0].region().clone());
    s.comment = comment.map(|c| c.region().clone());

    loop {
        let c = m.try_one(Name::Comment);
        let next = m.any(&[
            Name::Consumes,
            Name::Produces,
            Name::BraceRight,
        ])?;
        match *next.name() {
            Name::Consumes => match_system_service_ref(m, &mut s.consumes, c)?,
            Name::Produces => match_system_service_ref(m, &mut s.produces, c)?,
            Name::BraceRight => { break; }
            _ => unreachable!(),
        }
    }

    t.systems.push(s);

    Ok(())
}

fn match_system_service_ref<'a, 'b>(
    m: &mut Matcher<'a, 'b>,
    s: &mut Vec<tree::ServiceRef<'b>>,
    comment: Option<Token<'b>>,
) -> Result<()> {
    let head = m.all(&[Name::Identifier, Name::Semicolon])?;
    s.push(tree::ServiceRef {
        name: head[0].region().clone(),
        comment: comment.map(|c| c.region().clone()),
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::source::Source;
    use crate::source::Text;

    #[test]
    fn parse() {
        let texts = vec![
            Text::new("alpha.ahfs", concat!(
                "import \"test.ahfs\";\n",
                "\n",
                "// This comment is ignored.\n",
                "/* This one too! */\n",
                "/**\n",
                " * Comment A.\n",
                " * More comment A.\n",
                " */\n",
                "system TestSystem {\n",
                "    /// Comment B.\n",
                "    consumes TestServiceX;\n",
                "    consumes TestServiceY;\n",
                "\n",
                "    /** Comment C. */\n",
                "    produces TestServiceA;\n",
                "    produces TestServiceB;\n",
                "}\n",
            )),
        ];
        let source = Source::new(texts);
        match super::parse(&source) {
            Ok(tree) => panic!("{:?}", tree),
            Err(err) => {
                println!("{}", err);
                panic!("{:?}", err);
            }
        }
    }
}
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
use source::{Region, Source};
use std::result;

/// The `Result` of parsing.
pub type Result<T> = result::Result<T, Error>;

type M<'a, 'b> = Matcher<'a, 'b>;

/// Parses given source code texts.
pub fn parse(source: &Source) -> Result<Tree> {
    let tokens = lexer::analyze(source);
    let mut m = M::new(&tokens);
    let mut tree = Tree::new();

    while !m.at_end() {
        let comment = m.try_one(Name::Comment).map(|c| c.into_region());
        let token = m.any(&[
            Name::Import,
            Name::Service,
            Name::System,
        ])?;
        match *token.name() {
            Name::Import => tree.imports.push(import(&mut m)?),
            Name::Service => tree.services.push(service(&mut m, comment)?),
            Name::System => tree.systems.push(system(&mut m, comment)?),
            _ => unreachable!(),
        }
    }

    Ok(tree)
}

fn import<'a, 'b>(m: &mut M<'a, 'b>) -> Result<tree::Import<'b>> {
    let head = m.all(&[Name::String, Name::Semicolon])?;
    let name = head[0].region();
    Ok(tree::Import { name: name.clone() })
}

fn system<'a, 'b>(m: &mut M<'a, 'b>, comment: Option<Region<'b>>) -> Result<tree::System<'b>> {
    let head = m.all(&[Name::Identifier, Name::BraceLeft])?;
    let mut system = tree::System::new(head[0].region().clone(), comment);

    loop {
        let comment = m.try_one(Name::Comment).map(|c| c.into_region());
        let token = m.any(&[
            Name::Consumes,
            Name::Produces,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Consumes => system.consumes.push(service_ref(m, comment)?),
            Name::Produces => system.produces.push(service_ref(m, comment)?),
            Name::BraceRight => { break; }
            _ => unreachable!(),
        }
    }

    Ok(system)
}

fn service<'a, 'b>(m: &mut M<'a, 'b>, comment: Option<Region<'b>>) -> Result<tree::Service<'b>> {
    let head = m.all(&[Name::Identifier, Name::BraceLeft])?;
    let mut service = tree::Service::new(head[0].region().clone(), comment);

    loop {
        let comment = m.try_one(Name::Comment).map(|c| c.into_region());
        let token = m.any(&[
            Name::Interface,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Interface => service.interfaces.push(service_interface(m, comment)?),
            Name::BraceRight => { break; }
            _ => unreachable!(),
        }
    }

    Ok(service)
}

fn service_interface<'a, 'b>(m: &mut M<'a, 'b>, comment: Option<Region<'b>>) -> Result<tree::ServiceInterface<'b>> {
    let head = m.all(&[Name::Identifier, Name::Semicolon])?;
    let mut interface = tree::ServiceInterface::new(head[0].region().clone(), comment);

    loop {
        let comment = m.try_one(Name::Comment).map(|c| c.into_region());
        let token = m.any(&[
            Name::Method,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Method => interface.methods.push(service_method(m, comment)?),
            Name::BraceRight => { break; }
            _ => unreachable!(),
        }
    }

    Ok(interface)
}

fn service_method<'a, 'b>(m: &mut M<'a, 'b>, comment: Option<Region<'b>>) -> Result<tree::ServiceMethod<'b>> {
    let head = m.all(&[Name::Identifier, Name::ParenLeft])?;
    let mut method = tree::ServiceMethod::new(head[0].region().clone(), comment);

    method.input = try_type_ref(m, None);

    m.one(Name::ParenRight)?;

    if let Some(_) = m.try_one(Name::Comma) {
        method.output = try_type_ref(m, None);
    }

    m.one(Name::Semicolon)?;

    Ok(method)
}

fn service_ref<'a, 'b>(m: &mut M<'a, 'b>, comment: Option<Region<'b>>) -> Result<tree::ServiceRef<'b>> {
    let head = m.all(&[Name::Identifier, Name::Semicolon])?;
    Ok(tree::ServiceRef {
        name: head[0].region().clone(),
        comment,
    })
}

fn try_type_ref<'a, 'b>(m: &mut M<'a, 'b>, comment: Option<Region<'b>>) -> Option<tree::TypeRef<'b>> {
    let head = m.try_one(Name::Identifier)?;
    let mut type_ref = tree::TypeRef::new(head.region().clone());

    if let Some(_) = m.try_one(Name::AngleLeft) {
        let mut has_more = true;
        loop {
            match try_type_ref(m, None) {
                Some(t) => type_ref.params.push(t),
                None => break,
            }
            if !has_more {
                break;
            }
            if let None = m.try_one(Name::Comma) {
                has_more = false;
            }
            if let Some(_) = m.try_one(Name::AngleRight) {
                break;
            }
        }
    }

    Some(type_ref)
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
                "\n",
                "service TestServiceX {\n",
                "    interface X1 {\n",
                "        method FireMissiles();\n",
                "        method SetTarget(String);\n",
                "        method GetTarget(): String;\n",
                "    }\n",
                "}\n",
                "\n",
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
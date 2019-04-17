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
pub use self::tree::{Service, ServiceMethod, ServiceInterface, ServiceRef, System, Tree};

use self::matcher::Matcher;
use self::name::Name;
use self::scanner::Scanner;
use self::token::Token;
use source::{Region, Source};
use std::result;

/// The `Result` of parsing.
pub type Result<T> = result::Result<T, Error>;

type M<'a, 'b> = Matcher<'a, 'b>;
type R = Result<()>;
type C<'a> = Option<Region<'a>>;

/// Parses given source code texts.
pub fn parse(source: &Source) -> Result<Tree> {
    let tokens = lexer::analyze(source);

    let mut m = M::new(&tokens);
    let mut tree = Tree::new();

    inflate(&mut m, &mut tree, None)?;

    return Ok(tree);

    fn inflate<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Tree<'b>, c: C<'b>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Import,
            Name::Service,
            Name::System,
        ])?;
        match *token.name() {
            Name::Comment => inflate(m, t, Some(token.into_region()))?,
            Name::Import => import(m, t, c)?,
            Name::Service => service(m, t, c)?,
            Name::System => system(m, t, c)?,
            _ => unreachable!(),
        }
        if m.at_end() {
            return Ok(());
        }
        inflate(m, t, None)
    }
}

fn import<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Tree<'b>, c: C<'b>) -> R {
    let head = m.all(&[Name::String, Name::Semicolon])?;
    let name = head[0].region();
    t.imports.push(tree::Import { name: name.clone(), comment: c });
    Ok(())
}

fn system<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Tree<'b>, c: C<'b>) -> R {
    let head = m.all(&[Name::Identifier, Name::BraceLeft])?;
    let mut system = System::new(head[0].region().clone(), c);

    inflate(m, &mut system, None)?;
    t.systems.push(system);

    return Ok(());

    fn inflate<'a, 'b>(m: &mut M<'a, 'b>, t: &mut System<'b>, c: C<'b>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Consumes,
            Name::Produces,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => { return inflate(m, t, Some(token.into_region())); }
            Name::Consumes => service_ref(m, &mut t.consumes, c)?,
            Name::Produces => service_ref(m, &mut t.produces, c)?,
            Name::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        inflate(m, t, None)
    }
}

fn service<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Tree<'b>, c: C<'b>) -> R {
    let head = m.all(&[Name::Identifier, Name::BraceLeft])?;
    let mut service = Service::new(head[0].region().clone(), c);

    inflate(m, &mut service, None)?;
    t.services.push(service);

    return Ok(());

    fn inflate<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Service<'b>, c: C<'b>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Interface,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => inflate(m, t, Some(token.into_region()))?,
            Name::Interface => service_interface(m, t, c)?,
            Name::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        inflate(m, t, None)
    }
}

fn service_interface<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Service<'b>, c: C<'b>) -> R {
    let head = m.all(&[Name::Identifier, Name::BraceLeft])?;
    let mut interface = ServiceInterface::new(head[0].region().clone(), c);

    inflate(m, &mut interface, None);
    t.interfaces.push(interface);

    return Ok(());

    fn inflate<'a, 'b>(m: &mut M<'a, 'b>, s: &mut ServiceInterface<'b>, c: C<'b>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Method,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => inflate(m, s, Some(token.into_region()))?,
            Name::Method => service_method(m, &mut s.methods, c)?,
            Name::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        inflate(m, s, None)
    }
}

fn service_method<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Vec<ServiceMethod<'b>>, c: C<'b>) -> R {
    let head = m.all(&[Name::Identifier, Name::ParenLeft])?;
    let mut method = tree::ServiceMethod::new(head[0].region().clone(), c);

    method.input = try_type_ref(m, None);
    m.one(Name::ParenRight)?;
    if let Some(_) = m.try_one(Name::Colon) {
        method.output = try_type_ref(m, None);
    }
    m.one(Name::Semicolon)?;

    t.push(method);

    Ok(())
}

fn service_ref<'a, 'b>(m: &mut M<'a, 'b>, s: &mut Vec<ServiceRef<'b>>, c: C<'b>) -> R {
    let head = m.all(&[Name::Identifier, Name::Semicolon])?;
    s.push(ServiceRef {
        name: head[0].region().clone(),
        comment: c,
    });
    Ok(())
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
                "        method FireMissiles(Set<Plan>);\n",
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
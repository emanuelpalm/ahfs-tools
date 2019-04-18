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
pub use self::tree::{Service, ServiceMethod, ServiceInterface, ServiceRef, System, Tree, TypeRef};

use self::matcher::Matcher;
use self::name::Name;
use self::scanner::Scanner;
use self::token::Token;
use source::{Span, Source};
use std::result;

/// The `Result` of parsing.
pub type Result<T> = result::Result<T, Error>;

type M<'a, 'b> = Matcher<'a, 'b>;
type R = Result<()>;
type C<'a> = Option<Span<'a>>;

/// Parses given source code texts.
pub fn parse(source: &Source) -> Result<Tree> {
    let tokens = lexer::analyze(source);

    let mut m = M::new(&tokens);
    let mut tree = Tree::new();

    entry(&mut m, &mut tree, None)?;

    return Ok(tree);

    fn entry<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Tree<'b>, c: C<'b>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Import,
            Name::Service,
            Name::System,
        ])?;
        match *token.name() {
            Name::Comment => entry(m, t, Some(token.into_span()))?,
            Name::Import => import(m, t, c)?,
            Name::Service => service(m, t, c)?,
            Name::System => system(m, t, c)?,
            _ => unreachable!(),
        }
        if m.at_end() {
            return Ok(());
        }
        entry(m, t, None)
    }
}

fn import<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Tree<'b>, c: C<'b>) -> R {
    let head = m.all(&[Name::String, Name::Semicolon])?;
    let name = head[0].span();
    t.imports.push(tree::Import { name: name.clone(), comment: c });
    Ok(())
}

fn system<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Tree<'b>, c: C<'b>) -> R {
    let head = m.all(&[Name::Identifier, Name::BraceLeft])?;
    let mut system = System::new(head[0].span().clone(), c);

    entry(m, &mut system, None)?;
    t.systems.push(system);

    return Ok(());

    fn entry<'a, 'b>(m: &mut M<'a, 'b>, t: &mut System<'b>, c: C<'b>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Consumes,
            Name::Produces,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => { return entry(m, t, Some(token.into_span())); }
            Name::Consumes => service_ref(m, &mut t.consumes, c)?,
            Name::Produces => service_ref(m, &mut t.produces, c)?,
            Name::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, t, None)
    }
}

fn service<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Tree<'b>, c: C<'b>) -> R {
    let head = m.all(&[Name::Identifier, Name::BraceLeft])?;
    let mut service = Service::new(head[0].span().clone(), c);

    entry(m, &mut service, None)?;
    t.services.push(service);

    return Ok(());

    fn entry<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Service<'b>, c: C<'b>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Interface,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => entry(m, t, Some(token.into_span()))?,
            Name::Interface => service_interface(m, t, c)?,
            Name::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, t, None)
    }
}

fn service_interface<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Service<'b>, c: C<'b>) -> R {
    let head = m.all(&[Name::Identifier, Name::BraceLeft])?;
    let mut interface = ServiceInterface::new(head[0].span().clone(), c);

    entry(m, &mut interface, None);
    t.interfaces.push(interface);

    return Ok(());

    fn entry<'a, 'b>(m: &mut M<'a, 'b>, s: &mut ServiceInterface<'b>, c: C<'b>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Method,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => entry(m, s, Some(token.into_span()))?,
            Name::Method => service_method(m, &mut s.methods, c)?,
            Name::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, s, None)
    }
}

fn service_method<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Vec<ServiceMethod<'b>>, c: C<'b>) -> R {
    let head = m.all(&[Name::Identifier, Name::ParenLeft])?;
    let mut method = ServiceMethod::new(head[0].span().clone(), c);

    type_ref(m, &mut method.input, None)?;
    m.one(Name::ParenRight)?;
    if let Some(_) = m.try_one(Name::Colon) {
        type_ref(m, &mut method.output, None)?;
    }
    m.one(Name::Semicolon)?;

    t.push(method);

    Ok(())
}

fn service_ref<'a, 'b>(m: &mut M<'a, 'b>, s: &mut Vec<ServiceRef<'b>>, c: C<'b>) -> R {
    let head = m.all(&[Name::Identifier, Name::Semicolon])?;
    s.push(ServiceRef {
        name: head[0].span().clone(),
        comment: c,
    });
    Ok(())
}

fn type_ref<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Option<TypeRef<'b>>, c: C<'b>) -> R {
    let name = match m.try_one(Name::Identifier) {
        Some(token) => token,
        None => { return Ok(()); },
    };
    let mut type_ref = TypeRef::new(name.span().clone());

    params(m, &mut type_ref.params)?;
    *t = Some(type_ref);

    return Ok(());

    fn params<'a, 'b>(m: &mut M<'a, 'b>, t: &mut Vec<TypeRef<'b>>) -> R {
        if let None = m.try_one(Name::AngleLeft) {
            return Ok(());
        }

        match m.one(Name::AngleRight) {
            Ok(_) => Ok(()),
            Err(mut error) => {
                error.push_expected(&[Name::Identifier, Name::Comma]);
                Err(error)
            }
        }
    }
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
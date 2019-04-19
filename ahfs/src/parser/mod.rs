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
pub use self::tree::{
    Implement, ImplementInterface, ImplementMethod,
    Property,
    Record,
    Service, ServiceMethod, ServiceInterface, ServiceRef, System,
    Tree, TypeRef,
    Value,
};

use self::matcher::Matcher;
use self::name::Name;
use self::scanner::Scanner;
use self::token::Token;
use source::{Span, Source};
use std::result;

/// The `Result` of parsing.
pub type Result<T> = result::Result<T, Error>;

type M<'a> = Matcher<'a>;
type R = Result<()>;
type C<'a> = Option<Span<'a>>;

/// Parses given source code texts.
pub fn parse(source: &Source) -> Result<Tree> {
    let mut m = M::new(lexer::analyze(source));
    let mut tree = Tree::new();

    entry(&mut m, &mut tree, None)?;

    return Ok(tree);

    fn entry<'a>(m: &mut M<'a>, t: &mut Tree<'a>, c: C<'a>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Implement,
            Name::Import,
            Name::Service,
            Name::System,
        ])?;
        match *token.name() {
            Name::Comment => entry(m, t, Some(token.into_span()))?,
            Name::Implement => implement(m, t, c)?,
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

fn implement<'a>(m: &mut M<'a>, t: &mut Tree<'a>, c: C<'a>) -> R {
    let mut implement = {
        let tokens = m.all(&[
            Name::Identifier,
            Name::Using,
            Name::Identifier,
            Name::Slash,
            Name::Identifier,
            Name::BraceLeft,
        ])?;
        Implement::new(
            tokens[0].span().clone(),
            tokens[2].span().clone(),
            tokens[4].span().clone(),
            c,
        )
    };

    entry(m, &mut implement, None)?;
    t.implements.push(implement);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut Implement<'a>, c: C<'a>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Interface,
            Name::Property,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => { return entry(m, t, Some(token.into_span())); }
            Name::Interface => implement_interface(m, t, c)?,
            Name::Property => property(m, &mut t.properties, c)?,
            Name::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, t, None)
    }
}

fn implement_interface<'a>(m: &mut M<'a>, t: &mut Implement<'a>, c: C<'a>) -> R {
    let mut interface = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| ImplementInterface::new(tokens[0].span().clone(), c))?;

    entry(m, &mut interface, None)?;
    t.interfaces.push(interface);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, s: &mut ImplementInterface<'a>, c: C<'a>) -> R {
        let token = m.any(&[
            Name::Comment,
            Name::Method,
            Name::Property,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => entry(m, s, Some(token.into_span()))?,
            Name::Method => implement_method(m, &mut s.methods, c)?,
            Name::Property => property(m, &mut s.properties, c)?,
            Name::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, s, None)
    }
}

fn implement_method<'a>(m: &mut M<'a>, t: &mut Vec<ImplementMethod<'a>>, c: C<'a>) -> R {
    let mut method = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| ImplementMethod::new(tokens[0].span().clone(), c))?;

    map(m, &mut method.data)?;
    t.push(method);

    Ok(())
}

fn import<'a>(m: &mut M<'a>, t: &mut Tree<'a>, c: C<'a>) -> R {
    let import = m
        .all(&[Name::String, Name::Semicolon])
        .map(|tokens| tree::Import { name: tokens[0].span().clone(), comment: c })?;

    t.imports.push(import);

    Ok(())
}

fn list<'a>(m: &mut M<'a>, t: &mut Vec<Value<'a>>) -> R {
    if let Some(_) = m.opt_one(Name::SquareRight) {
        return Ok(());
    }

    t.push(value(m)?);

    let token = m.any(&[
        Name::Comma,
        Name::SquareRight,
    ])?;
    match *token.name() {
        Name::Comma => list(m, t),
        Name::SquareRight => Ok(()),
        _ => unreachable!(),
    }
}

fn map<'a>(m: &mut M<'a>, t: &mut Vec<(Span<'a>, Value<'a>)>) -> R {
    let key = {
        let token = m.any(&[
            Name::Identifier,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Identifier => token.span().clone(),
            Name::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
    };

    m.one(Name::Colon)?;

    t.push((key, value(m)?));

    let token = m.any(&[
        Name::Comma,
        Name::BraceRight,
    ])?;
    match *token.name() {
        Name::Comma => map(m, t),
        Name::BraceRight => Ok(()),
        _ => unreachable!(),
    }
}

fn property<'a>(m: &mut M<'a>, t: &mut Vec<Property<'a>>, c: C<'a>) -> R {
    let name = m
        .all(&[Name::Identifier, Name::Colon])
        .map(|tokens| tokens[0].span().clone())?;

    let value = value(m)?;

    m.one(Name::Semicolon)?;

    t.push(Property {
        name,
        value,
        comment: c,
    });

    Ok(())
}

fn record<'a>(m: &mut M<'a>, t: &mut Vec<Record<'a>>, c: C<'a>) -> R {
    Ok(())
}

fn system<'a>(m: &mut M<'a>, t: &mut Tree<'a>, c: C<'a>) -> R {
    let mut system = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| System::new(tokens[0].span().clone(), c))?;

    entry(m, &mut system, None)?;
    t.systems.push(system);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut System<'a>, c: C<'a>) -> R {
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

fn service<'a>(m: &mut M<'a>, t: &mut Tree<'a>, c: C<'a>) -> R {
    let mut service = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| Service::new(tokens[0].span().clone(), c))?;

    entry(m, &mut service, None)?;
    t.services.push(service);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut Service<'a>, c: C<'a>) -> R {
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

fn service_interface<'a>(m: &mut M<'a>, t: &mut Service<'a>, c: C<'a>) -> R {
    let mut interface = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| ServiceInterface::new(tokens[0].span().clone(), c))?;

    entry(m, &mut interface, None)?;
    t.interfaces.push(interface);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, s: &mut ServiceInterface<'a>, c: C<'a>) -> R {
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

fn service_method<'a>(m: &mut M<'a>, t: &mut Vec<ServiceMethod<'a>>, c: C<'a>) -> R {
    let mut method = m
        .all(&[Name::Identifier, Name::ParenLeft])
        .map(|tokens| ServiceMethod::new(tokens[0].span().clone(), c))?;

    let token = m.any(&[Name::Identifier, Name::ParenRight])?;
    match *token.name() {
        Name::Identifier => {
            let mut type_ref = TypeRef::new(token.into_span());
            type_params(m, &mut type_ref.params)?;

            if let Err(mut error) = m.one(Name::ParenRight) {
                if type_ref.params.len() == 0 {
                    error.push_expected(&[Name::AngleLeft]);
                }
                return Err(error);
            }

            method.input = Some(type_ref);
        }
        Name::ParenRight => {}
        _ => unreachable!(),
    }

    let token = m.any(&[Name::Colon, Name::Semicolon])?;
    match *token.name() {
        Name::Colon => {
            let token = m.one(Name::Identifier)?;

            let mut type_ref = TypeRef::new(token.into_span());
            type_params(m, &mut type_ref.params)?;

            if let Err(mut error) = m.one(Name::Semicolon) {
                if type_ref.params.len() == 0 {
                    error.push_expected(&[Name::AngleLeft]);
                }
                return Err(error);
            }

            method.output = Some(type_ref);
        }
        Name::Semicolon => {}
        _ => unreachable!(),
    }

    t.push(method);

    Ok(())
}

fn service_ref<'a>(m: &mut M<'a>, s: &mut Vec<ServiceRef<'a>>, c: C<'a>) -> R {
    let service_ref = m
        .all(&[Name::Identifier, Name::Semicolon])
        .map(|tokens| ServiceRef { name: tokens[0].span().clone(), comment: c })?;

    s.push(service_ref);

    Ok(())
}

fn type_params<'a>(m: &mut M<'a>, t: &mut Vec<TypeRef<'a>>) -> R {
    if let None = m.opt_one(Name::AngleLeft) {
        return Ok(());
    }

    entry(m, t)?;

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut Vec<TypeRef<'a>>) -> R {
        let token = m.one(Name::Identifier)?;
        let mut type_ref = TypeRef::new(token.into_span());

        type_params(m, &mut type_ref.params)?;

        let token = m.any(&[
            Name::Comma,
            Name::AngleRight,
        ])?;
        match *token.name() {
            Name::Comma => entry(m, t)?,
            Name::AngleRight => {}
            _ => unreachable!(),
        };

        t.push(type_ref);

        Ok(())
    }
}

fn value<'a>(m: &mut M<'a>) -> Result<Value<'a>> {
    let token = m.any(&[
        Name::Boolean,
        Name::Integer,
        Name::Float,
        Name::String,
        Name::SquareLeft,
        Name::BraceLeft,
    ])?;
    Ok(match *token.name() {
        Name::Boolean => Value::Boolean(token.into_span()),
        Name::Integer => Value::Integer(token.into_span()),
        Name::Float => Value::Float(token.into_span()),
        Name::String => Value::String(token.into_span()),
        Name::SquareLeft => {
            let mut entries = Vec::new();
            list(m, &mut entries)?;
            Value::List(entries.into_boxed_slice())
        }
        Name::BraceLeft => {
            let mut entries = Vec::new();
            map(m, &mut entries)?;
            Value::Map(entries.into_boxed_slice())
        }
        _ => unreachable!(),
    })
}

#[cfg(test)]
mod tests {
    use crate::source::Source;

    #[test]
    fn parse() {
        let source = Source::new("alpha.ahfs", concat!(
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
            "        method FireMissiles(Set<Target>);\n",
            "        method SetTarget(Target);\n",
            "        method GetTarget(): Target;\n",
            "    }\n",
            "}\n",
            "\n",
            "implement TestServiceX using HTTP/JSON {\n",
            "    interface X1 {\n",
            "        property BasePath: \"/x\";\n",
            "\n",
            "        method FireMissiles {\n",
            "            Method: \"POST\",\n",
            "            Path: \"/missile-launches\",\n",
            "         }\n",
            "    }\n",
            "}\n",
            "\n",
        ));
        match super::parse(&source) {
            Ok(tree) => panic!("{:?}", tree),
            Err(err) => {
                println!("{}", err);
                panic!("{:?}", err);
            }
        }
    }
}
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
    Record, RecordEntry,
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

/// Parses given source code texts.
pub fn parse(source: &Source) -> Result<Tree> {
    let mut matcher = Matcher::new(lexer::analyze(source));
    let mut tree = Tree::new();

    entry(&mut matcher, &mut tree, None)?;

    return Ok(tree);

    fn entry<'a>(m: &mut Matcher<'a>, t: &mut Tree<'a>, c: Option<Span<'a>>) -> Result<()> {
        let token = m.any(&[
            Name::Comment,
            Name::Implement,
            Name::Record,
            Name::Service,
            Name::System,
        ])?;
        match *token.name() {
            Name::Comment => entry(m, t, Some(token.into_span()))?,
            Name::Implement => implement(m, t, c)?,
            Name::Record => record(m, t, c)?,
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

fn implement<'a>(m: &mut Matcher<'a>, t: &mut Tree<'a>, c: Option<Span<'a>>) -> Result<()> {
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

    fn entry<'a>(m: &mut Matcher<'a>, t: &mut Implement<'a>, c: Option<Span<'a>>) -> Result<()> {
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

fn implement_interface<'a>(m: &mut Matcher<'a>, t: &mut Implement<'a>, c: Option<Span<'a>>) -> Result<()> {
    let mut interface = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| ImplementInterface::new(tokens[0].span().clone(), c))?;

    entry(m, &mut interface, None)?;
    t.interfaces.push(interface);

    return Ok(());

    fn entry<'a>(m: &mut Matcher<'a>, s: &mut ImplementInterface<'a>, c: Option<Span<'a>>) -> Result<()> {
        let token = m.any(&[
            Name::Comment,
            Name::Method,
            Name::Property,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => { return entry(m, s, Some(token.into_span())); }
            Name::Method => implement_method(m, &mut s.methods, c)?,
            Name::Property => property(m, &mut s.properties, c)?,
            Name::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, s, None)
    }
}

fn implement_method<'a>(m: &mut Matcher<'a>, t: &mut Vec<ImplementMethod<'a>>, c: Option<Span<'a>>) -> Result<()> {
    let mut method = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| ImplementMethod::new(tokens[0].span().clone(), c))?;

    map(m, &mut method.data)?;
    t.push(method);

    Ok(())
}

fn list<'a>(m: &mut Matcher<'a>, t: &mut Vec<Value<'a>>) -> Result<()> {
    if let Some(_) = m.one_optional(Name::SquareRight) {
        return Ok(());
    }

    t.push(value(m).map_err(|mut error| {
        error.push_expected(&[Name::SquareRight]);
        error
    })?);

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

fn map<'a>(m: &mut Matcher<'a>, t: &mut Vec<(Span<'a>, Value<'a>)>) -> Result<()> {
    let key = {
        let token = m.any(&[
            Name::Identifier,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Identifier => token.into_span(),
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

fn property<'a>(m: &mut Matcher<'a>, t: &mut Vec<Property<'a>>, c: Option<Span<'a>>) -> Result<()> {
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

fn record<'a>(m: &mut Matcher<'a>, t: &mut Tree<'a>, c: Option<Span<'a>>) -> Result<()> {
    let name = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| tokens[0].span().clone())?;

    let mut record = Record::new(name, c);

    entry(m, &mut record, None)?;
    t.records.push(record);

    return Ok(());

    fn entry<'a>(m: &mut Matcher<'a>, t: &mut Record<'a>, c: Option<Span<'a>>) -> Result<()> {
        let name = {
            let token = m.any(&[
                Name::Comment,
                Name::Identifier,
                Name::BraceRight,
            ])?;
            match *token.name() {
                Name::Comment => { return entry(m, t, Some(token.into_span())); }
                Name::Identifier => token.into_span(),
                Name::BraceRight => { return Ok(()); }
                _ => unreachable!(),
            }
        };

        let type_ref = {
            let mut type_ref = m
                .all(&[Name::Colon, Name::Identifier])
                .map(|tokens| TypeRef::new(tokens[1].span().clone()))?;

            type_params(m, &mut type_ref.params)?;

            type_ref
        };

        t.entries.push(RecordEntry { name, type_ref, comment: c });

        let token = m.any(&[
            Name::Comma,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comma => entry(m, t, None),
            Name::BraceRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn system<'a>(m: &mut Matcher<'a>, t: &mut Tree<'a>, c: Option<Span<'a>>) -> Result<()> {
    let mut system = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| System::new(tokens[0].span().clone(), c))?;

    entry(m, &mut system, None)?;
    t.systems.push(system);

    return Ok(());

    fn entry<'a>(m: &mut Matcher<'a>, t: &mut System<'a>, c: Option<Span<'a>>) -> Result<()> {
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

fn service<'a>(m: &mut Matcher<'a>, t: &mut Tree<'a>, c: Option<Span<'a>>) -> Result<()> {
    let mut service = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| Service::new(tokens[0].span().clone(), c))?;

    entry(m, &mut service, None)?;
    t.services.push(service);

    return Ok(());

    fn entry<'a>(m: &mut Matcher<'a>, t: &mut Service<'a>, c: Option<Span<'a>>) -> Result<()> {
        let token = m.any(&[
            Name::Comment,
            Name::Interface,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => entry(m, t, Some(token.into_span())),
            Name::Interface => {
                service_interface(m, t, c)?;
                entry(m, t, None)
            }
            Name::BraceRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn service_interface<'a>(m: &mut Matcher<'a>, t: &mut Service<'a>, c: Option<Span<'a>>) -> Result<()> {
    let mut interface = m
        .all(&[Name::Identifier, Name::BraceLeft])
        .map(|tokens| ServiceInterface::new(tokens[0].span().clone(), c))?;

    entry(m, &mut interface, None)?;
    t.interfaces.push(interface);

    return Ok(());

    fn entry<'a>(m: &mut Matcher<'a>, s: &mut ServiceInterface<'a>, c: Option<Span<'a>>) -> Result<()> {
        let token = m.any(&[
            Name::Comment,
            Name::Method,
            Name::BraceRight,
        ])?;
        match *token.name() {
            Name::Comment => entry(m, s, Some(token.into_span())),
            Name::Method => {
                service_method(m, &mut s.methods, c)?;
                entry(m, s, None)
            }
            Name::BraceRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn service_method<'a>(m: &mut Matcher<'a>, t: &mut Vec<ServiceMethod<'a>>, c: Option<Span<'a>>) -> Result<()> {
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

fn service_ref<'a>(m: &mut Matcher<'a>, s: &mut Vec<ServiceRef<'a>>, c: Option<Span<'a>>) -> Result<()> {
    let service_ref = m
        .all(&[Name::Identifier, Name::Semicolon])
        .map(|tokens| ServiceRef { name: tokens[0].span().clone(), comment: c })?;

    s.push(service_ref);

    Ok(())
}

fn type_params<'a>(m: &mut Matcher<'a>, t: &mut Vec<TypeRef<'a>>) -> Result<()> {
    if let None = m.one_optional(Name::AngleLeft) {
        return Ok(());
    }
    return entry(m, t);

    fn entry<'a>(m: &mut Matcher<'a>, t: &mut Vec<TypeRef<'a>>) -> Result<()> {
        let mut type_ref = m
            .one(Name::Identifier)
            .map(|token| TypeRef::new(token.into_span()))?;

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

fn value<'a>(m: &mut Matcher<'a>) -> Result<Value<'a>> {
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
    fn example1() {
        let source = Source::new("alpha.ahfs", concat!(
            "/// Comment A.\n",
            "service MyService {\n",
            "    /// Comment B.\n",
            "    interface MyInterface {\n",
            "        /// Comment C.\n",
            "        method MyMethod(Argument): Result;\n",
            "    }\n",
            "}\n",
        ));
        let tree = match super::parse(&source) {
            Ok(tree) => tree,
            Err(err) => {
                println!("{}", err);
                panic!("{:?}", err);
            }
        };

        assert_eq!(tree.implements.len(), 0);
        assert_eq!(tree.records.len(), 0);
        assert_eq!(tree.services.len(), 1);
        assert_eq!(tree.systems.len(), 0);

        let service = &tree.services[0];
        assert_eq!(service.name.as_str(), "MyService");
        assert_eq!(service.interfaces.len(), 1);
        assert_eq!(service.comment.as_ref().unwrap().as_str(), "/// Comment A.");

        let interface = &service.interfaces[0];
        assert_eq!(interface.name.as_str(), "MyInterface");
        assert_eq!(interface.methods.len(), 1);
        assert_eq!(interface.comment.as_ref().unwrap().as_str(), "/// Comment B.");

        let method = &interface.methods[0];
        assert_eq!(method.name.as_str(), "MyMethod");
        assert_eq!(method.input.as_ref().unwrap().name.as_str(), "Argument");
        assert_eq!(method.output.as_ref().unwrap().name.as_str(), "Result");
        assert_eq!(method.comment.as_ref().unwrap().as_str(), "/// Comment C.");
    }

    #[test]
    fn example2() {
        let source = Source::new("alpha.ahfs", concat!(
            "// This comment is ignored.\n",
            "/* This one too! */\n",
            "/**\n",
            " * Comment A.\n",
            " * More comment A.\n",
            " */\n",
            "system TestSystem {\n",
            "    /// Comment B.\n",
            "    consumes TestServiceX;\n",
            "\n",
            "    /** Comment C. */\n",
            "    produces TestServiceA;\n",
            "}\n",
            "\n",
            "/// Comment D.\n",
            "service TestServiceX {\n",
            "    /// Comment E.\n",
            "    interface X1 {\n",
            "        /// Comment F.\n",
            "        method FireMissiles(Set<Target>);\n",
            "    }\n",
            "}\n",
            "\n",
            "/// Comment G.\n",
            "implement TestServiceX using HTTP/JSON {\n",
            "    /// Comment H.\n",
            "    interface X1 {\n",
            "        /// Comment I.\n",
            "        property BasePath: \"/x\";\n",
            "\n",
            "        /// Comment J.\n",
            "        method FireMissiles {\n",
            "            Method: \"POST\",\n",
            "            Path: \"/missile-launches\",\n",
            "         }\n",
            "    }\n",
            "}\n",
            "\n",
            "/// Comment K.\n",
            "record Target {\n",
            "    /// Comment L.\n",
            "    X: Integer,\n",
            "}\n",
        ));
        if let Err(error) = super::parse(&source) {
            panic!("{}", error);
        }
    }
}
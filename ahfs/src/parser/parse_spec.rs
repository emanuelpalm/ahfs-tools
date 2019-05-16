use ahfs_parse::{Matcher, Result, Span};
use crate::parser::token_kind::TokenKind;
use crate::parser::tree::{
    Implement, ImplementInterface, ImplementMethod,
    Property,
    Record, RecordEntry,
    Service, ServiceMethod, ServiceInterface, ServiceRef,
    System,
    Tree, TypeRef,
    Value,
};


type R<T> = Result<T, TokenKind>;
type M<'a> = Matcher<'a, TokenKind>;

/// Match all tokens in `m` that make up a valid AHF specification root.
pub fn root<'a>(mut m: &mut M<'a>) -> R<Tree<'a>> {
    let mut tree = Tree::default();

    entry(&mut m, &mut tree, None)?;

    return Ok(tree);

    fn entry<'a>(m: &mut M<'a>, t: &mut Tree<'a>, c: Option<Span<'a>>) -> R<()> {
        let token = m.any(&[
            TokenKind::Comment,
            TokenKind::Implement,
            TokenKind::Record,
            TokenKind::Service,
            TokenKind::System,
        ])?;
        match token.kind {
            TokenKind::Comment => entry(m, t, Some(token.span.clone()))?,
            TokenKind::Implement => implement(m, t, c)?,
            TokenKind::Record => record(m, t, c)?,
            TokenKind::Service => service(m, t, c)?,
            TokenKind::System => system(m, t, c)?,
            _ => unreachable!(),
        }
        if m.at_end() {
            return Ok(());
        }
        entry(m, t, None)
    }
}

fn implement<'a>(m: &mut M<'a>, t: &mut Tree<'a>, c: Option<Span<'a>>) -> R<()> {
    let mut implement = {
        let tokens = m.all(&[
            TokenKind::Identifier,
            TokenKind::Using,
            TokenKind::Identifier,
            TokenKind::Slash,
            TokenKind::Identifier,
            TokenKind::BraceLeft,
        ])?;
        Implement::new(
            tokens[0].span.clone(),
            tokens[2].span.clone(),
            tokens[4].span.clone(),
            c,
        )
    };

    entry(m, &mut implement, None)?;
    t.implements.push(implement);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut Implement<'a>, c: Option<Span<'a>>) -> R<()> {
        let token = m.any(&[
            TokenKind::Comment,
            TokenKind::Interface,
            TokenKind::Property,
            TokenKind::BraceRight,
        ])?;
        match token.kind {
            TokenKind::Comment => { return entry(m, t, Some(token.span.clone())); }
            TokenKind::Interface => implement_interface(m, t, c)?,
            TokenKind::Property => property(m, &mut t.properties, c)?,
            TokenKind::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, t, None)
    }
}

fn implement_interface<'a>(m: &mut M<'a>, t: &mut Implement<'a>, c: Option<Span<'a>>) -> R<()> {
    let mut interface = m
        .all(&[TokenKind::Identifier, TokenKind::BraceLeft])
        .map(|tokens| ImplementInterface::new(tokens[0].span.clone(), c))?;

    entry(m, &mut interface, None)?;
    t.interfaces.push(interface);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, s: &mut ImplementInterface<'a>, c: Option<Span<'a>>) -> R<()> {
        let token = m.any(&[
            TokenKind::Comment,
            TokenKind::Method,
            TokenKind::Property,
            TokenKind::BraceRight,
        ])?;
        match token.kind {
            TokenKind::Comment => { return entry(m, s, Some(token.span.clone())); }
            TokenKind::Method => implement_method(m, &mut s.methods, c)?,
            TokenKind::Property => property(m, &mut s.properties, c)?,
            TokenKind::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, s, None)
    }
}

fn implement_method<'a>(m: &mut M<'a>, t: &mut Vec<ImplementMethod<'a>>, c: Option<Span<'a>>) -> R<()> {
    let mut method = m
        .all(&[TokenKind::Identifier, TokenKind::BraceLeft])
        .map(|tokens| ImplementMethod::new(tokens[0].span.clone(), c))?;

    map(m, &mut method.data)?;
    t.push(method);

    Ok(())
}

fn list<'a>(m: &mut M<'a>, t: &mut Vec<Value<'a>>) -> R<()> {
    if let Some(_) = m.one_optional(TokenKind::SquareRight) {
        return Ok(());
    }

    t.push(value(m).map_err(|mut error| {
        error.expected.push(TokenKind::SquareRight);
        error
    })?);

    let token = m.any(&[
        TokenKind::Comma,
        TokenKind::SquareRight,
    ])?;
    match token.kind {
        TokenKind::Comma => list(m, t),
        TokenKind::SquareRight => Ok(()),
        _ => unreachable!(),
    }
}

fn map<'a>(m: &mut M<'a>, t: &mut Vec<(Span<'a>, Value<'a>)>) -> R<()> {
    let key = {
        let token = m.any(&[
            TokenKind::Identifier,
            TokenKind::BraceRight,
        ])?;
        match token.kind {
            TokenKind::Identifier => token.span.clone(),
            TokenKind::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
    };

    m.one(TokenKind::Colon)?;

    t.push((key, value(m)?));

    let token = m.any(&[
        TokenKind::Comma,
        TokenKind::BraceRight,
    ])?;
    match token.kind {
        TokenKind::Comma => map(m, t),
        TokenKind::BraceRight => Ok(()),
        _ => unreachable!(),
    }
}

fn property<'a>(m: &mut M<'a>, t: &mut Vec<Property<'a>>, c: Option<Span<'a>>) -> R<()> {
    let name = m
        .all(&[TokenKind::Identifier, TokenKind::Colon])
        .map(|tokens| tokens[0].span.clone())?;

    let value = value(m)?;

    m.one(TokenKind::Semicolon)?;

    t.push(Property {
        name,
        value,
        comment: c,
    });

    Ok(())
}

fn record<'a>(m: &mut M<'a>, t: &mut Tree<'a>, c: Option<Span<'a>>) -> R<()> {
    let name = m
        .all(&[TokenKind::Identifier, TokenKind::BraceLeft])
        .map(|tokens| tokens[0].span.clone())?;

    let mut record = Record::new(name, c);

    entry(m, &mut record, None)?;
    t.records.push(record);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut Record<'a>, c: Option<Span<'a>>) -> R<()> {
        let name = {
            let token = m.any(&[
                TokenKind::Comment,
                TokenKind::Identifier,
                TokenKind::BraceRight,
            ])?;
            match token.kind {
                TokenKind::Comment => { return entry(m, t, Some(token.span.clone())); }
                TokenKind::Identifier => token.span.clone(),
                TokenKind::BraceRight => { return Ok(()); }
                _ => unreachable!(),
            }
        };

        let type_ref = {
            let mut type_ref = m
                .all(&[TokenKind::Colon, TokenKind::Identifier])
                .map(|tokens| TypeRef::new(tokens[1].span.clone()))?;

            type_params(m, &mut type_ref.params)?;

            type_ref
        };

        t.entries.push(RecordEntry { name, type_ref, comment: c });

        let token = m.any(&[
            TokenKind::Comma,
            TokenKind::BraceRight,
        ])?;
        match token.kind {
            TokenKind::Comma => entry(m, t, None),
            TokenKind::BraceRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn system<'a>(m: &mut M<'a>, t: &mut Tree<'a>, c: Option<Span<'a>>) -> R<()> {
    let mut system = m
        .all(&[TokenKind::Identifier, TokenKind::BraceLeft])
        .map(|tokens| System::new(tokens[0].span.clone(), c))?;

    entry(m, &mut system, None)?;
    t.systems.push(system);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut System<'a>, c: Option<Span<'a>>) -> R<()> {
        let token = m.any(&[
            TokenKind::Comment,
            TokenKind::Consumes,
            TokenKind::Produces,
            TokenKind::BraceRight,
        ])?;
        match token.kind {
            TokenKind::Comment => { return entry(m, t, Some(token.span.clone())); }
            TokenKind::Consumes => service_ref(m, &mut t.consumes, c)?,
            TokenKind::Produces => service_ref(m, &mut t.produces, c)?,
            TokenKind::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, t, None)
    }
}

fn service<'a>(m: &mut M<'a>, t: &mut Tree<'a>, c: Option<Span<'a>>) -> R<()> {
    let mut service = m
        .all(&[TokenKind::Identifier, TokenKind::BraceLeft])
        .map(|tokens| Service::new(tokens[0].span.clone(), c))?;

    entry(m, &mut service, None)?;
    t.services.push(service);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut Service<'a>, c: Option<Span<'a>>) -> R<()> {
        let token = m.any(&[
            TokenKind::Comment,
            TokenKind::Interface,
            TokenKind::BraceRight,
        ])?;
        match token.kind {
            TokenKind::Comment => entry(m, t, Some(token.span.clone())),
            TokenKind::Interface => {
                service_interface(m, t, c)?;
                entry(m, t, None)
            }
            TokenKind::BraceRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn service_interface<'a>(m: &mut M<'a>, t: &mut Service<'a>, c: Option<Span<'a>>) -> R<()> {
    let mut interface = m
        .all(&[TokenKind::Identifier, TokenKind::BraceLeft])
        .map(|tokens| ServiceInterface::new(tokens[0].span.clone(), c))?;

    entry(m, &mut interface, None)?;
    t.interfaces.push(interface);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, s: &mut ServiceInterface<'a>, c: Option<Span<'a>>) -> R<()> {
        let token = m.any(&[
            TokenKind::Comment,
            TokenKind::Method,
            TokenKind::BraceRight,
        ])?;
        match token.kind {
            TokenKind::Comment => entry(m, s, Some(token.span.clone())),
            TokenKind::Method => {
                service_method(m, &mut s.methods, c)?;
                entry(m, s, None)
            }
            TokenKind::BraceRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn service_method<'a>(m: &mut M<'a>, t: &mut Vec<ServiceMethod<'a>>, c: Option<Span<'a>>) -> R<()> {
    let mut method = m
        .all(&[TokenKind::Identifier, TokenKind::ParenLeft])
        .map(|tokens| ServiceMethod::new(tokens[0].span.clone(), c))?;

    let token = m.any(&[TokenKind::Identifier, TokenKind::ParenRight])?;
    match token.kind {
        TokenKind::Identifier => {
            let mut type_ref = TypeRef::new(token.span.clone());
            type_params(m, &mut type_ref.params)?;

            if let Err(mut error) = m.one(TokenKind::ParenRight) {
                if type_ref.params.len() == 0 {
                    error.expected.push(TokenKind::AngleLeft);
                }
                return Err(error);
            }

            method.input = Some(type_ref);
        }
        TokenKind::ParenRight => {}
        _ => unreachable!(),
    }

    let token = m.any(&[TokenKind::Colon, TokenKind::Semicolon])?;
    match token.kind {
        TokenKind::Colon => {
            let token = m.one(TokenKind::Identifier)?;

            let mut type_ref = TypeRef::new(token.span.clone());
            type_params(m, &mut type_ref.params)?;

            if let Err(mut error) = m.one(TokenKind::Semicolon) {
                if type_ref.params.len() == 0 {
                    error.expected.push(TokenKind::AngleLeft);
                }
                return Err(error);
            }

            method.output = Some(type_ref);
        }
        TokenKind::Semicolon => {}
        _ => unreachable!(),
    }

    t.push(method);

    Ok(())
}

fn service_ref<'a>(m: &mut M<'a>, s: &mut Vec<ServiceRef<'a>>, c: Option<Span<'a>>) -> R<()> {
    let service_ref = m
        .all(&[TokenKind::Identifier, TokenKind::Semicolon])
        .map(|tokens| ServiceRef { name: tokens[0].span.clone(), comment: c })?;

    s.push(service_ref);

    Ok(())
}

fn type_params<'a>(m: &mut M<'a>, t: &mut Vec<TypeRef<'a>>) -> R<()> {
    if let None = m.one_optional(TokenKind::AngleLeft) {
        return Ok(());
    }
    return entry(m, t);

    fn entry<'a>(m: &mut M<'a>, t: &mut Vec<TypeRef<'a>>) -> R<()> {
        let mut type_ref = m
            .one(TokenKind::Identifier)
            .map(|token| TypeRef::new(token.span.clone()))?;

        type_params(m, &mut type_ref.params)?;

        let token = m.any(&[
            TokenKind::Comma,
            TokenKind::AngleRight,
        ])?;
        match token.kind {
            TokenKind::Comma => entry(m, t)?,
            TokenKind::AngleRight => {}
            _ => unreachable!(),
        };

        t.push(type_ref);

        Ok(())
    }
}

fn value<'a>(m: &mut M<'a>) -> R<Value<'a>> {
    let token = m.any(&[
        TokenKind::Null,
        TokenKind::Boolean,
        TokenKind::Integer,
        TokenKind::Float,
        TokenKind::String,
        TokenKind::SquareLeft,
        TokenKind::BraceLeft,
    ])?;
    Ok(match token.kind {
        TokenKind::Null => Value::Null,
        TokenKind::Boolean => Value::Boolean(token.span.clone()),
        TokenKind::Integer => Value::Integer(token.span.clone()),
        TokenKind::Float => Value::Float(token.span.clone()),
        TokenKind::String => Value::String(token.span.clone()),
        TokenKind::SquareLeft => {
            let mut entries = Vec::new();
            list(m, &mut entries)?;
            Value::List(entries.into_boxed_slice())
        }
        TokenKind::BraceLeft => {
            let mut entries = Vec::new();
            map(m, &mut entries)?;
            Value::Map(entries.into_boxed_slice())
        }
        _ => unreachable!(),
    })
}
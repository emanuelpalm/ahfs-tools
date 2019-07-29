use arspec_parser::{Error, Matcher, Span};
use crate::spec::{
    Attribute,
    Enum, EnumVariant,
    Implement, ImplementInterface, ImplementMethod,
    Primitive,
    Property,
    Record, RecordEntry,
    Service, ServiceMethod, ServiceInterface, ServiceRef,
    Specification,
    System,
    TypeRef,
    Value,
};
use super::Class;

type R<T> = Result<T, Error<Class>>;
type M<'a> = Matcher<'a, Class>;

/// Attempt to consume all tokens in `m` and produce a [`Specification`][spc].
///
/// [spc]: ../struct.Specification.html
pub fn root<'a>(spec: &mut Specification<'a>, mut m: &mut M<'a>) -> R<()> {
    return entry(&mut m, spec, vec![]);

    fn entry<'a>(m: &mut M<'a>, t: &mut Specification<'a>, a: Vec<Attribute<'a>>) -> R<()> {
        let token = m.any(&[
            Class::At,
            Class::Enum,
            Class::Implement,
            Class::Primitive,
            Class::Record,
            Class::Service,
            Class::System,
        ])?;
        match token.class {
            Class::At => {
                let a = attribute(m, a)?;
                entry(m, t, a)?
            }
            Class::Enum => enum_(m, t, a)?,
            Class::Implement => implement(m, t, a)?,
            Class::Primitive => primitive(m, t, a)?,
            Class::Record => record(m, t, a)?,
            Class::Service => service(m, t, a)?,
            Class::System => system(m, t, a)?,
            _ => unreachable!(),
        }
        if m.at_end() {
            return Ok(());
        }
        entry(m, t, vec![])
    }
}

fn attribute<'a>(m: &mut M<'a>, mut a: Vec<Attribute<'a>>) -> R<Vec<Attribute<'a>>> {
    let name = m
        .all(&[Class::Identifier, Class::ParenLeft])
        .map(|tokens| tokens[0].span.clone())?;

    let value = value(m)?;

    a.push(Attribute { name, value });

    m.one(Class::ParenRight)?;

    Ok(a)
}

fn enum_<'a>(m: &mut M<'a>, t: &mut Specification<'a>, a: Vec<Attribute<'a>>) -> R<()> {
    let name = m
        .all(&[Class::Identifier, Class::BraceLeft])
        .map(|tokens| tokens[0].span.clone())?;

    let mut enum_ = Enum::new(name, a);

    entry(m, &mut enum_, vec![])?;
    t.enums.push(enum_);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut Enum<'a>, a: Vec<Attribute<'a>>) -> R<()> {
        let name = {
            let token = m.any(&[
                Class::At,
                Class::Identifier,
                Class::BraceRight,
            ])?;
            match token.class {
                Class::At => {
                    let a = attribute(m, a)?;
                    return entry(m, t, a);
                }
                Class::Identifier => token.span.clone(),
                Class::BraceRight => { return Ok(()); }
                _ => unreachable!(),
            }
        };

        t.variants.push(EnumVariant { name, attributes: a });

        let token = m.any(&[
            Class::Comma,
            Class::BraceRight,
        ])?;
        match token.class {
            Class::Comma => entry(m, t, vec![]),
            Class::BraceRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn implement<'a>(m: &mut M<'a>, t: &mut Specification<'a>, a: Vec<Attribute<'a>>) -> R<()> {
    let mut implement = {
        let tokens = m.all(&[
            Class::Identifier,
            Class::Using,
            Class::Identifier,
            Class::Slash,
            Class::Identifier,
            Class::BraceLeft,
        ])?;
        Implement::new(
            tokens[0].span.clone(),
            tokens[2].span.clone(),
            tokens[4].span.clone(),
            a,
        )
    };

    entry(m, &mut implement, vec![])?;
    t.implementations.push(implement);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut Implement<'a>, a: Vec<Attribute<'a>>) -> R<()> {
        let token = m.any(&[
            Class::At,
            Class::Interface,
            Class::Property,
            Class::BraceRight,
        ])?;
        match token.class {
            Class::At => {
                let a = attribute(m, a)?;
                return entry(m, t, a);
            }
            Class::Interface => implement_interface(m, t, a)?,
            Class::Property => property(m, &mut t.properties, a)?,
            Class::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, t, vec![])
    }
}

fn implement_interface<'a>(m: &mut M<'a>, t: &mut Implement<'a>, a: Vec<Attribute<'a>>) -> R<()> {
    let mut interface = m
        .all(&[Class::Identifier, Class::BraceLeft])
        .map(|tokens| ImplementInterface::new(tokens[0].span.clone(), a))?;

    entry(m, &mut interface, vec![])?;
    t.interfaces.push(interface);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, s: &mut ImplementInterface<'a>, a: Vec<Attribute<'a>>) -> R<()> {
        let token = m.any(&[
            Class::At,
            Class::Method,
            Class::Property,
            Class::BraceRight,
        ])?;
        match token.class {
            Class::At => {
                let a = attribute(m, a)?;
                return entry(m, s, a);
            }
            Class::Method => implement_method(m, &mut s.methods, a)?,
            Class::Property => property(m, &mut s.properties, a)?,
            Class::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, s, vec![])
    }
}

fn primitive<'a>(m: &mut M<'a>, t: &mut Specification<'a>, a: Vec<Attribute<'a>>) -> R<()> {
    let mut generic_parameters = Vec::new();

    let token = m.any(&[
        Class::AngleLeft,
        Class::Identifier,
    ])?;
    let token = match token.class {
        Class::AngleLeft => {
            parameters(m, &mut generic_parameters)?;
            m.one(Class::Identifier)?
        }
        Class::Identifier => token,
        _ => unreachable!(),
    };
    let mut type_ref = TypeRef::new(token.span.clone());
    type_params(m, &mut type_ref.params)?;

    t.primitives.push(Primitive {
        generic_parameters,
        definition: type_ref,
        attributes: a,
    });

    return m.one(Class::Semicolon)
        .map(|_token| ());

    fn parameters<'a>(m: &mut M<'a>, t: &mut Vec<Span<'a>>) -> R<()> {
        let token = m.any(&[
            Class::Identifier,
            Class::AngleRight,
        ])?;
        match token.class {
            Class::Identifier => {
                t.push(token.span.clone());
                let token = m.any(&[
                    Class::Comma,
                    Class::AngleRight,
                ])?;
                match token.class {
                    Class::Comma => parameters(m, t),
                    Class::AngleRight => Ok(()),
                    _ => unreachable!(),
                }
            }
            Class::AngleRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn implement_method<'a>(m: &mut M<'a>, t: &mut Vec<ImplementMethod<'a>>, a: Vec<Attribute<'a>>) -> R<()> {
    let mut method = m
        .all(&[Class::Identifier, Class::BraceLeft])
        .map(|tokens| ImplementMethod::new(tokens[0].span.clone(), a))?;

    map(m, &mut method.data)?;
    t.push(method);

    Ok(())
}

fn list<'a>(m: &mut M<'a>, t: &mut Vec<Value<'a>>) -> R<()> {
    if let Some(_) = m.one_optional(Class::SquareRight) {
        return Ok(());
    }

    t.push(value(m).map_err(|mut error| {
        error.expected.push(Class::SquareRight);
        error
    })?);

    let token = m.any(&[
        Class::Comma,
        Class::SquareRight,
    ])?;
    match token.class {
        Class::Comma => list(m, t),
        Class::SquareRight => Ok(()),
        _ => unreachable!(),
    }
}

fn map<'a>(m: &mut M<'a>, t: &mut Vec<(Span<'a>, Value<'a>)>) -> R<()> {
    let key = {
        let token = m.any(&[
            Class::Identifier,
            Class::BraceRight,
        ])?;
        match token.class {
            Class::Identifier => token.span.clone(),
            Class::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
    };

    m.one(Class::Colon)?;

    t.push((key, value(m)?));

    let token = m.any(&[
        Class::Comma,
        Class::BraceRight,
    ])?;
    match token.class {
        Class::Comma => map(m, t),
        Class::BraceRight => Ok(()),
        _ => unreachable!(),
    }
}

fn property<'a>(m: &mut M<'a>, t: &mut Vec<Property<'a>>, a: Vec<Attribute<'a>>) -> R<()> {
    let name = m
        .all(&[Class::Identifier, Class::Colon])
        .map(|tokens| tokens[0].span.clone())?;

    let value = value(m)?;

    m.one(Class::Semicolon)?;

    t.push(Property {
        name,
        value,
        attributes: a,
    });

    Ok(())
}

fn record<'a>(m: &mut M<'a>, t: &mut Specification<'a>, a: Vec<Attribute<'a>>) -> R<()> {
    let name = m
        .all(&[Class::Identifier, Class::BraceLeft])
        .map(|tokens| tokens[0].span.clone())?;

    let mut record = Record::new(name, a);

    entry(m, &mut record, vec![])?;
    t.records.push(record);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut Record<'a>, a: Vec<Attribute<'a>>) -> R<()> {
        let name = {
            let token = m.any(&[
                Class::At,
                Class::Identifier,
                Class::BraceRight,
            ])?;
            match token.class {
                Class::At => {
                    let a = attribute(m, a)?;
                    return entry(m, t, a);
                }
                Class::Identifier => token.span.clone(),
                Class::BraceRight => { return Ok(()); }
                _ => unreachable!(),
            }
        };

        let type_ref = {
            let mut type_ref = m
                .all(&[Class::Colon, Class::Identifier])
                .map(|tokens| TypeRef::new(tokens[1].span.clone()))?;

            type_params(m, &mut type_ref.params)?;

            type_ref
        };

        t.entries.push(RecordEntry { name, type_ref, attributes: a });

        let token = m.any(&[
            Class::Comma,
            Class::BraceRight,
        ])?;
        match token.class {
            Class::Comma => entry(m, t, vec![]),
            Class::BraceRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn system<'a>(m: &mut M<'a>, t: &mut Specification<'a>, a: Vec<Attribute<'a>>) -> R<()> {
    let mut system = m
        .all(&[Class::Identifier, Class::BraceLeft])
        .map(|tokens| System::new(tokens[0].span.clone(), a))?;

    entry(m, &mut system, vec![])?;
    t.systems.push(system);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut System<'a>, a: Vec<Attribute<'a>>) -> R<()> {
        let token = m.any(&[
            Class::At,
            Class::Consumes,
            Class::Produces,
            Class::BraceRight,
        ])?;
        match token.class {
            Class::At => {
                let a = attribute(m, a)?;
                return entry(m, t, a);
            }
            Class::Consumes => service_ref(m, &mut t.consumes, a)?,
            Class::Produces => service_ref(m, &mut t.produces, a)?,
            Class::BraceRight => { return Ok(()); }
            _ => unreachable!(),
        }
        entry(m, t, vec![])
    }
}

fn service<'a>(m: &mut M<'a>, t: &mut Specification<'a>, a: Vec<Attribute<'a>>) -> R<()> {
    let mut service = m
        .all(&[Class::Identifier, Class::BraceLeft])
        .map(|tokens| Service::new(tokens[0].span.clone(), a))?;

    entry(m, &mut service, vec![])?;
    t.services.push(service);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, t: &mut Service<'a>, a: Vec<Attribute<'a>>) -> R<()> {
        let token = m.any(&[
            Class::At,
            Class::Interface,
            Class::BraceRight,
        ])?;
        match token.class {
            Class::At => {
                let a = attribute(m, a)?;
                return entry(m, t, a);
            }
            Class::Interface => {
                service_interface(m, t, a)?;
                entry(m, t, vec![])
            }
            Class::BraceRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn service_interface<'a>(m: &mut M<'a>, t: &mut Service<'a>, a: Vec<Attribute<'a>>) -> R<()> {
    let mut interface = m
        .all(&[Class::Identifier, Class::BraceLeft])
        .map(|tokens| ServiceInterface::new(tokens[0].span.clone(), a))?;

    entry(m, &mut interface, vec![])?;
    t.interfaces.push(interface);

    return Ok(());

    fn entry<'a>(m: &mut M<'a>, s: &mut ServiceInterface<'a>, a: Vec<Attribute<'a>>) -> R<()> {
        let token = m.any(&[
            Class::At,
            Class::Method,
            Class::BraceRight,
        ])?;
        match token.class {
            Class::At => {
                let a = attribute(m, a)?;
                return entry(m, s, a);
            }
            Class::Method => {
                service_method(m, &mut s.methods, a)?;
                entry(m, s, vec![])
            }
            Class::BraceRight => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn service_method<'a>(m: &mut M<'a>, t: &mut Vec<ServiceMethod<'a>>, a: Vec<Attribute<'a>>) -> R<()> {
    let mut method = m
        .all(&[Class::Identifier, Class::ParenLeft])
        .map(|tokens| ServiceMethod::new(tokens[0].span.clone(), a))?;

    let token = m.any(&[Class::Identifier, Class::ParenRight])?;
    match token.class {
        Class::Identifier => {
            let mut type_ref = TypeRef::new(token.span.clone());
            type_params(m, &mut type_ref.params)?;

            if let Err(mut error) = m.one(Class::ParenRight) {
                if type_ref.params.len() == 0 {
                    error.expected.push(Class::AngleLeft);
                }
                return Err(error);
            }

            method.input = Some(type_ref);
        }
        Class::ParenRight => {}
        _ => unreachable!(),
    }

    let token = m.any(&[Class::Colon, Class::Semicolon])?;
    match token.class {
        Class::Colon => {
            let token = m.one(Class::Identifier)?;

            let mut type_ref = TypeRef::new(token.span.clone());
            type_params(m, &mut type_ref.params)?;

            if let Err(mut error) = m.one(Class::Semicolon) {
                if type_ref.params.len() == 0 {
                    error.expected.push(Class::AngleLeft);
                }
                return Err(error);
            }

            method.output = Some(type_ref);
        }
        Class::Semicolon => {}
        _ => unreachable!(),
    }

    t.push(method);

    Ok(())
}

fn service_ref<'a>(m: &mut M<'a>, s: &mut Vec<ServiceRef<'a>>, a: Vec<Attribute<'a>>) -> R<()> {
    let service_ref = m
        .all(&[Class::Identifier, Class::Semicolon])
        .map(|tokens| ServiceRef { name: tokens[0].span.clone(), attributes: a })?;

    s.push(service_ref);

    Ok(())
}

fn type_params<'a>(m: &mut M<'a>, t: &mut Vec<TypeRef<'a>>) -> R<()> {
    if let None = m.one_optional(Class::AngleLeft) {
        return Ok(());
    }
    return entry(m, t);

    fn entry<'a>(m: &mut M<'a>, t: &mut Vec<TypeRef<'a>>) -> R<()> {
        let mut type_ref = m
            .one(Class::Identifier)
            .map(|token| TypeRef::new(token.span.clone()))?;

        type_params(m, &mut type_ref.params)?;

        let token = m.any(&[
            Class::Comma,
            Class::AngleRight,
        ])?;
        match token.class {
            Class::Comma => entry(m, t)?,
            Class::AngleRight => {}
            _ => unreachable!(),
        };

        t.push(type_ref);

        Ok(())
    }
}

fn value<'a>(m: &mut M<'a>) -> R<Value<'a>> {
    let token = m.any(&[
        Class::Null,
        Class::Boolean,
        Class::Integer,
        Class::Float,
        Class::String,
        Class::SquareLeft,
        Class::BraceLeft,
    ])?;
    Ok(match token.class {
        Class::Null => Value::Null,
        Class::Boolean => Value::Boolean(token.span.clone()),
        Class::Integer => Value::Integer(token.span.clone()),
        Class::Float => Value::Float(token.span.clone()),
        Class::String => Value::String(token.span.clone()),
        Class::SquareLeft => {
            let mut entries = Vec::new();
            list(m, &mut entries)?;
            Value::List(entries.into_boxed_slice())
        }
        Class::BraceLeft => {
            let mut entries = Vec::new();
            map(m, &mut entries)?;
            Value::Map(entries.into_boxed_slice())
        }
        _ => unreachable!(),
    })
}
use arspec_parser::Span;
use super::{Attribute, Property, Specification, Value, VerificationError};

/// Specifies how to implement a named [`Service`][srv].
///
/// [srv]: struct.Service.html
#[derive(Debug)]
pub struct Implement<'a> {
    /// Name of implemented service.
    pub name: Span<'a>,

    /// Name of communication protocol to use.
    pub protocol: Span<'a>,

    /// Name of payload encoding to use.
    pub encoding: Span<'a>,

    /// Any implementation properties.
    pub properties: Vec<Property<'a>>,

    /// Any interface implementation definitions.
    pub methods: Vec<ImplementMethod<'a>>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> Implement<'a> {
    /// Create new service implementation specification.
    #[inline]
    pub fn new(
        name: Span<'a>,
        protocol: Span<'a>,
        encoding: Span<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Self {
        Implement {
            name,
            protocol,
            encoding,
            properties: Vec::new(),
            methods: Vec::new(),
            attributes,
        }
    }

    pub fn verify(&self, spec: &Specification) -> Result<(), VerificationError> {
        let service = spec.services.iter()
            .find(|service| self.name == service.name)
            .ok_or_else(|| VerificationError::NoSuchServiceToImplement {
                service: self.name.to_excerpt(),
            })?;

        'outer0: for method0 in &self.methods {
            for method1 in &service.methods {
                if method0.name == method1.name {
                    continue 'outer0;
                }
            }
            return Err(VerificationError::NoSuchInterfaceToImplement {
                service: service.name.to_excerpt(),
                interface: method0.name.to_excerpt(),
            });
        }

        'outer1: for method0 in &service.methods {
            for method1 in &self.methods {
                if method0.name == method1.name {
                    continue 'outer1;
                }
            }
            return Err(VerificationError::InterfaceNotImplemented {
                interface: method0.name.to_excerpt(),
                implementation: self.name.to_excerpt(),
            });
        }

        match self.protocol.as_str() {
            "COAP" | "HTTP" | "MQTT" => Ok(()),
            _ => Err(VerificationError::UnknownServiceProtocol {
                protocol: self.protocol.to_excerpt(),
            })
        }?;

        match self.encoding.as_str() {
            "CBOR" | "JSON" | "XML" => Ok(()),
            _ => Err(VerificationError::UnknownServiceEncoding {
                encoding: self.encoding.to_excerpt(),
            })
        }
    }
}

/// Specifies how to implement a named [`ServiceMethod`][met].
///
/// [met]: struct.ServiceMethod.html
#[derive(Debug)]
pub struct ImplementMethod<'a> {
    /// Name of implemented [`ServiceMethod`](struct.ServiceMethod.html).
    pub name: Span<'a>,

    /// Table of method specification data.
    pub data: Vec<(Span<'a>, Value<'a>)>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> ImplementMethod<'a> {
    /// Create new [`ServiceMethod`][irf] implementation definition.
    ///
    /// [irf]: struct.ServiceMethod.html
    #[inline]
    pub fn new(name: Span<'a>, attributes: Vec<Attribute<'a>>) -> Self {
        ImplementMethod {
            name,
            data: Vec::new(),
            attributes,
        }
    }
}
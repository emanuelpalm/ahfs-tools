use std::fmt;

pub enum Predicate {
    Is,

    Consumes,
    Produces,

    Exposes,
    Accepts,
    Returns,
}

impl Predicate {
    pub fn parse(word: &str) -> Option<Predicate> {
        Some(match word {
            meta::IS => Predicate::Is,

            meta::CONSUMES => Predicate::Consumes,
            meta::PRODUCES => Predicate::Produces,

            meta::EXPOSES => Predicate::Exposes,
            meta::ACCEPTS => Predicate::Accepts,
            meta::RETURNS => Predicate::Returns,

            _ => { return None; }
        })
    }
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Predicate::Is => meta::IS,

            Predicate::Consumes => meta::CONSUMES,
            Predicate::Produces => meta::PRODUCES,

            Predicate::Exposes => meta::EXPOSES,
            Predicate::Accepts => meta::ACCEPTS,
            Predicate::Returns => meta::RETURNS,
        })
    }
}

mod meta {
    pub const IS: &'static str = "is";

    pub const CONSUMES: &'static str = "consumes";
    pub const PRODUCES: &'static str = "produces";

    pub const EXPOSES: &'static str = "exposes";
    pub const ACCEPTS: &'static str = "accepts";
    pub const RETURNS: &'static str = "returns";

    pub const PREDICATES: &'static [&'static str] = &[
        IS,
        CONSUMES,
        PRODUCES,
        EXPOSES,
        ACCEPTS,
        RETURNS,
    ];
}
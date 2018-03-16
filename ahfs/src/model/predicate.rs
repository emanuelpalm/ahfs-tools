use std::fmt;

#[derive(Eq, PartialEq)]
pub enum Predicate {
    Type,
    Description,
    Function,
    Consumes,
    Produces,
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Predicate::Type => "type",
            Predicate::Description => "description",
            Predicate::Function => "function",
            Predicate::Consumes => "consumes",
            Predicate::Produces => "produces",
        })
    }
}
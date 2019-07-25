mod class;
mod parser;
mod lexer;

pub use self::class::Class;

use arspec_parser::{Corpus, Error, Matcher, Parser, Scanner, Token};
use crate::spec::Specification;

/// Attempt to create [`Specification`][spc] from given source [`text`][txt].
///
/// [spc]: ../struct.Specification.html
/// [txt]: ../../../arspec_parser/struct.Text.html
#[inline]
pub fn parse(corpus: &Corpus) -> Result<Specification, Error<Class>> {
    SpecParser::parse(corpus)
}

struct SpecParser;

impl<'a> Parser<'a> for SpecParser {
    type Class = Class;
    type Output = Specification<'a>;

    #[inline]
    fn analyze(scanner: Scanner<'a>) -> Vec<Token<'a, Class>> {
        lexer::scan(scanner)
    }

    #[inline]
    fn combine(spec: &mut Specification<'a>, mut matcher: Matcher<'a, Class>) -> Result<(), Error<Class>> {
        parser::root(spec, &mut matcher)
    }
}

#[cfg(test)]
mod tests {
    use arspec_parser::{Text, Corpus};

    #[test]
    fn example1() {
        let corpus: Corpus = Text {
            name: "alpha.ahfs".into(),
            body: concat!(
                "/// Comment A.\n",
                "service MyService {\n",
                "    /// Comment B.\n",
                "    interface MyInterface {\n",
                "        /// Comment C.\n",
                "        method MyMethod(Argument): Result;\n",
                "    }\n",
                "}\n",
            ).into(),
        }.into();
        let tree = match super::parse(&corpus) {
            Ok(tree) => tree,
            Err(err) => {
                println!("{}", err);
                panic!("{:?}", err);
            }
        };

        assert_eq!(tree.implementations.len(), 0);
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
        let corpus: Corpus = Text {
            name: "alpha.ahfs".into(),
            body: concat!(
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
            ).into()
        }.into();
        if let Err(error) = super::parse(&corpus) {
            panic!("{}", error);
        }
    }
}
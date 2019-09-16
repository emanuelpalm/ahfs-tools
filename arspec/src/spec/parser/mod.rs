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
    use arspec_parser::{Corpus, Text};
    use crate::spec::Value;

    #[test]
    fn example1() {
        let corpus: Corpus = Text {
            name: "alpha.ahfs".into(),
            body: concat!(
                "@Doc(\"Comment A.\")\n",
                "@Author(\"Author Name\")\n",
                "service MyService {\n",
                "    @Doc(\"Comment B.\")\n",
                "    method MyMethod(Argument): Result;\n",
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
        assert_eq!(service.methods.len(), 1);
        assert_eq!(service.attributes.len(), 2);
        assert_eq!(service.attributes[0].name.as_str(), "Doc");
        assert_value_eq_str(&service.attributes[0].value, "\"Comment A.\"");
        assert_eq!(service.attributes[1].name.as_str(), "Author");
        assert_value_eq_str(&service.attributes[1].value, "\"Author Name\"");

        let method = &service.methods[0];
        assert_eq!(method.name.as_str(), "MyMethod");
        assert_eq!(method.input.as_ref().unwrap().name.as_str(), "Argument");
        assert_eq!(method.output.as_ref().unwrap().name.as_str(), "Result");
        assert_eq!(method.attributes.len(), 1);
        assert_value_eq_str(&method.attributes[0].value, "\"Comment B.\"");

        fn assert_value_eq_str(actual: &Value<'_>, expected: &str) {
            match actual {
                Value::String(span) => assert_eq!(span.as_str(), expected),
                other => panic!("Expected Value::String(_), got: {:?}", other),
            }
        }
    }

    #[test]
    fn example2() {
        let corpus: Corpus = Text {
            name: "alpha.ahfs".into(),
            body: concat!(
                "/*\n",
                " * Comment A.\n",
                " * More comment A.\n",
                " */\n",
                "system TestSystem {\n",
                "    // Comment B.\n",
                "    consumes TestServiceX;\n",
                "\n",
                "    /** Comment C. */\n",
                "    produces TestServiceA;\n",
                "}\n",
                "\n",
                "// Comment D.\n",
                "service TestServiceX {\n",
                "    // Comment E.\n",
                "    method FireMissiles(Set<Target>);\n",
                "}\n",
                "\n",
                "// Comment E.\n",
                "implement TestServiceX using HTTP/JSON {\n",
                "    // Comment F.\n",
                "    property BasePath: \"/x\";\n",
                "\n",
                "    // Comment G.\n",
                "    method FireMissiles {\n",
                "        Method: \"POST\",\n",
                "        Path: \"/missile-launches\",\n",
                "     }\n",
                "}\n",
                "\n",
                "// Comment H.\n",
                "record Target {\n",
                "    // Comment I.\n",
                "    X: Integer,\n",
                "}\n",
            ).into()
        }.into();
        if let Err(error) = super::parse(&corpus) {
            panic!("{}", error);
        }
    }
}
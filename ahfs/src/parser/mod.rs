//! Arrowhead specification parsing utilities.
//!
//! This module contains tools useful for parsing specification source texts.

mod token_kind;
mod parse_spec;
mod scan;
mod tree;

pub use self::token_kind::TokenKind;
pub use self::tree::{
    Implement, ImplementInterface, ImplementMethod,
    Property,
    Record, RecordEntry,
    Service, ServiceMethod, ServiceInterface, ServiceRef,
    System,
    Tree, TypeRef,
    Value,
};

use ahfs_parse::{Matcher, Result, Scanner, Source, Token};

#[inline]
pub fn parse(source: &Source) -> Result<Tree, TokenKind> {
    use ahfs_parse::Parser;

    ParserAHFS::parse(source)
}

struct ParserAHFS;

impl<'a> ahfs_parse::Parser<'a> for ParserAHFS {
    type Output = Tree<'a>;
    type TokenKind = TokenKind;

    #[inline]
    fn scan_(scanner: Scanner<'a>) -> Vec<Token<'a, TokenKind>> {
        scan::all(scanner)
    }

    #[inline]
    fn match_(mut matcher: Matcher<'a, TokenKind>) -> Result<Tree<'a>, TokenKind> {
        parse_spec::root(&mut matcher)
    }
}

#[cfg(test)]
mod tests {
    use ahfs_parse::Source;

    #[test]
    fn example1() {
        let source = Source {
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
        };
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
        let source = Source {
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
        };
        if let Err(error) = super::parse(&source) {
            panic!("{}", error);
        }
    }
}
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct Grammar;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier() {
        parses_to! {
            parser: Grammar,
            input: "A",
            rule: Rule::identifier,
            tokens: [
                identifier(0, 1)
            ]
        };
        parses_to! {
            parser: Grammar,
            input: "_is__special____",
            rule: Rule::identifier,
            tokens: [
                identifier(0, 16)
            ]
        };
    }

    #[test]
    fn text() {
        parses_to! {
            parser: Grammar,
            input: "Hello",
            rule: Rule::text,
            tokens: [
                text(0, 5)
            ]
        };
        parses_to! {
            parser: Grammar,
            input: "This is a text with spẽcial chäracters!",
            rule: Rule::text,
            tokens: [
                text(0, 42)
            ]
        };
        parses_to! {
            parser: Grammar,
            input: "This is a text with \\; escaped \\ \\\\characters!",
            rule: Rule::text,
            tokens: [
                text(0, 46)
            ]
        };
        parses_to! {
            parser: Grammar,
            input: "This is a text; with trailing characters.",
            rule: Rule::text,
            tokens: [
                text(0, 14)
            ]
        };
    }

    #[test]
    fn triple() {
        parses_to! {
            parser: Grammar,
            input: "A is: B;",
            rule: Rule::triple,
            tokens: [
                triple(0, 8, [
                    identifier(0, 1),
                    identifier(2, 4),
                    text(6, 7)
                ])
            ]
        };
    }
}
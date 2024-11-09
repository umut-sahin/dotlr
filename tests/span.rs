mod common;

use dotlr::{
    Grammar,
    Parser,
    Span,
};


#[test]
fn correctly_calculating_spans_on_multiline_input() {
    let grammar = Grammar::parse(common::grammars::CALCULATOR).unwrap();
    let parser = Parser::lalr(grammar).unwrap();

    let input = "  11 +  221+3
+20

    +44 +5";
    let tokens = parser.tokenize(input).unwrap();
    
    #[rustfmt::skip]
    assert_eq!(
        tokens.iter().map(|(token, _)| token.span().clone()).collect::<Vec<_>>(),
        [
            Span { line: 1, column: 3, offset: 2, len: 2 },
            Span { line: 1, column: 6, offset: 5, len: 1 },
            Span { line: 1, column: 9, offset: 8, len: 3 },
            Span { line: 1, column: 12, offset: 11, len: 1 },
            Span { line: 1, column: 13, offset: 12, len: 1 },
            Span { line: 2, column: 1, offset: 14, len: 1 },
            Span { line: 2, column: 2, offset: 15, len: 2 },
            Span { line: 4, column: 5, offset: 23, len: 1 },
            Span { line: 4, column: 6, offset: 24, len: 2 },
            Span { line: 4, column: 9, offset: 27, len: 1 },
            Span { line: 4, column: 10, offset: 28, len: 1 },
            Span { line: 4, column: 11, offset: 29, len: 0 },
        ]
    );
}

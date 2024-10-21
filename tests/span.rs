mod common;

use dotlr::{
    Grammar,
    Parser,
    Span,
    Spanned,
    Token,
};

/// Formats the expected and got tokens and spans into a more readable format.
#[allow(unused)]
fn fmt_expected<'i>(tokens: &[(Spanned<Token>, &'i str)], spans: &[Span]) -> String {
    if tokens.len() != spans.len() {
        panic!(
            "Mismatch in the number of tokens and spans. Expected {} got {}",
            spans.len(),
            tokens.len()
        );
    }
    format!(
        "[Expected -> Got] [Offset expected -> Offset Got] {{length}} \n{}",
        tokens
            .iter()
            .zip(spans)
            .map(|((expected_token, slice), got)| {
                let span = expected_token.span();
                format!(
                    "{}:{} -> {}:{} ({} -> {}) [{}] {{{} -> {}}}",
                    span.line,
                    span.column,
                    got.line,
                    got.column,
                    span.offset,
                    got.offset,
                    slice,
                    span.len,
                    got.len
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    )
}

/// Checks if the spans of the tokens are equal to the expected spans.
#[allow(unused)]
fn check_spans<'i>(tokens: Vec<(Spanned<Token>, &'i str)>, spans: &[Span]) {
    if tokens.len() != spans.len() {
        panic!(
            "Mismatch in the number of tokens and spans. Expected {} got {}",
            spans.len(),
            tokens.len()
        );
    }
    for (token, expected_span) in tokens.iter().zip(spans) {
        let span = token.0.span();
        if *span != *expected_span {
            panic!("{}", fmt_expected(&tokens, spans));
        }
    }
}


#[test]
fn correctly_calculate_spans_multi_line() {
    let grammar = Grammar::parse(common::grammars::CALCULATOR).unwrap();
    let parser = Parser::lalr(grammar).unwrap();
    // do not remove the spaces in the string
    let str = "  11 +  221+3
+20

    +44 +5";
    let tokens = parser.tokenize(str).unwrap();

    check_spans(tokens, &[
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
    ]);
}

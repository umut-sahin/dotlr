mod common;

use dotlr::{
    ConstantToken,
    Grammar,
    RegexToken,
    Rule,
    Symbol,
};

#[cfg(target_family = "wasm")]
use wasm_bindgen_test::*;

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn parsing_syntactically_correct_grammars() {
    for grammar in common::grammars::CORRECT {
        assert!(Grammar::parse(grammar).is_ok())
    }
    for grammar in common::grammars::SEMANTICALLY_INCORRECT {
        assert!(Grammar::parse(grammar).is_ok())
    }
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn failing_to_parse_syntactically_incorrect_grammars() {
    for grammar in common::grammars::SYNTACTICALLY_INCORRECT {
        assert!(Grammar::parse(grammar).is_err())
    }
}


#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn raising_correct_error_when_parsing_unexpected_token_grammar() {
    let error = Grammar::parse(common::grammars::UNEXPECTED_TOKEN).unwrap_err();
    match error {
        dotlr::GrammarError::UnexpectedToken { line, column, token, expected } => {
            assert_eq!(line, 1);
            assert_eq!(column, 6);
            assert_eq!(token.as_str(), "->");
            assert_eq!(expected.iter().map(|token| token.as_str()).collect::<Vec<_>>(), [
                "symbol",
                "constant token",
                "regular expression token"
            ]);
        },
        _ => unreachable!(),
    }
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn raising_correct_error_when_parsing_invalid_regex_grammar() {
    let error = Grammar::parse(common::grammars::INVALID_REGEX).unwrap_err();
    match error {
        dotlr::GrammarError::InvalidRegex { line, column, regex } => {
            assert_eq!(line, 3);
            assert_eq!(column, 8);
            assert_eq!(regex.as_str(), "/[1-9][0-9+/");
        },
        _ => unreachable!(),
    }
}


#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn correctly_parsing_calculator_grammar() {
    let grammar = Grammar::parse(common::grammars::CALCULATOR).unwrap();

    assert_eq!(
        grammar.to_string().trim(),
        r#"

Expr -> Expr '+' Factor
Expr -> Expr '-' Factor
Expr -> Factor
Factor -> Factor '*' Exponent
Factor -> Factor '/' Exponent
Factor -> Exponent
Exponent -> Term '^' Exponent
Exponent -> Term
Term -> '(' Expr ')'
Term -> %f

%f -> /^[-]?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?/

        "#
        .trim(),
    );

    assert_eq!(grammar.symbols().iter().map(|symbol| symbol.as_str()).collect::<Vec<_>>(), [
        "Expr", "Factor", "Exponent", "Term"
    ],);

    assert_eq!(grammar.start_symbol().as_str(), "Expr");

    assert_eq!(grammar.constant_tokens().iter().map(|token| token.as_str()).collect::<Vec<_>>(), [
        "+", "-", "*", "/", "^", "(", ")"
    ],);

    assert_eq!(
        grammar.regular_expressions().keys().map(|token| token.as_str()).collect::<Vec<_>>(),
        ["f"],
    );

    assert_eq!(
        grammar.regular_expressions().values().map(|regex| regex.as_str()).collect::<Vec<_>>(),
        [r#"^[-]?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?"#],
    );

    #[rustfmt::skip]
    assert_eq!(
        grammar.rules(),
        [
            // Expr -> Expr '+' Factor
            Rule::new(
                "Expr",
                [
                    Symbol::from("Expr").into(),
                    ConstantToken::from("+").into(),
                    Symbol::from("Factor").into(),
                ]
            ),
            // Expr -> Expr '-' Factor
            Rule::new(
                "Expr",
                [
                    Symbol::from("Expr").into(),
                    ConstantToken::from("-").into(),
                    Symbol::from("Factor").into(),
                ]
            ),
            // Expr -> Factor
            Rule::new(
                "Expr",
                [
                    Symbol::from("Factor").into(),
                ]
            ),

            // Factor -> Factor '*' Exponent
            Rule::new(
                "Factor",
                [
                    Symbol::from("Factor").into(),
                    ConstantToken::from("*").into(),
                    Symbol::from("Exponent").into(),
                ]
            ),
            // Factor -> Factor '/' Exponent
            Rule::new(
                "Factor",
                [
                    Symbol::from("Factor").into(),
                    ConstantToken::from("/").into(),
                    Symbol::from("Exponent").into(),
                ]
            ),
            // Factor -> Exponent
            Rule::new(
                "Factor",
                [
                    Symbol::from("Exponent").into(),
                ]
            ),

            // Exponent -> Term '^' Exponent
            Rule::new(
                "Exponent",
                [
                    Symbol::from("Term").into(),
                    ConstantToken::from("^").into(),
                    Symbol::from("Exponent").into(),
                ]
            ),
            // Exponent -> Term
            Rule::new(
                "Exponent",
                [
                    Symbol::from("Term").into(),
                ]
            ),

            // Term -> '(' Expr ')'
            Rule::new(
                "Term",
                [
                    ConstantToken::from("(").into(),
                    Symbol::from("Expr").into(),
                    ConstantToken::from(")").into(),
                ]
            ),
            // Term -> %f
            Rule::new(
                "Term",
                [
                    RegexToken::from("f").into(),
                ]
            ),
        ],
    );
}

mod common;

use dotlr::{
    Action,
    Grammar,
    Parser,
};


#[test]
fn raising_correct_error_when_encountering_unknown_token_during_parsing_calculator_grammar() {
    let grammar = Grammar::parse(common::grammars::CALCULATOR).unwrap();
    let parser = Parser::lr(grammar).unwrap();

    let error = parser.tokenize("a").unwrap_err();
    assert_eq!(error.to_string(), "unknown token a at 1:1");
}

#[test]
fn raising_correct_error_when_encountering_unexpected_token_during_parsing_calculator_grammar() {
    let grammar = Grammar::parse(common::grammars::CALCULATOR).unwrap();
    let parser = Parser::lr(grammar).unwrap();
    let tokens = parser.tokenize("1 + /").unwrap();

    let error = parser.parse(tokens).unwrap_err();
    assert_eq!(error.to_string(), "unexpected token / at 1:5 (expected one of '(', %f)");
}

#[test]
fn raising_correct_error_when_encountering_unexpected_eof_during_parsing_calculator_grammar() {
    let grammar = Grammar::parse(common::grammars::CALCULATOR).unwrap();
    let parser = Parser::lr(grammar).unwrap();
    let tokens = parser.tokenize("1 + (2").unwrap();

    let error = parser.parse(tokens).unwrap_err();
    assert_eq!(
        error.to_string(),
        "unexpected end of input at 1:7 (expected one of '^', '+', '-', '*', '/', ')')",
    );
}


#[test]
fn correctly_trace_parsing_of_calculator_grammar() {
    let grammar = Grammar::parse(common::grammars::CALCULATOR).unwrap();
    let parser = Parser::lr(grammar).unwrap();

    let expression = "1 + 2 * 3 / (4 ^ 5)";
    let tokens = parser.tokenize(expression).unwrap();

    let (parse_trace, parse_tree) = parser.trace(tokens).unwrap();
    {
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | Step |   State Stack   |         Symbol Stack         |  Remaining Input   |             Action Taken             |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 0    | 0               |                              | %f '+' %f '*' %f $ | Shift 6                              |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 1    | 0 6             | %f                           |    '+' %f '*' %f $ | Reduce Term -> %f                    |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 2    | 0 4             | Term                         |    '+' %f '*' %f $ | Reduce Exponent -> Term              |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 3    | 0 3             | Exponent                     |    '+' %f '*' %f $ | Reduce Factor -> Exponent            |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 4    | 0 2             | Factor                       |    '+' %f '*' %f $ | Reduce Expr -> Factor                |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 5    | 0 1             | Expr                         |    '+' %f '*' %f $ | Shift 32                             |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 6    | 0 1 32          | Expr '+'                     |        %f '*' %f $ | Shift 6                              |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 7    | 0 1 32 6        | Expr '+' %f                  |           '*' %f $ | Reduce Term -> %f                    |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 8    | 0 1 32 4        | Expr '+' Term                |           '*' %f $ | Reduce Exponent -> Term              |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 9    | 0 1 32 3        | Expr '+' Exponent            |           '*' %f $ | Reduce Factor -> Exponent            |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 10   | 0 1 32 35       | Expr '+' Factor              |           '*' %f $ | Shift 28                             |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 11   | 0 1 32 35 28    | Expr '+' Factor '*'          |               %f $ | Shift 6                              |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 12   | 0 1 32 35 28 6  | Expr '+' Factor '*' %f       |                  $ | Reduce Term -> %f                    |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 13   | 0 1 32 35 28 4  | Expr '+' Factor '*' Term     |                  $ | Reduce Exponent -> Term              |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 14   | 0 1 32 35 28 31 | Expr '+' Factor '*' Exponent |                  $ | Reduce Factor -> Factor '*' Exponent |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        // | 15   | 0 1 32 35       | Expr '+' Factor              |                  $ | Accept Expr -> Expr '+' Factor       |
        // +------+-----------------+------------------------------+--------------------+--------------------------------------+
        assert_eq!(
            parse_trace.steps().iter().map(|step| *step.action_taken()).collect::<Vec<_>>(),
            [
                Action::Shift { next_state: 6 },
                Action::Reduce { rule_index: 9 },
                Action::Reduce { rule_index: 7 },
                Action::Reduce { rule_index: 5 },
                Action::Reduce { rule_index: 2 },
                Action::Shift { next_state: 32 },
                Action::Shift { next_state: 6 },
                Action::Reduce { rule_index: 9 },
                Action::Reduce { rule_index: 7 },
                Action::Reduce { rule_index: 5 },
                Action::Shift { next_state: 28 },
                Action::Shift { next_state: 6 },
                Action::Reduce { rule_index: 9 },
                Action::Reduce { rule_index: 7 },
                Action::Reduce { rule_index: 3 },
                Action::Shift { next_state: 29 },
                Action::Shift { next_state: 5 },
                Action::Shift { next_state: 9 },
                Action::Reduce { rule_index: 9 },
                Action::Shift { next_state: 14 },
                Action::Shift { next_state: 9 },
                Action::Reduce { rule_index: 9 },
                Action::Reduce { rule_index: 7 },
                Action::Reduce { rule_index: 6 },
                Action::Reduce { rule_index: 5 },
                Action::Reduce { rule_index: 2 },
                Action::Shift { next_state: 25 },
                Action::Reduce { rule_index: 8 },
                Action::Reduce { rule_index: 7 },
                Action::Reduce { rule_index: 4 },
                Action::Accept { rule_index: 0 }
            ],
        );
    }
    {
        assert_eq!(
            parse_tree.to_string().trim(),
            r#"

Expr
├─ Expr
│  └─ Factor
│     └─ Exponent
│        └─ Term
│           └─ 1
├─ +
└─ Factor
   ├─ Factor
   │  ├─ Factor
   │  │  └─ Exponent
   │  │     └─ Term
   │  │        └─ 2
   │  ├─ *
   │  └─ Exponent
   │     └─ Term
   │        └─ 3
   ├─ /
   └─ Exponent
      └─ Term
         ├─ (
         ├─ Expr
         │  └─ Factor
         │     └─ Exponent
         │        ├─ Term
         │        │  └─ 4
         │        ├─ ^
         │        └─ Exponent
         │           └─ Term
         │              └─ 5
         └─ )

            "#
            .trim(),
        );
    }
}

#[test]
fn correctly_parsing_conditional_grammar() {
    let grammar = Grammar::parse(common::grammars::CONDITIONAL).unwrap();
    let parser = Parser::lr(grammar).unwrap();

    let expression = "if true { if_case } else { else_case }";
    let tokens = parser.tokenize(expression).unwrap();

    let parse_tree = parser.parse(tokens).unwrap();
    assert_eq!(
        parse_tree.to_string().trim(),
        r#"

Conditional
├─ if
├─ Expression
│  └─ true
├─ {
├─ Expression
│  └─ Identifier
│     └─ if_case
├─ }
├─ else
├─ {
├─ Expression
│  └─ Identifier
│     └─ else_case
└─ } 

            "#
        .trim(),
    );
}

#[test]
fn correctly_parsing_json_grammar_with_lalr() {
    let grammar = Grammar::parse(common::grammars::JSON).unwrap();
    let parser = Parser::lalr(grammar).unwrap();

    let expression = include_str!("../assets/data/sample.json");
    let tokens = parser.tokenize(expression).unwrap();

    let parse_tree = parser.parse(tokens).unwrap();
    assert_eq!(
        parse_tree.to_string().trim(),
        r#"

Json
└─ Value
   └─ Object
      ├─ {
      ├─ ObjectElements
      │  ├─ ObjectElements
      │  │  ├─ ObjectElements
      │  │  │  ├─ ObjectElements
      │  │  │  │  ├─ ObjectElements
      │  │  │  │  │  ├─ ObjectElements
      │  │  │  │  │  │  ├─ String
      │  │  │  │  │  │  │  └─ "name"
      │  │  │  │  │  │  ├─ :
      │  │  │  │  │  │  └─ Value
      │  │  │  │  │  │     └─ String
      │  │  │  │  │  │        └─ "Sample"
      │  │  │  │  │  ├─ ,
      │  │  │  │  │  ├─ String
      │  │  │  │  │  │  └─ "rating"
      │  │  │  │  │  ├─ :
      │  │  │  │  │  └─ Value
      │  │  │  │  │     └─ Number
      │  │  │  │  │        └─ 4.2
      │  │  │  │  ├─ ,
      │  │  │  │  ├─ String
      │  │  │  │  │  └─ "homepage"
      │  │  │  │  ├─ :
      │  │  │  │  └─ Value
      │  │  │  │     └─ Null
      │  │  │  │        └─ null
      │  │  │  ├─ ,
      │  │  │  ├─ String
      │  │  │  │  └─ "is_active"
      │  │  │  ├─ :
      │  │  │  └─ Value
      │  │  │     └─ Boolean
      │  │  │        └─ true
      │  │  ├─ ,
      │  │  ├─ String
      │  │  │  └─ "tags"
      │  │  ├─ :
      │  │  └─ Value
      │  │     └─ Array
      │  │        ├─ [
      │  │        ├─ ArrayElements
      │  │        │  ├─ ArrayElements
      │  │        │  │  ├─ ArrayElements
      │  │        │  │  │  └─ Value
      │  │        │  │  │     └─ String
      │  │        │  │  │        └─ "a"
      │  │        │  │  ├─ ,
      │  │        │  │  └─ Value
      │  │        │  │     └─ String
      │  │        │  │        └─ "b"
      │  │        │  ├─ ,
      │  │        │  └─ Value
      │  │        │     └─ String
      │  │        │        └─ "c"
      │  │        └─ ]
      │  ├─ ,
      │  ├─ String
      │  │  └─ "metadata"
      │  ├─ :
      │  └─ Value
      │     └─ Object
      │        ├─ {
      │        ├─ ObjectElements
      │        │  ├─ ObjectElements
      │        │  │  ├─ String
      │        │  │  │  └─ "foo"
      │        │  │  ├─ :
      │        │  │  └─ Value
      │        │  │     └─ String
      │        │  │        └─ "bar"
      │        │  ├─ ,
      │        │  ├─ String
      │        │  │  └─ "bar"
      │        │  ├─ :
      │        │  └─ Value
      │        │     └─ Number
      │        │        └─ 10
      │        └─ }
      └─ }

            "#
        .trim(),
    );
}

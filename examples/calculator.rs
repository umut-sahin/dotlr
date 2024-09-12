use {
    colored::Colorize,
    dotlr::{
        Grammar,
        Parser,
        Token,
        Tree,
    },
    rustyline::{
        DefaultEditor,
        error::ReadlineError,
    },
    std::process::ExitCode,
};

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);

    let grammar_string = include_str!("../assets/grammars/correct/calculator.lr");
    let grammar = Grammar::parse(grammar_string).expect("invalid grammar");
    let parser = Parser::lr(grammar).expect("unsupported grammar");

    match args.next() {
        Some(input) => calculate(&parser, &input),
        None => repl(&parser),
    }
}

fn repl(parser: &Parser) -> ExitCode {
    let mut editor = DefaultEditor::new().expect("repl cannot be created");

    let history_file = dirs::data_dir().map(|dir| dir.join("dotlr")).map(|dir| {
        if !dir.exists() {
            std::fs::create_dir_all(&dir).ok();
        }
        dir.join("calculator.history")
    });
    history_file.as_ref().inspect(|history_file| {
        editor.load_history(&history_file).ok();
    });

    let cursor = format!("{} ", ">".cyan().bold());
    loop {
        let readline = editor.readline(&cursor);
        match readline {
            Ok(line) => {
                if !line.is_empty() {
                    editor.add_history_entry(line.as_str()).ok();
                    history_file.as_ref().inspect(|history_file| {
                        editor.save_history(&history_file).ok();
                    });
                    calculate(parser, &line);
                }
            },
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                return ExitCode::SUCCESS;
            },
            Err(error) => {
                panic!("input cannot be read: {}", error);
            },
        }
    }
}

fn calculate(parser: &Parser, input: &str) -> ExitCode {
    let tokens = match parser.tokenize(input) {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("{} {}", "tokenization error:".red().bold(), error);
            return ExitCode::FAILURE;
        },
    };
    match parser.parse(tokens) {
        Ok(parse_tree) => {
            println!("{}", evaluate(parse_tree));
            ExitCode::SUCCESS
        },
        Err(error) => {
            eprintln!("{} {}", "syntax error:".red().bold(), error);
            ExitCode::FAILURE
        },
    }
}

fn evaluate(tree: Tree<'_>) -> f64 {
    match tree {
        Tree::Terminal { token, slice } => {
            match token {
                Token::Regex(regex_token) => {
                    match regex_token.as_str() {
                        "f" => slice.parse().unwrap(),
                        _ => unreachable!(),
                    }
                },
                Token::Constant(_) | Token::Eof => {
                    unreachable!();
                },
            }
        },
        Tree::NonTerminal { symbol, pattern } => {
            let mut pattern = pattern.into_iter();
            match symbol.as_str() {
                "Expr" => {
                    if pattern.len() == 1 {
                        evaluate(pattern.next().unwrap())
                    } else {
                        assert_eq!(pattern.len(), 3);
                        let lhs = pattern.next().unwrap();
                        let operation = pattern.next().unwrap();
                        let rhs = pattern.next().unwrap();
                        match operation {
                            Tree::Terminal { slice, .. } => {
                                match slice {
                                    "+" => evaluate(lhs) + evaluate(rhs),
                                    "-" => evaluate(lhs) - evaluate(rhs),
                                    _ => unreachable!(),
                                }
                            },
                            _ => unreachable!(),
                        }
                    }
                },
                "Factor" => {
                    if pattern.len() == 1 {
                        evaluate(pattern.next().unwrap())
                    } else {
                        assert_eq!(pattern.len(), 3);
                        let lhs = pattern.next().unwrap();
                        let operation = pattern.next().unwrap();
                        let rhs = pattern.next().unwrap();
                        match operation {
                            Tree::Terminal { slice, .. } => {
                                match slice {
                                    "*" => evaluate(lhs) * evaluate(rhs),
                                    "/" => evaluate(lhs) / evaluate(rhs),
                                    _ => unreachable!(),
                                }
                            },
                            _ => unreachable!(),
                        }
                    }
                },
                "Exponent" => {
                    if pattern.len() == 1 {
                        evaluate(pattern.next().unwrap())
                    } else {
                        assert_eq!(pattern.len(), 3);
                        let lhs = pattern.next().unwrap();
                        let operation = pattern.next().unwrap();
                        let rhs = pattern.next().unwrap();
                        match operation {
                            Tree::Terminal { slice, .. } => {
                                if slice == "^" {
                                    evaluate(lhs).powf(evaluate(rhs))
                                } else {
                                    unreachable!()
                                }
                            },
                            _ => unreachable!(),
                        }
                    }
                },
                "Term" => {
                    if pattern.len() == 1 {
                        evaluate(pattern.next().unwrap())
                    } else {
                        assert_eq!(pattern.len(), 3);
                        evaluate(pattern.nth(1).unwrap())
                    }
                },
                _ => {
                    unreachable!();
                },
            }
        },
    }
}

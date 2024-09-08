use {
    clap::Parser as Clap,
    colored::Colorize,
    dotlr::{
        Grammar,
        Parser,
        ParserError,
    },
    rustyline::{
        error::ReadlineError,
        DefaultEditor,
    },
    std::{
        path::PathBuf,
        process::ExitCode,
    },
};

#[derive(Clap)]
struct Args {
    /// Create an LALR(1) parser instead of an LR(1) parser.
    #[arg(long)]
    lalr: bool,

    /// Grammar to parse.
    grammar: PathBuf,

    /// Input to parse.
    input: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let grammar = match std::fs::read_to_string(args.grammar) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("{} grammar file cannot be read ({})", "io error:".red().bold(), error);
            return ExitCode::FAILURE;
        },
    };
    let grammar = match Grammar::parse(&grammar) {
        Ok(grammar) => grammar,
        Err(error) => {
            eprintln!("{} {}", "grammar error:".red().bold(), error);
            return ExitCode::FAILURE;
        },
    };
    let parser = {
        if args.lalr {
            match Parser::lalr(grammar) {
                Ok(parser) => parser,
                Err(error) => {
                    eprintln!("{} {}", "lr parser error:".red().bold(), error);
                    if let ParserError::Conflict { parser, .. } = error {
                        parser.dump();
                    }
                    return ExitCode::FAILURE;
                },
            }
        } else {
            match Parser::lr(grammar) {
                Ok(parser) => parser,
                Err(error) => {
                    eprintln!("{} {}", "lalr parser error:".red().bold(), error);
                    if let ParserError::Conflict { parser, .. } = error {
                        parser.dump();
                    }
                    return ExitCode::FAILURE;
                },
            }
        }
    };

    println!();
    parser.dump();
    println!();

    match args.input {
        Some(input) => {
            println!("{} {}", ">".cyan().bold(), input);
            parse(&parser, &input)
        },
        None => repl(&parser),
    }
}

fn repl(parser: &Parser) -> ExitCode {
    let mut editor = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(error) => {
            eprintln!("{} editor cannot be created ({})", "repl error:".red().bold(), error);
            return ExitCode::FAILURE;
        },
    };

    let history_file = dirs::data_dir().map(|dir| dir.join("dotlr")).map(|dir| {
        if !dir.exists() {
            std::fs::create_dir_all(&dir).ok();
        }
        dir.join("repl.history")
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
                    parse(parser, &line);
                }
            },
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!();
                return ExitCode::SUCCESS;
            },
            Err(error) => {
                println!();
                eprintln!("{} input to parse cannot be read ({})", "io error:".red().bold(), error);
                println!();
                return ExitCode::FAILURE;
            },
        }
    }
}

fn parse(parser: &Parser, input: &str) -> ExitCode {
    let tokens = match parser.tokenize(input) {
        Ok(tokens) => tokens,
        Err(error) => {
            println!();
            eprintln!("{} {}", "tokenization error:".red().bold(), error);
            println!();
            return ExitCode::FAILURE;
        },
    };
    match parser.trace(tokens) {
        Ok((parse_trace, parse_tree)) => {
            println!();
            parse_tree.dump();
            println!();
            parse_trace.dump(parser.grammar());
            println!();
            ExitCode::SUCCESS
        },
        Err(error) => {
            println!();
            eprintln!("{} {}", "syntax error:".red().bold(), error);
            println!();
            ExitCode::FAILURE
        },
    }
}

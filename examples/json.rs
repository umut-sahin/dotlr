use {
    dotlr::{
        Grammar,
        Parser,
        Token,
        Tree,
    },
    indexmap::IndexMap,
};

#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

impl From<Tree<'_>> for Value {
    fn from(tree: Tree) -> Value {
        match tree {
            Tree::Terminal { token, slice } => {
                match token {
                    Token::Constant(constant_token) => {
                        match constant_token.as_str() {
                            "null" => Value::Null,
                            "true" => Value::Bool(true),
                            "false" => Value::Bool(false),
                            _ => unreachable!(),
                        }
                    },
                    Token::Regex(regex_token) => {
                        match regex_token.as_str() {
                            "f" => Value::Number(slice.parse().unwrap()),
                            "s" => Value::String(slice.trim_matches('"').to_owned()),
                            _ => unreachable!(),
                        }
                    },
                    Token::Eof => {
                        unreachable!();
                    },
                }
            },
            Tree::NonTerminal { symbol, pattern } => {
                let mut pattern = pattern.into_iter();
                match symbol.as_str() {
                    "Json" | "Value" | "Null" | "Boolean" | "Number" | "String" => {
                        assert_eq!(pattern.len(), 1);
                        Value::from(pattern.next().unwrap())
                    },

                    "Array" => {
                        if pattern.len() == 2 {
                            // Array -> '[' ']'
                            Value::Array(Vec::new())
                        } else {
                            // Array -> '[' ArrayElements ']'
                            fn collect(elements: Tree, values: &mut Vec<Value>) {
                                match elements {
                                    Tree::NonTerminal { symbol, pattern }
                                        if symbol.as_str() == "ArrayElements" =>
                                    {
                                        let mut pattern = pattern.into_iter();
                                        if pattern.len() == 1 {
                                            // ArrayElements -> Value
                                            values.push(Value::from(pattern.next().unwrap()));
                                        } else {
                                            // ArrayElements -> ArrayElements ',' Value
                                            assert_eq!(pattern.len(), 3);
                                            collect(pattern.next().unwrap(), values);
                                            let _comma = pattern.next().unwrap();
                                            values.push(Value::from(pattern.next().unwrap()));
                                        }
                                    },
                                    _ => {
                                        unreachable!();
                                    },
                                }
                            }

                            let mut values = Vec::new();
                            collect(pattern.nth(1).unwrap(), &mut values);

                            Value::Array(values)
                        }
                    },
                    "Object" => {
                        if pattern.len() == 2 {
                            // Object -> '{' '}'
                            Value::Object(IndexMap::new())
                        } else {
                            // Object -> '{' ObjectElements '}'
                            fn collect(elements: Tree, entries: &mut IndexMap<String, Value>) {
                                match elements {
                                    Tree::NonTerminal { symbol, pattern }
                                        if symbol.as_str() == "ObjectElements" =>
                                    {
                                        let mut pattern = pattern.into_iter();

                                        if pattern.len() == 5 {
                                            // ObjectElements -> ObjectElements ',' String ':' Value
                                            collect(pattern.next().unwrap(), entries);
                                            let _comma = pattern.next().unwrap();
                                        }

                                        // ObjectElements -> String ':' Value
                                        assert_eq!(pattern.len(), 3);

                                        let key = match Value::from(pattern.next().unwrap()) {
                                            Value::String(key) => key,
                                            _ => unreachable!(),
                                        };
                                        let _colon = pattern.next().unwrap();
                                        let value = Value::from(pattern.next().unwrap());

                                        entries.insert(key, value);
                                    },
                                    _ => {
                                        unreachable!();
                                    },
                                }
                            }

                            let mut entries = IndexMap::new();
                            collect(pattern.nth(1).unwrap(), &mut entries);

                            Value::Object(entries)
                        }
                    },

                    _ => {
                        unreachable!();
                    },
                }
            },
        }
    }
}

fn main() {
    let grammar_string = include_str!("../assets/grammars/correct/json.lr");
    let grammar = Grammar::parse(grammar_string).expect("invalid grammar");

    let parser = Parser::lalr(grammar).expect("unsupported grammar");

    let input = include_str!("../assets/data/sample.json");
    let tokens = parser.tokenize(input).expect("tokenization failed");

    let parse_tree = parser.parse(tokens).expect("parsing failed");
    parse_tree.dump();

    let json = Value::from(parse_tree);
    println!("\n{:#?}", json);
}

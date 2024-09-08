use crate::prelude::*;


/// LR(1) parser of a grammar.
#[derive(Debug)]
pub struct Parser {
    grammar: Grammar,
    first_table: FirstTable,
    follow_table: FollowTable,
    automaton: Automaton,
    parsing_tables: ParsingTables,
}

impl Parser {
    /// Crates an LR(1) parser of a grammar.
    pub fn lr(grammar: Grammar) -> Result<Parser, ParserError> {
        Parser::check_grammar_internal(&grammar)?;

        let first_table = FirstTable::construct(&grammar);
        let follow_table = FollowTable::construct(&grammar, &first_table);
        let automaton = Automaton::construct(&grammar, &first_table);
        let parsing_tables = ParsingTables::construct(&grammar, &follow_table, &automaton)?;

        let parser = Parser { grammar, first_table, follow_table, automaton, parsing_tables };
        parser.check_conflicts_internal()
    }

    /// Crates an LALR(1) parser of a grammar.
    pub fn lalr(grammar: Grammar) -> Result<Parser, ParserError> {
        Parser::check_grammar_internal(&grammar)?;

        let first_table = FirstTable::construct(&grammar);
        let follow_table = FollowTable::construct(&grammar, &first_table);
        let automaton = Automaton::construct(&grammar, &first_table).to_lalr();
        let parsing_tables = ParsingTables::construct(&grammar, &follow_table, &automaton)?;

        let parser = Parser { grammar, first_table, follow_table, automaton, parsing_tables };
        parser.check_conflicts_internal()
    }
}

impl Parser {
    /// Gets the grammar of the parser.
    pub fn grammar(&self) -> &Grammar {
        &self.grammar
    }

    /// Gets the first table of the symbols in the grammar of the parser.
    pub fn first_table(&self) -> &FirstTable {
        &self.first_table
    }

    /// Gets the follow table of the symbols in the grammar of the parser.
    pub fn follow_table(&self) -> &FollowTable {
        &self.follow_table
    }

    /// Gets the automaton of the grammar of the parser.
    pub fn automaton(&self) -> &Automaton {
        &self.automaton
    }

    /// Gets the parsing tables of the parser.
    pub fn parsing_tables(&self) -> &ParsingTables {
        &self.parsing_tables
    }

    /// Gets the action table of the parser.
    pub fn action_table(&self) -> &[IndexMap<Token, IndexSet<Action>>] {
        self.parsing_tables.action_table()
    }

    /// Gets the goto table of the parser.
    pub fn goto_table(&self) -> &[IndexMap<Symbol, usize>] {
        self.parsing_tables.goto_table()
    }
}

impl Parser {
    /// Tokenizes an input into a stream of tokens and their corresponding input slices.
    pub fn tokenize<'i>(&self, input: &'i str) -> Result<Vec<(Token, &'i str)>, ParsingError> {
        let mut tokens = Vec::new();

        let mut ordered_constant_tokens = self.grammar.constant_tokens().iter().collect::<Vec<_>>();
        ordered_constant_tokens.sort_by_key(|token| token.len());

        let mut remaining_input = input.trim_start();
        while !remaining_input.is_empty() {
            let mut matching_token = None;
            let mut matching_slice = "";

            for token in ordered_constant_tokens.iter().rev() {
                if remaining_input.starts_with(token.as_str()) {
                    matching_token = Some(Token::Constant((*token).clone()));
                    matching_slice = &remaining_input[..token.len()];
                    break;
                }
            }
            for (regex_token, regex) in self.grammar.regular_expressions() {
                if let Some(match_info) = regex.find(remaining_input) {
                    if match_info.len() > matching_slice.len() {
                        matching_token = Some(Token::Regex(regex_token.clone()));
                        matching_slice = &remaining_input[..match_info.end()];
                    }
                }
            }

            if matching_token.is_none() {
                return Err(ParsingError::UnknownToken {
                    token: format_smolstr!("{}", remaining_input.chars().next().unwrap()),
                });
            }

            tokens.push((matching_token.unwrap(), matching_slice));
            remaining_input = remaining_input[matching_slice.len()..].trim();
        }
        tokens.push((Token::Eof, "\0"));

        Ok(tokens)
    }

    /// Parses a tokenized input.
    pub fn parse<'i>(&self, tokens: Vec<(Token, &'i str)>) -> Result<Tree<'i>, ParsingError> {
        self.parse_and_trace_internal(tokens, false).map(|(_, tree)| tree)
    }

    /// Traces the parsing of a tokenized input.
    pub fn trace<'i>(
        &self,
        tokens: Vec<(Token, &'i str)>,
    ) -> Result<(Trace<'i>, Tree<'i>), ParsingError> {
        self.parse_and_trace_internal(tokens, true)
    }
}

impl Parser {
    /// Internal grammar checks.
    fn check_grammar_internal(grammar: &Grammar) -> Result<(), ParserError> {
        if grammar.rules().is_empty() {
            return Err(ParserError::EmptyGrammar);
        }
        for rule in grammar.rules() {
            for atomic_pattern in rule.pattern() {
                match atomic_pattern {
                    AtomicPattern::Symbol(symbol) => {
                        if !grammar.symbols().contains(symbol) {
                            return Err(ParserError::UndefinedSymbol {
                                symbol: symbol.clone(),
                                rule: rule.clone(),
                            });
                        }
                    },
                    AtomicPattern::Token(token) => {
                        if let Token::Regex(regex_token) = token {
                            if !grammar.regular_expressions().contains_key(regex_token) {
                                return Err(ParserError::UndefinedRegexToken {
                                    regex_token: regex_token.clone(),
                                    rule: rule.clone(),
                                });
                            }
                        }
                    },
                }
            }
        }
        Ok(())
    }

    /// Internal conflict checks.
    fn check_conflicts_internal(self) -> Result<Parser, ParserError> {
        for (state, action_map) in self.action_table().iter().enumerate() {
            for (token, actions) in action_map.iter() {
                if actions.len() > 1 {
                    let token = token.clone();
                    let parser = Box::new(self);
                    return Err(ParserError::Conflict { parser, state, token });
                }
            }
        }
        Ok(self)
    }

    /// Internal parsing logic.
    fn parse_and_trace_internal<'i>(
        &self,
        mut tokens: Vec<(Token, &'i str)>,
        traced: bool,
    ) -> Result<(Trace<'i>, Tree<'i>), ParsingError> {
        let mut state_stack = vec![0];
        let mut tree_stack = vec![];
        let mut remaining_tokens = {
            tokens.reverse();
            tokens
        };

        let mut trace = Trace::default();

        let (mut current_token, mut current_slice) = remaining_tokens.pop().unwrap();
        loop {
            let current_state = *state_stack.last().unwrap();
            let action_to_take = match self.action_table()[current_state].get(&current_token) {
                Some(actions) => {
                    assert_eq!(actions.len(), 1);
                    *actions.iter().next().unwrap()
                },
                None => {
                    let mut expected = smallvec![];
                    for (token, _) in self.action_table()[current_state].iter() {
                        expected.push(token.clone());
                    }

                    return Err(if current_token == Token::Eof {
                        ParsingError::UnexpectedEof { expected }
                    } else {
                        ParsingError::UnexpectedToken { token: current_slice.into(), expected }
                    });
                },
            };

            if traced {
                let mut remaining_tokens_without_slices =
                    remaining_tokens.iter().map(|(token, _)| token.clone()).collect::<Vec<_>>();
                remaining_tokens_without_slices.push(current_token.clone());

                trace.step(Step {
                    state_stack: state_stack.clone(),
                    tree_stack: tree_stack.clone(),
                    remaining_tokens: remaining_tokens_without_slices,
                    action_taken: action_to_take,
                });
            }

            match action_to_take {
                Action::Accept { .. } => {
                    let parse_tree = Tree::NonTerminal {
                        symbol: self.grammar.start_symbol().clone(),
                        pattern: tree_stack,
                    };
                    return Ok((trace, parse_tree));
                },
                Action::Shift { next_state } => {
                    state_stack.push(next_state);
                    tree_stack.push(Tree::Terminal {
                        token: current_token.clone(),
                        slice: current_slice,
                    });
                    (current_token, current_slice) = remaining_tokens.pop().unwrap();
                },
                Action::Reduce { rule_index } => {
                    let rule = &self.grammar.rules()[rule_index];
                    let pattern_length = rule.pattern().len();

                    let symbol = rule.symbol().clone();
                    let pattern =
                        tree_stack.split_off(tree_stack.len().saturating_sub(pattern_length));

                    tree_stack.push(Tree::NonTerminal { symbol, pattern });

                    let new_state_stack_len = state_stack.len().saturating_sub(pattern_length);
                    state_stack.truncate(new_state_stack_len);

                    let new_state = *state_stack.last().unwrap();
                    match self.goto_table()[new_state].get(rule.symbol()) {
                        Some(state) => {
                            state_stack.push(*state);
                        },
                        None => {
                            unreachable!();
                        },
                    }
                },
            }
        }
    }
}

impl Parser {
    /// Dumps the parser to stdout.
    pub fn dump(&self) {
        {
            let mut pretty_grammar = Table::new();

            pretty_grammar.add_row(row![cbFy->"Grammar"]);
            pretty_grammar.add_row(row![self.grammar]);

            pretty_grammar.printstd();
        }
        {
            let mut pretty_first_and_follow_tables = Table::new();

            pretty_first_and_follow_tables
                .add_row(row![cbFy->"Symbol", cbFy->"First Set", cbFy->"Follow Set"]);
            for (symbol, first_set) in self.first_table.iter() {
                let first_set_formatted = {
                    if first_set.is_empty() {
                        "{}".to_owned()
                    } else {
                        format!("{{ {} }}", first_set.iter().join(", "))
                    }
                };
                let follow_set_formatted = {
                    match self.follow_table.get(symbol) {
                        Some(follow_set) if !follow_set.is_empty() => {
                            format!("{{ {} }}", follow_set.iter().join(", "))
                        },
                        _ => "{}".to_owned(),
                    }
                };
                pretty_first_and_follow_tables.add_row(row![
                    symbol,
                    first_set_formatted,
                    follow_set_formatted
                ]);
            }

            pretty_first_and_follow_tables.printstd();
        }
        {
            let mut pretty_automaton = Table::new();

            pretty_automaton.add_row(
                row![cbFy->"State", cbFy->"Items", cbFy->"Lookaheads", cbFy->"Transitions"],
            );
            for state in self.automaton.states().iter() {
                let mut pretty_items = Table::new();
                {
                    for item in state.items().iter() {
                        pretty_items.add_row(row![item]);
                    }
                    pretty_items.set_format(FormatBuilder::new().padding(1, 1).build());
                }

                let mut pretty_lookaheads = Table::new();
                {
                    for item in state.items().iter() {
                        let lookahead_set_formatted = {
                            if item.lookahead().is_empty() {
                                "{}".to_owned()
                            } else {
                                format!("{{ {} }}", item.lookahead().iter().join(", "))
                            }
                        };
                        pretty_lookaheads.add_row(row![lookahead_set_formatted]);
                    }
                    pretty_lookaheads.set_format(FormatBuilder::new().padding(0, 0).build());
                }

                let mut pretty_transitions = Table::new();
                {
                    let mut sorted_transitions = state.transitions().iter().collect::<Vec<_>>();
                    sorted_transitions.sort_by_key(|transition| transition.1);
                    for (atomic_pattern, new_state) in sorted_transitions {
                        pretty_transitions.add_row(row![c->atomic_pattern, "->", l->new_state]);
                    }
                    pretty_transitions.set_format(FormatBuilder::new().padding(1, 1).build());
                }

                pretty_automaton.add_row(
                    row![state.id(), pretty_items, pretty_lookaheads, c->pretty_transitions],
                );
            }

            pretty_automaton.printstd();
        }
        {
            let all_tokens = self
                .grammar
                .constant_tokens()
                .iter()
                .cloned()
                .map(Token::Constant)
                .chain(self.grammar.regular_expressions().keys().cloned().map(Token::Regex))
                .chain(std::iter::once(Token::Eof))
                .collect::<Vec<_>>();

            let longest_state_length = format_smolstr!("{}", self.automaton.states().len()).len();
            let longest_actions_length = self
                .action_table()
                .iter()
                .flat_map(|action_map| action_map.values())
                .map(|actions| {
                    let actions =
                        actions.iter().map(|action| format_smolstr!("{}", action)).join(", ");
                    actions.len()
                })
                .max()
                .unwrap_or(0);

            fn pad(string: impl ToString, to: usize) -> String {
                let string = string.to_string();
                if string.len() >= to {
                    return string;
                }

                let spaces = to - string.len();
                let to_lhs = spaces / 2;
                let to_rhs = spaces - to_lhs;

                [" ".repeat(to_lhs), string, " ".repeat(to_rhs)].join("")
            }

            let mut pretty_action_header = Table::new();
            {
                let mut actions_row = Row::empty();
                for token in all_tokens.iter() {
                    let action = if *token == Token::Eof {
                        format_smolstr!(" {} ", token)
                    } else {
                        format_smolstr!("{}", token)
                    };
                    actions_row.add_cell(cell![pad(action, longest_actions_length)]);
                }

                let mut pretty_actions_table = Table::init(vec![actions_row]);
                pretty_actions_table.set_format(
                    FormatBuilder::new()
                        .padding(2, 2)
                        .separator(LinePosition::Intern, LineSeparator::new('-', '+', '+', '+'))
                        .build(),
                );

                pretty_action_header.add_row(row![c->"Action"]);
                pretty_action_header.add_row(row![pretty_actions_table]);

                pretty_action_header.set_format(*prettytable::format::consts::FORMAT_NO_BORDER);
            }

            let mut pretty_goto_header = Table::new();
            {
                let mut gotos_row = Row::empty();
                for symbol in self.grammar.symbols().iter() {
                    gotos_row.add_cell(cell![pad(symbol, longest_state_length)]);
                }

                let mut pretty_gotos_table = Table::init(vec![gotos_row]);
                pretty_gotos_table.set_format(
                    FormatBuilder::new()
                        .padding(2, 2)
                        .separator(LinePosition::Intern, LineSeparator::new('-', '+', '+', '+'))
                        .build(),
                );

                pretty_goto_header.add_row(row![c->"Goto"]);
                pretty_goto_header.add_row(row![pretty_gotos_table]);

                pretty_goto_header.set_format(*prettytable::format::consts::FORMAT_NO_BORDER);
            }

            let mut pretty_parsing_tables = Table::new();
            {
                pretty_parsing_tables.add_row(
                    row![cbFy->"\nState", bFy->pretty_action_header, bFy->pretty_goto_header],
                );

                for (i, (action_map, goto_map)) in self
                    .parsing_tables
                    .action_table()
                    .iter()
                    .zip(self.parsing_tables.goto_table().iter())
                    .enumerate()
                {
                    let mut actions_row = Row::empty();
                    for token in all_tokens.iter() {
                        let mut padding = format_smolstr!("{}", token).len();
                        if *token == Token::Eof {
                            padding += 2;
                        }
                        padding = padding.max(longest_actions_length);
                        match action_map.get(token) {
                            Some(actions) => {
                                let actions = actions
                                    .iter()
                                    .map(|action| format_smolstr!("{}", action))
                                    .join(", ");
                                actions_row.add_cell(cell![pad(actions, padding)]);
                            },
                            None => {
                                actions_row.add_cell(cell![pad("-", padding)]);
                            },
                        }
                    }

                    let mut pretty_actions_table = Table::init(vec![actions_row]);
                    pretty_actions_table.set_format(
                        FormatBuilder::new()
                            .padding(2, 2)
                            .separator(LinePosition::Intern, LineSeparator::new('-', '+', '+', '+'))
                            .build(),
                    );

                    let mut gotos_row = Row::empty();
                    for symbol in self.grammar.symbols().iter() {
                        match goto_map.get(symbol) {
                            Some(state) => {
                                gotos_row.add_cell(cell![pad(
                                    state,
                                    symbol.len().max(longest_state_length)
                                )]);
                            },
                            None => {
                                gotos_row.add_cell(cell![pad(
                                    "-",
                                    symbol.len().max(longest_state_length)
                                )]);
                            },
                        }
                    }

                    let mut pretty_gotos_table = Table::init(vec![gotos_row]);
                    pretty_gotos_table.set_format(
                        FormatBuilder::new()
                            .padding(2, 2)
                            .separator(LinePosition::Intern, LineSeparator::new('-', '+', '+', '+'))
                            .build(),
                    );

                    pretty_parsing_tables
                        .add_row(row![i, c->pretty_actions_table, c->pretty_gotos_table]);
                }
            }

            pretty_parsing_tables.printstd()
        }
    }
}

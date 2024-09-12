use crate::prelude::*;


/// First table of the symbols in a grammar.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Debug)]
pub struct FirstTable(IndexMap<Symbol, IndexSet<Token>>);

impl FirstTable {
    /// Constructs the first table from the grammar.
    pub fn construct(grammar: &Grammar) -> FirstTable {
        let mut first_table = IndexMap::new();

        let mut done = false;
        while !done {
            done = true;
            for rule in grammar.rules().iter() {
                let symbol = rule.symbol();
                let first_atomic_pattern = rule.pattern().first().unwrap();

                if matches!(
                    first_atomic_pattern,
                    AtomicPattern::Symbol(atomic_pattern) if atomic_pattern == symbol
                ) {
                    continue;
                }

                let mut possible_first_tokens = match first_table.get_mut(symbol) {
                    Some(entry) => std::mem::take(entry),
                    None => IndexSet::new(),
                };

                let old_possible_first_token_count = possible_first_tokens.len();
                match first_atomic_pattern {
                    AtomicPattern::Symbol(symbol) => {
                        if let Some(new_possible_first_tokens) = first_table.get(symbol) {
                            possible_first_tokens.extend(new_possible_first_tokens.iter().cloned());
                        }
                    },
                    AtomicPattern::Token(token) => {
                        possible_first_tokens.insert(token.clone());
                    },
                }
                let new_possible_first_token_count = possible_first_tokens.len();

                if new_possible_first_token_count != old_possible_first_token_count {
                    done = false;
                }
                if new_possible_first_token_count > 0 {
                    match first_table.get_mut(symbol) {
                        Some(entry) => {
                            *entry = possible_first_tokens;
                        },
                        None => {
                            first_table.insert(symbol.clone(), possible_first_tokens);
                        },
                    }
                }
            }
        }

        FirstTable(first_table)
    }
}

impl Deref for FirstTable {
    type Target = IndexMap<Symbol, IndexSet<Token>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


/// Follow table of the symbols in a grammar.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Debug)]
pub struct FollowTable(IndexMap<Symbol, IndexSet<Token>>);

impl FollowTable {
    /// Constructs the follow table from the grammar.
    pub fn construct(grammar: &Grammar, first_table: &FirstTable) -> FollowTable {
        let mut follow_table =
            IndexMap::from([(grammar.start_symbol().clone(), IndexSet::from([Token::Eof]))]);

        let mut done = false;
        while !done {
            done = true;
            for rule in grammar.rules().iter() {
                for consecutive_atomic_patterns in rule.pattern().windows(2) {
                    let ap1 = &consecutive_atomic_patterns[0];
                    let ap2 = &consecutive_atomic_patterns[1];

                    let ap1_symbol = match ap1 {
                        AtomicPattern::Symbol(symbol) => symbol,
                        AtomicPattern::Token(_) => continue,
                    };

                    let mut possible_follow_tokens = match follow_table.get_mut(ap1_symbol) {
                        Some(entry) => std::mem::take(entry),
                        None => IndexSet::new(),
                    };

                    let old_possible_follow_token_count = possible_follow_tokens.len();
                    match ap2 {
                        AtomicPattern::Symbol(symbol) => {
                            if let Some(new_possible_follow_tokens) = first_table.get(symbol) {
                                possible_follow_tokens
                                    .extend(new_possible_follow_tokens.iter().cloned());
                            }
                        },
                        AtomicPattern::Token(token) => {
                            possible_follow_tokens.insert(token.clone());
                        },
                    }
                    let new_possible_follow_token_count = possible_follow_tokens.len();

                    if new_possible_follow_token_count != old_possible_follow_token_count {
                        done = false;
                    }

                    if new_possible_follow_token_count > 0 {
                        match follow_table.get_mut(ap1_symbol) {
                            Some(entry) => {
                                *entry = possible_follow_tokens;
                            },
                            None => {
                                follow_table.insert(ap1_symbol.clone(), possible_follow_tokens);
                            },
                        }
                    }
                }

                if let AtomicPattern::Symbol(last_ap) = rule.pattern().last().unwrap() {
                    let mut possible_follow_tokens = match follow_table.get_mut(last_ap) {
                        Some(entry) => std::mem::take(entry),
                        None => IndexSet::new(),
                    };

                    let old_possible_follow_token_count = possible_follow_tokens.len();
                    if let Some(new_possible_follow_tokens) = follow_table.get(rule.symbol()) {
                        possible_follow_tokens.extend(new_possible_follow_tokens.iter().cloned());
                    }
                    let new_possible_follow_token_count = possible_follow_tokens.len();

                    if new_possible_follow_token_count != old_possible_follow_token_count {
                        done = false;
                    }

                    if new_possible_follow_token_count > 0 {
                        match follow_table.get_mut(last_ap) {
                            Some(entry) => {
                                *entry = possible_follow_tokens;
                            },
                            None => {
                                follow_table.insert(last_ap.clone(), possible_follow_tokens);
                            },
                        }
                    }
                }
            }
        }

        FollowTable(follow_table)
    }
}

impl Deref for FollowTable {
    type Target = IndexMap<Symbol, IndexSet<Token>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


/// Action (e.g., `Shift 3`, `Reduce 2`, `Accept 1`) to perform during a parsing step.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    /// Shift the first remaining input token into symbol stack and transition to a new state.
    Shift {
        /// State to transition to.
        next_state: usize,
    },
    /// Apply a rule of the grammar to the symbol and state stacks, then goto a new state.
    Reduce {
        /// Index of the rule to reduce.
        rule_index: usize,
    },
    /// Accept the parse and finish parsing.
    Accept {
        /// Index of the rule that was matched.
        rule_index: usize,
    },
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Shift { next_state } => write!(f, "s{}", next_state),
            Action::Reduce { rule_index } => write!(f, "r{}", rule_index + 1),
            Action::Accept { rule_index } => write!(f, "a{}", rule_index + 1),
        }
    }
}


/// Action and goto tables of a parser.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Debug)]
pub struct ParsingTables {
    action_table: Vec<IndexMap<Token, IndexSet<Action>>>,
    goto_table: Vec<IndexMap<Symbol, usize>>,
}

impl ParsingTables {
    /// Constructs the parsing tables of the parser.
    pub fn construct(
        grammar: &Grammar,
        follow_table: &FollowTable,
        automaton: &Automaton,
    ) -> Result<ParsingTables, ParserError> {
        let mut action_table = Vec::with_capacity(automaton.states().len());
        let mut goto_table = Vec::with_capacity(automaton.states().len());

        for state in automaton.states().iter() {
            let mut actions = IndexMap::<Token, IndexSet<Action>>::new();
            let mut gotos = IndexMap::<Symbol, usize>::new();

            for item in state.items() {
                if item.dot() == item.rule().pattern().len() {
                    if let Some(follows) = follow_table.get(item.rule().symbol()) {
                        let rule_index =
                            grammar.rules().iter().position(|rule| rule == item.rule()).unwrap();
                        for token in follows {
                            if !item.lookahead().contains(token) {
                                continue;
                            }

                            if *token == Token::Eof
                                && item.rule().symbol() == grammar.start_symbol()
                            {
                                actions
                                    .entry(token.clone())
                                    .or_default()
                                    .insert(Action::Accept { rule_index });
                            } else {
                                actions
                                    .entry(token.clone())
                                    .or_default()
                                    .insert(Action::Reduce { rule_index });
                            }
                        }
                    }
                } else {
                    let next_atomic_pattern = &item.rule().pattern()[item.dot()];
                    let transition = state.transitions()[next_atomic_pattern];
                    match next_atomic_pattern {
                        AtomicPattern::Symbol(symbol) => {
                            gotos.insert(symbol.clone(), transition);
                        },
                        AtomicPattern::Token(token) => {
                            actions
                                .entry(token.clone())
                                .or_default()
                                .insert(Action::Shift { next_state: transition });
                        },
                    }
                }
            }

            action_table.push(actions);
            goto_table.push(gotos);
        }

        Ok(ParsingTables { action_table, goto_table })
    }
}

impl ParsingTables {
    /// Gets the action table of the parser.
    pub fn action_table(&self) -> &[IndexMap<Token, IndexSet<Action>>] {
        &self.action_table
    }

    /// Gets the goto table of the parser.
    pub fn goto_table(&self) -> &[IndexMap<Symbol, usize>] {
        &self.goto_table
    }
}

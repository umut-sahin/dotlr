use crate::prelude::*;


/// Item of a state of an LR(1) automaton.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    rule: Rule,
    dot: usize,
    lookahead: IndexSet<Token>,
}

impl Item {
    /// Creates a new item.
    pub fn new(rule: Rule, dot: usize, lookahead: impl Into<IndexSet<Token>>) -> Item {
        Item { rule, dot, lookahead: lookahead.into() }
    }
}

impl Item {
    /// Gets the rule of the item.
    pub fn rule(&self) -> &Rule {
        &self.rule
    }

    /// Gets the position of the dot of the item.
    pub fn dot(&self) -> usize {
        self.dot
    }

    /// Gets the lookahead set of the item.
    pub fn lookahead(&self) -> &IndexSet<Token> {
        &self.lookahead
    }
}

impl Item {
    /// Creates a new item by moving the dot to the right.
    pub fn advance(mut self) -> Item {
        self.dot += 1;
        self
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ->", self.rule.symbol())?;
        for (i, atomic_pattern) in self.rule.pattern().iter().enumerate() {
            if i == self.dot {
                write!(f, " .")?;
            }
            write!(f, " {}", atomic_pattern)?;
        }
        if self.dot == self.rule.pattern().len() {
            write!(f, " .")?;
        }
        Ok(())
    }
}


/// State of an LR(1) automaton.
#[derive(Debug, Default, Eq)]
pub struct State {
    id: usize,
    items: SmallVec<[Item; 2]>,
    transitions: IndexMap<AtomicPattern, usize>,
}

impl State {
    /// Creates a new state.
    pub fn new(
        id: usize,
        items: impl IntoIterator<Item = Item>,
        transitions: impl Into<IndexMap<AtomicPattern, usize>>,
    ) -> State {
        State { id, items: items.into_iter().collect(), transitions: transitions.into() }
    }
}

impl State {
    /// Gets the identifier of the state.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Gets the items of the state.
    pub fn items(&self) -> &SmallVec<[Item; 2]> {
        &self.items
    }

    /// Gets the transitions of the state.
    pub fn transitions(&self) -> &IndexMap<AtomicPattern, usize> {
        &self.transitions
    }
}

impl State {
    /// Computes the closure of the state.
    fn compute_closure(&mut self, grammar: &Grammar, first_table: &FirstTable) {
        loop {
            let mut new_items = vec![];
            for item in self.items.iter() {
                if item.dot == item.rule.pattern().len() {
                    continue;
                }
                if let AtomicPattern::Symbol(symbol) = &item.rule.pattern()[item.dot] {
                    let lookahead = if item.dot == item.rule.pattern().len() - 1 {
                        item.lookahead.clone()
                    } else {
                        let next_atomic_pattern = &item.rule.pattern()[item.dot + 1];
                        match next_atomic_pattern {
                            AtomicPattern::Symbol(symbol) => {
                                first_table.get(symbol).cloned().unwrap_or_default()
                            },
                            AtomicPattern::Token(token) => IndexSet::from([token.clone()]),
                        }
                    };
                    for rule in grammar.rules().iter().filter(|rule| rule.symbol() == symbol) {
                        let new_item =
                            Item { rule: rule.clone(), dot: 0, lookahead: lookahead.clone() };
                        if !self.items.contains(&new_item) {
                            new_items.push(new_item);
                        }
                    }
                }
            }

            let mut changed = false;
            for new_item in new_items {
                let mut already_exists = false;
                for existing_item in self.items.iter_mut() {
                    if new_item.dot == existing_item.dot && new_item.rule == existing_item.rule {
                        already_exists = true;
                        if !new_item.lookahead.is_subset(&existing_item.lookahead) {
                            changed = true;
                            for new_lookahead in new_item.lookahead.iter() {
                                existing_item.lookahead.insert(new_lookahead.clone());
                            }
                        }
                        break;
                    }
                }
                if !already_exists {
                    changed = true;
                    self.items.push(new_item);
                }
            }
            if !changed {
                break;
            }
        }
    }

    /// Computes the transitions of the state.
    fn compute_transitions(&self, state_counter: &mut usize) -> Vec<(AtomicPattern, State)> {
        let mut transitions = IndexMap::<AtomicPattern, State>::new();
        for item in self.items.iter() {
            if item.dot == item.rule.pattern().len() {
                continue;
            }

            let atomic_pattern_after_dot = &item.rule.pattern()[item.dot];
            let state_after_transition =
                transitions.entry(atomic_pattern_after_dot.clone()).or_insert_with(|| {
                    let id = *state_counter;
                    *state_counter += 1;
                    State { id, ..State::default() }
                });

            state_after_transition.items.push(item.clone().advance());
        }
        transitions.into_iter().collect()
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}


/// LR(1) automaton of a grammar.
#[derive(Debug)]
pub struct Automaton {
    states: Vec<State>,
}

impl Automaton {
    /// Constructs the LR(1) automaton of a grammar.
    pub fn construct(grammar: &Grammar, first_table: &FirstTable) -> Automaton {
        let first_state = State {
            id: 0,
            items: grammar
                .rules()
                .iter()
                .filter(|rule| rule.symbol() == grammar.start_symbol())
                .map(|rule| {
                    Item { rule: rule.clone(), dot: 0, lookahead: IndexSet::from([Token::Eof]) }
                })
                .collect(),
            transitions: IndexMap::new(),
        };

        let mut states_to_process = vec![first_state];
        let mut processed_states = BTreeMap::<usize, State>::new();

        let mut state_counter = 1;
        while let Some(mut state_to_process) = states_to_process.pop() {
            state_to_process.compute_closure(grammar, first_table);

            if let Some(existing_state_with_same_items) = processed_states
                .values()
                .find(|existing_state| existing_state.items == state_to_process.items)
            {
                let id_to_replace = state_to_process.id;
                let new_id = existing_state_with_same_items.id;

                for state in processed_states.values_mut() {
                    for transition_target in state.transitions.values_mut() {
                        if *transition_target == id_to_replace {
                            *transition_target = new_id;
                        }
                    }
                }
                continue;
            }

            let transitions = state_to_process.compute_transitions(&mut state_counter);
            for (pattern, state) in transitions {
                state_to_process.transitions.insert(pattern, state.id);
                states_to_process.push(state);
            }

            processed_states.insert(state_to_process.id, state_to_process);
        }

        let mut transition_map = IndexMap::new();
        let mut final_states = Vec::with_capacity(processed_states.len());

        for (id, mut state) in processed_states.into_values().enumerate() {
            transition_map.insert(state.id, id);
            state.id = id;
            final_states.push(state);
        }

        for state in final_states.iter_mut() {
            for transition_target in state.transitions.values_mut() {
                *transition_target = *transition_map.get(transition_target).unwrap();
            }
        }

        Automaton { states: final_states }
    }
}

impl Automaton {
    /// Gets the states of the automaton.
    pub fn states(&self) -> &[State] {
        &self.states
    }
}

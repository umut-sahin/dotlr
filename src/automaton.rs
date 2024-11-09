use crate::prelude::*;


/// Item of a state of an LR(1) or an LALR(1) automaton.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
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


/// State of an LR(1) or an LALR(1) automaton.
#[derive(Clone, Debug, Default, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
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
            if item.dot == item.rule.pattern().len() || item.rule().is_empty_pattern() {
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


/// LR(1) or LALR(1) automaton of a grammar.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
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
    /// Converts the LR(1) automaton into an LALR(1) automaton.
    pub fn to_lalr(self) -> Automaton {
        // We'll start by computing the states that share the same core.
        // Core of a state is its items without the lookahead.
        // In the end we want `state_groups` to be something like:
        // [
        //   { 0 },       -> New state 0 will be the copy of the original state 0
        //   { 1 },       -> New state 1 will be the copy of the original state 1
        //   { 2, 9 },    -> New state 2 will be the merge of the original states 2 and 9
        //   { 3, 6 },    -> New state 3 will be the merge of the original states 3 and 6
        //   { 4, 7 },    -> New state 4 will be the merge of the original states 4 and 7
        //   { 5, 8 },    -> New state 5 will be the merge of the original states 5 and 8
        //   { 10, 13 },  -> New state 6 will be the merge of the original states 10 and 13
        //   { 11, 14 },  -> New state 7 will be the merge of the original states 11 and 14
        //   { 12, 15 },  -> New state 8 will be the merge of the original states 12 and 15
        // ]
        let mut state_groups = Vec::<IndexSet<usize>>::new();
        for (state_index, state) in self.states.iter().enumerate() {
            let mut group = None;
            for state_group in state_groups.iter_mut() {
                assert!(!state_group.is_empty());

                let candidate_index = state_group.iter().next().unwrap();
                let candidate_state = &self.states[*candidate_index];

                if state.items.len() == candidate_state.items.len() {
                    let mut can_be_merged = true;
                    for item in state.items.iter() {
                        let mut candidate_state_has_same_item_without_lookahead = false;
                        for candidate_item in candidate_state.items.iter() {
                            if item.dot == candidate_item.dot && item.rule == candidate_item.rule {
                                candidate_state_has_same_item_without_lookahead = true;
                                break;
                            }
                        }
                        if !candidate_state_has_same_item_without_lookahead {
                            can_be_merged = false;
                            break;
                        }
                    }
                    if can_be_merged {
                        group = Some(state_group);
                    }
                }
            }
            match group {
                Some(group) => {
                    group.insert(state_index);
                },
                None => {
                    state_groups.push(IndexSet::from([state_index]));
                },
            }
        }

        // Now we'll compute the mapping from the old states to the new states.
        // In the end we want `state_map` to be something like:
        // {
        //      0: 0,  -> Original state  0 will become the new state 0
        //      1: 1,  -> Original state  1 will become the new state 1
        //      2: 2,  -> Original state  2 will become the new state 2
        //      3: 3,  -> Original state  3 will become the new state 3
        //      4: 4,  -> Original state  4 will become the new state 4
        //      5: 5,  -> Original state  5 will become the new state 5
        //      6: 3,  -> Original state  6 will become the new state 3
        //      7: 4,  -> Original state  7 will become the new state 4
        //      8: 5,  -> Original state  8 will become the new state 5
        //      9: 2,  -> Original state  9 will become the new state 2
        //     10: 6,  -> Original state 10 will become the new state 6
        //     11: 7,  -> Original state 11 will become the new state 7
        //     12: 8,  -> Original state 12 will become the new state 8
        //     13: 6,  -> Original state 13 will become the new state 6
        //     14: 7,  -> Original state 14 will become the new state 7
        //     15: 8,  -> Original state 15 will become the new state 8
        // }
        let mut state_map = BTreeMap::<usize, usize>::new();
        for (new_state_index, state_group) in state_groups.iter().enumerate() {
            for old_state_index in state_group.iter().copied() {
                state_map.insert(old_state_index, new_state_index);
            }
        }

        // Finally, we compute the new states.
        let mut new_states = Vec::<State>::with_capacity(state_groups.len());
        for (id, state_group) in state_groups.into_iter().enumerate() {
            // We'll create a new state for each group in `state_groups`.

            // We make sure that the group is not empty, which shouldn't happen.
            assert!(!state_group.is_empty());

            // Get an iterator of the indices of the states to merge.
            let mut state_indices = state_group.into_iter();

            // Create the new state from the first original state.
            let mut new_state = self.states[state_indices.next().unwrap()].clone();

            // Set the id of the state to the index of the group.
            new_state.id = id;

            // Update the transitions of the new state according to `state_map`.
            for next_state in new_state.transitions.values_mut() {
                *next_state = state_map[next_state];
            }

            // Merge the new state with other states in the group.
            for state_index in state_indices {
                // Get the state to merge.
                let state_to_merge = &self.states[state_index];

                // Make sure the state is merged into the correct state.
                assert_eq!(state_map[&state_to_merge.id], id);

                // Make sure the transitions of the state are the same as the new state.
                for (atomic_pattern, next_state) in state_to_merge.transitions.iter() {
                    assert!(new_state.transitions.contains_key(atomic_pattern));
                    assert_eq!(new_state.transitions[atomic_pattern], state_map[next_state])
                }

                // Extend the lookahead of the items of the new state.
                for item in state_to_merge.items.iter() {
                    let mut merged = false;
                    for new_item in new_state.items.iter_mut() {
                        if new_item.dot == item.dot && new_item.rule == item.rule {
                            new_item.lookahead.extend(item.lookahead.iter().cloned());
                            merged = true;
                            break;
                        }
                    }
                    // Make sure the item existed in both states.
                    assert!(merged);
                }
            }

            // Add the merged state to the new states.
            new_states.push(new_state);
        }

        // Crate the LALR(1) automaton using the new states.
        Automaton { states: new_states }
    }
}

impl Automaton {
    /// Gets the states of the automaton.
    pub fn states(&self) -> &[State] {
        &self.states
    }
}

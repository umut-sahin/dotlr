use crate::prelude::*;


/// Step of a parsing trace.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
pub struct Step<'i> {
    pub(crate) state_stack: Vec<usize>,
    pub(crate) tree_stack: Vec<Tree<'i>>,
    pub(crate) remaining_tokens: Vec<Spanned<Token>>,
    pub(crate) action_taken: Action,
}

impl<'i> Step<'i> {
    /// Gets the state stack during the step.
    pub fn state_stack(&self) -> &[usize] {
        &self.state_stack
    }

    /// Gets the tree stack during the step.
    pub fn tree_stack(&self) -> &[Tree<'i>] {
        &self.tree_stack
    }

    /// Gets the remaining tokens during the step.
    pub fn remaining_tokens(&self) -> &[Spanned<Token>] {
        &self.remaining_tokens
    }

    /// Gets the action taken in the step.
    pub fn action_taken(&self) -> &Action {
        &self.action_taken
    }
}


/// Trace of a parse.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
pub struct Trace<'i> {
    steps: Vec<Step<'i>>,
}

impl Trace<'_> {
    /// Creates a new trace.
    pub fn new() -> Self {
        Self { steps: vec![] }
    }
}

impl<'i> Trace<'i> {
    /// Adds a step to the trace.
    pub fn step(&mut self, step: Step<'i>) {
        self.steps.push(step);
    }
}

impl<'i> Trace<'i> {
    /// Gets the steps in the trace.
    pub fn steps(&self) -> &[Step<'i>] {
        &self.steps
    }
}

impl Trace<'_> {
    /// Dumps the trace to stdout.
    pub fn dump(&self, grammar: &Grammar) {
        let mut pretty_trace_table = Table::new();
        pretty_trace_table.add_row(row![
            cbFy->"Step",
            cbFy->"State Stack",
            cbFy->"Symbol Stack",
            cbFy->"Remaining Input",
            cbFy->"Action Taken",
        ]);
        for (i, step) in self.steps.iter().enumerate() {
            let state_stack = step.state_stack.iter().join(" ");
            let tree_stack = step
                .tree_stack
                .iter()
                .map(|tree| {
                    match tree {
                        Tree::Terminal { token, .. } => {
                            format_smolstr!("{}", token)
                        },
                        Tree::NonTerminal { symbol, .. } => {
                            format_smolstr!("{}", symbol)
                        },
                    }
                })
                .join(" ");
            let remaining_input = step.remaining_tokens.iter().rev().map(|t| t.value()).join(" ");
            let action_taken = match step.action_taken {
                Action::Shift { next_state } => {
                    format!("Shift {}", next_state)
                },
                Action::Reduce { rule_index } => {
                    format!("Reduce {} ({})", rule_index + 1, grammar.rules()[rule_index])
                },
                Action::Accept { rule_index } => {
                    format!("Accept {} ({})", rule_index + 1, grammar.rules()[rule_index])
                },
            };

            pretty_trace_table.add_row(row![
                i,
                state_stack,
                tree_stack,
                r->remaining_input,
                action_taken,
            ]);
        }
        pretty_trace_table.printstd();
    }
}

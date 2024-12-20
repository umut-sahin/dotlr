<div align="center">

  <h1>.lr</h1>
  <h5 style="font-weight: normal;">An LR(1) parser generator and visualizer created for educational purposes.</h5>

[![crates.io](https://img.shields.io/crates/v/dotlr)](https://crates.io/crates/dotlr)
[![docs.rs](https://img.shields.io/docsrs/dotlr)](https://docs.rs/dotlr)
[![ci](https://img.shields.io/github/actions/workflow/status/umut-sahin/dotlr/ci.yml)](https://github.com/umut-sahin/dotlr/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/dotlr)](https://crates.io/crates/dotlr)

  <hr/>

  <div>
    <img src="https://raw.githubusercontent.com/umut-sahin/dotlr/main/assets/docs/demo.png" />
  </div>

</div>

## Table of Contents

* [What is an LR(1) parser?](#what-is-an-lr1-parser)
* [Why did you make this?](#why-did-you-make-this)
* [How can I use the CLI in the picture?](#how-can-i-use-the-cli-in-the-picture)
* [Can I use it as a standalone library?](#can-i-use-it-as-a-standalone-library)
* [How it works?](#how-does-it-work)
  * [1) Parsing the grammar](#1-parsing-the-grammar)
  * [2) Computing FIRST sets](#2-computing-first-sets)
  * [3) Computing FOLLOW sets](#3-computing-follow-sets)
  * [4) Constructing the LR(1) automaton](#4-constructing-the-lr1-automaton)
  * [5) Constructing ACTION and GOTO tables](#5-constructing-action-and-goto-tables)
  * [6) Tokenizing the input](#6-tokenizing-the-input)
  * [7) Parsing the tokens](#7-parsing-the-tokens)
* [Can I have symbols that can match to empty string?](#can-i-have-symbols-that-can-match-to-empty-string)
* [Can I have an LALR(1) parser instead of an LR(1) parser?](#can-i-have-an-lalr1-parser-instead-of-an-lr1-parser)
* [Any benchmarks?](#any-benchmarks)
* [Can I modify it?](#can-i-modify-it)
* [Which resources did you use when creating this?](#which-resources-did-you-use-when-creating-this)
* [How is it licensed?](#how-is-it-licensed)

## What is an LR(1) parser?

[LR(1) parser](https://en.wikipedia.org/wiki/Canonical_LR_parser) is a type of
[bottom-up parser](https://en.wikipedia.org/wiki/Bottom-up_parsing) for parsing a subset of
[context free languages](https://en.wikipedia.org/wiki/Context-free_language).

`1` indicates that the parser will use a single lookahead token when making parsing decisions.

LR(1) parsers are powerful because they can parse a wide range of context free languages,
including most programming languages!

## Why did you make this?

To learn and to help others learn! This project allows you to visualize LR(1) parsers, from
construction to parsing, which makes it easier to understand how they work.

## How can I use the CLI in the picture?

If you want to play around with `dotlr` to visualize parser construction and step-by-step parsing of
different grammars, you can use the main executable of the crate.

### Installation

You can install the `dotlr` cli from [crates.io](https://crates.io/crates/dotlr).

```shell
cargo install dotlr
```

### Usage

Paste the following into a file called `grammar.lr`:

```
P -> E

E -> E '+' T
E -> T

T -> %id '(' E ')'
T -> %id

%id -> /[A-Za-z][A-Za-z0-9]+/
```

Run `dotlr` cli with the grammar file and an input:

```shell
dotlr grammar.lr "foo(bar + baz)"
```

It'll print:

```
+--------------------------------+
|            Grammar             |
+--------------------------------+
| P -> E                         |
| E -> E '+' T                   |
| E -> T                         |
| T -> %id '(' E ')'             |
| T -> %id                       |
|                                |
| %id -> /^[A-Za-z][A-Za-z0-9]+/ |
+--------------------------------+
+--------+-----------+-----------------+
| Symbol | First Set |   Follow Set    |
+--------+-----------+-----------------+
| T      | { %id }   | { $, '+', ')' } |
+--------+-----------+-----------------+
| E      | { %id }   | { $, '+', ')' } |
+--------+-----------+-----------------+
| P      | { %id }   | { $ }           |
+--------+-----------+-----------------+
+-------+------------------------+--------------+---------------+
| State |         Items          |  Lookaheads  |  Transitions  |
+-------+------------------------+--------------+---------------+
| 0     |  P -> . E              | { $ }        |   E   ->  1   |
|       |  E -> . E '+' T        | { $, '+' }   |   T   ->  2   |
|       |  E -> . T              | { $, '+' }   |  %id  ->  3   |
|       |  T -> . %id '(' E ')'  | { $, '+' }   |               |
|       |  T -> . %id            | { $, '+' }   |               |
+-------+------------------------+--------------+---------------+
| 1     |  P -> E .              | { $ }        |  '+'  ->  14  |
|       |  E -> E . '+' T        | { $, '+' }   |               |
+-------+------------------------+--------------+---------------+
| 2     |  E -> T .              | { $, '+' }   |               |
+-------+------------------------+--------------+---------------+
| 3     |  T -> %id . '(' E ')'  | { $, '+' }   |  '('  ->  4   |
|       |  T -> %id .            | { $, '+' }   |               |
+-------+------------------------+--------------+---------------+
| 4     |  T -> %id '(' . E ')'  | { $, '+' }   |   E   ->  5   |
|       |  E -> . E '+' T        | { ')', '+' } |  %id  ->  6   |
|       |  E -> . T              | { ')', '+' } |   T   ->  9   |
|       |  T -> . %id '(' E ')'  | { ')', '+' } |               |
|       |  T -> . %id            | { ')', '+' } |               |
+-------+------------------------+--------------+---------------+
| 5     |  T -> %id '(' E . ')'  | { $, '+' }   |  '+'  ->  11  |
|       |  E -> E . '+' T        | { ')', '+' } |  ')'  ->  13  |
+-------+------------------------+--------------+---------------+
| 6     |  T -> %id . '(' E ')'  | { ')', '+' } |  '('  ->  7   |
|       |  T -> %id .            | { ')', '+' } |               |
+-------+------------------------+--------------+---------------+
| 7     |  T -> %id '(' . E ')'  | { ')', '+' } |  %id  ->  6   |
|       |  E -> . E '+' T        | { ')', '+' } |   E   ->  8   |
|       |  E -> . T              | { ')', '+' } |   T   ->  9   |
|       |  T -> . %id '(' E ')'  | { ')', '+' } |               |
|       |  T -> . %id            | { ')', '+' } |               |
+-------+------------------------+--------------+---------------+
| 8     |  T -> %id '(' E . ')'  | { ')', '+' } |  ')'  ->  10  |
|       |  E -> E . '+' T        | { ')', '+' } |  '+'  ->  11  |
+-------+------------------------+--------------+---------------+
| 9     |  E -> T .              | { ')', '+' } |               |
+-------+------------------------+--------------+---------------+
| 10    |  T -> %id '(' E ')' .  | { ')', '+' } |               |
+-------+------------------------+--------------+---------------+
| 11    |  E -> E '+' . T        | { ')', '+' } |  %id  ->  6   |
|       |  T -> . %id '(' E ')'  | { ')', '+' } |   T   ->  12  |
|       |  T -> . %id            | { ')', '+' } |               |
+-------+------------------------+--------------+---------------+
| 12    |  E -> E '+' T .        | { ')', '+' } |               |
+-------+------------------------+--------------+---------------+
| 13    |  T -> %id '(' E ')' .  | { $, '+' }   |               |
+-------+------------------------+--------------+---------------+
| 14    |  E -> E '+' . T        | { $, '+' }   |  %id  ->  3   |
|       |  T -> . %id '(' E ')'  | { $, '+' }   |   T   ->  15  |
|       |  T -> . %id            | { $, '+' }   |               |
+-------+------------------------+--------------+---------------+
| 15    |  E -> E '+' T .        | { $, '+' }   |               |
+-------+------------------------+--------------+---------------+
+-------+---------------------------------------+----------------------+
|       |                Action                 |         Goto         |
| State | ------------------------------------- | -------------------- |
|       |    '+'    '('    ')'    %id     $     |    P     E     T     |
+-------+---------------------------------------+----------------------+
| 0     |     -      -      -     s3      -     |    -     1     2     |
+-------+---------------------------------------+----------------------+
| 1     |    s14     -      -      -     a1     |    -     -     -     |
+-------+---------------------------------------+----------------------+
| 2     |    r3      -      -      -     r3     |    -     -     -     |
+-------+---------------------------------------+----------------------+
| 3     |    r5     s4      -      -     r5     |    -     -     -     |
+-------+---------------------------------------+----------------------+
| 4     |     -      -      -     s6      -     |    -     5     9     |
+-------+---------------------------------------+----------------------+
| 5     |    s11     -     s13     -      -     |    -     -     -     |
+-------+---------------------------------------+----------------------+
| 6     |    r5     s7     r5      -      -     |    -     -     -     |
+-------+---------------------------------------+----------------------+
| 7     |     -      -      -     s6      -     |    -     8     9     |
+-------+---------------------------------------+----------------------+
| 8     |    s11     -     s10     -      -     |    -     -     -     |
+-------+---------------------------------------+----------------------+
| 9     |    r3      -     r3      -      -     |    -     -     -     |
+-------+---------------------------------------+----------------------+
| 10    |    r4      -     r4      -      -     |    -     -     -     |
+-------+---------------------------------------+----------------------+
| 11    |     -      -      -     s6      -     |    -     -     12    |
+-------+---------------------------------------+----------------------+
| 12    |    r2      -     r2      -      -     |    -     -     -     |
+-------+---------------------------------------+----------------------+
| 13    |    r4      -      -      -     r4     |    -     -     -     |
+-------+---------------------------------------+----------------------+
| 14    |     -      -      -     s3      -     |    -     -     15    |
+-------+---------------------------------------+----------------------+
| 15    |    r2      -      -      -     r2     |    -     -     -     |
+-------+---------------------------------------+----------------------+

> foo(bar + baz)

P
└─ E
   └─ T
      ├─ foo
      ├─ (
      ├─ E
      │  ├─ E
      │  │  └─ T
      │  │     └─ bar
      │  ├─ +
      │  └─ T
      │     └─ baz
      └─ )

+------+---------------+-------------------+---------------------------+-------------------------------+
| Step |  State Stack  |   Symbol Stack    |      Remaining Input      |         Action Taken          |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 0    | 0             |                   | %id '(' %id '+' %id ')' $ | Shift 3                       |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 1    | 0 3           | %id               |     '(' %id '+' %id ')' $ | Shift 4                       |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 2    | 0 3 4         | %id '('           |         %id '+' %id ')' $ | Shift 6                       |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 3    | 0 3 4 6       | %id '(' %id       |             '+' %id ')' $ | Reduce 4 (T -> %id)           |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 4    | 0 3 4 9       | %id '(' T         |             '+' %id ')' $ | Reduce 2 (E -> T)             |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 5    | 0 3 4 5       | %id '(' E         |             '+' %id ')' $ | Shift 11                      |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 6    | 0 3 4 5 11    | %id '(' E '+'     |                 %id ')' $ | Shift 6                       |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 7    | 0 3 4 5 11 6  | %id '(' E '+' %id |                     ')' $ | Reduce 4 (T -> %id)           |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 8    | 0 3 4 5 11 12 | %id '(' E '+' T   |                     ')' $ | Reduce 1 (E -> E '+' T)       |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 9    | 0 3 4 5       | %id '(' E         |                     ')' $ | Shift 13                      |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 10   | 0 3 4 5 13    | %id '(' E ')'     |                         $ | Reduce 3 (T -> %id '(' E ')') |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 11   | 0 2           | T                 |                         $ | Reduce 2 (E -> T)             |
+------+---------------+-------------------+---------------------------+-------------------------------+
| 12   | 0 1           | E                 |                         $ | Accept 0 (P -> E)             |
+------+---------------+-------------------+---------------------------+-------------------------------+
```

You can also enter REPL mode if you omit the input:

```shell
dotlr grammar.lr
```

## Can I use it as a standalone library?

Yes, you can depend on the `dotlr` crate from [crates.io](https://crates.io/crates/dotlr).

### Setup

Paste the following to your `dependencies` section of your `Cargo.toml`:

```toml
dotlr = { version = "0.4", default-features = false }
```

### Example

Here is a basic example that shows the primary operations of the `dotlr` crate:

```rust
use dotlr::{
  Grammar,
  Parser,
  ParserError,
};

const GRAMMAR: &str = r#"

P -> E

E -> E '+' T
E -> T

T -> %id '(' E ')'
T -> %id

%id -> /[A-Za-z][A-Za-z0-9]+/

"#;

const INPUT: &str = r#"

foo(bar + baz)

"#;

fn main() {
  let grammar = match Grammar::parse(GRAMMAR.trim()) {
    Ok(grammar) => grammar,
    Err(error) => {
      eprintln!("grammar error: {}", error);
      return;
    }
  };
  let parser = match Parser::lr(grammar) {
    Ok(parser) => parser,
    Err(error) => {
      eprintln!("parser error: {}", error);
      if let ParserError::Conflict { parser, .. } = error {
        parser.dump();
      }
      return;
    }
  };
  let tokens = match parser.tokenize(INPUT.trim()) {
    Ok(tokens) => tokens,
    Err(error) => {
      eprintln!("tokenization error: {}", error);
      return;
    }
  };
  let (parse_trace, parse_tree) = match parser.trace(tokens) {
    Ok(result) => result,
    Err(error) => {
      eprintln!("tokenization error: {}", error);
      return;
    }
  };

  parser.dump();
  println!();
  parse_tree.dump();
  println!();
  parse_trace.dump(parser.grammar());
}
```

## How does it work?

Let's go over a step-by-step construction of the parser for the following grammar:

```
E -> E '+' F
E -> F

F -> F '*' T
F -> T

T -> %b

%b -> /[0-1]/
```

And then do a step-by-step explanation of the parsing steps for the following input:

```
1 + 0 * 1
```

Few notes before starting:

- `$` represents the end of input token
- `ε` represents the empty token

### 1) Parsing the grammar

First, we need to parse the grammar string to a grammar object that we can work with.

The grammar object will consist of:

- **symbols (HashSet<Symbol>):**
  - The set of symbols in the grammar \
    (e.g., `{ E, F, T }`)

- **start_symbol (Symbol):**
  - The symbol to parse \
    (e.g., `E`)

- **empty_symbols (HashSet<Symbol>):**
  - The set of symbols that can expand to empty string (i.e., `∀S: S -> '' ∈ rules`) \
    (e.g., `T`)

- **constant_tokens (HashSet<ConstantToken>):**
  - The set of constant tokens in the grammar \
    (e.g., `{ '+', '*' }`)

- **regular_expressions (HashMap<RegexToken, Regex>):**
  - The map of regular expression tokens to their corresponding compiled regular expressions \
    (e.g., `{ b -> /[0-1]/ }`)

- **rules (Vec<Rule>):**
  - The list of rules in the grammar \
    (e.g., `[ E -> E '+' F, E -> F, ... ]`)

This is done in [src/grammar.rs](https://github.com/umut-sahin/dotlr/blob/main/src/grammar.rs) with
a simple handwritten parser.

### 2) Computing FIRST sets

Now, we need to compute a set of tokens for each symbol in the grammar according to the
following constraints:

- For each `token ∈ FIRST(Symbol)`, at least one of the following conditions must hold:
  - `Symbol -> '' ∈ grammar.rules`
    **and** `token == ε`

  - `Symbol -> (EmptySymbol1 ... EmptySymbolN) ∈ grammar.rules`
    **and** `token == ε`

  - `Symbol -> (EmptySymbol1 ... EmptySymbolN) token ... ∈ grammar.rules`

  - `Symbol -> (EmptySymbol1 ... EmptySymbolN) NonEmptySymbol ... ∈ grammar.rules`
    **and** `token ∈ FIRST(NonEmptySymbol)`

As for the implementation, here is a python-like pseudocode of the algorithm to compute FIRST sets:

```python
# Start with FIRST(all_symbols) = {}
first_sets = {}

# Iterate until no more changes
while first_sets.has_changed():
    # Iterate over the rules of the grammar
    for rule in grammar:
        # If rule is empty pattern
        if rule.is_empty_pattern:
            # S -> '' <==> S can start with ε
            # -------------------------------
            # Add ε to the FIRST set of the symbol of the rule
            first_sets[rule.symbol].add(ε)
            # Break the processing of the rule as it's fully processed
            break

        # Iterate over the atoms of the pattern of the rule
        for atom in rule.pattern:
            # If atom is a token
            if atom.is_token:
                # S -> (EmptySymbol1 ... EmptySymbolN) '+' ... <==> S can start with '+'
                # ----------------------------------------------------------------------
                # Add the matching token to the FIRST set of the symbol of the rule
                first_sets[rule.symbol].add(atom)
                # Break the processing of the rule
                # as the rule cannot provide any other first tokens
                break

            # If atom is a symbol
            elif atom.is_symbol:
                # S -> (EmptySymbol1 ... EmptySymbolN) E ... <==> S can start with anything E can start with
                # ------------------------------------------------------------------------------------------
                # Add every token in the FIRST set of the matching symbol
                # to the FIRST set of the symbol except ε as it's treated
                # in a special way through the loop
                first_sets[rule.symbol].extend(first_sets[atom].exclude(ε))
                # If the symbol cannot be empty (E -> '' ∉ grammar.rules)
                if not atom.can_be_empty:
                    # Break the processing of the rule
                    # as the rule cannot provide any other first tokens
                    break

        # If the loop is not broken manually
        else:
            # S -> EmptySymbol1 ... EmptySymbolN <==> S can start with 'ε'
            # -------------------------------------------------------------
            # Add ε to the FIRST set of the symbol of the rule
            first_sets[rule.symbol].add(ε)
```

This is done in [src/tables.rs](https://github.com/umut-sahin/dotlr/blob/main/src/tables.rs).

FIRST sets of the example grammar:

```
+--------+-----------+
| Symbol | First Set |
+--------+-----------+
| T      | { %b }    |
+--------+-----------+
| F      | { %b }    |
+--------+-----------+
| E      | { %b }    |
+--------+-----------+
```

### 3) Computing FOLLOW sets

Next, we need to compute another set of tokens for each symbol in the grammar according the
following constraints:

- For each `token ∈ FOLLOW(Symbol)`, at least one of the following conditions must hold:
  - `Symbol == grammar.start_symbol`
    **and** `token == $`

  - `Anything -> ... Symbol (EmptySymbol1 ... EmptySymbolN) token ... ∈ grammar.rules`

  - `Anything -> ... Symbol (EmptySymbol1 ... EmptySymbolN) AnotherSymbol ... ∈ grammar.rules`
    **and** `token ∈ FIRST(AnotherSymbol)`
    **and** `token != ε`

  - `AnotherSymbol -> ... Symbol (EmptySymbol1 ... EmptySymbolN) ∈ grammar.rules`
    **and** `token ∈ FOLLOW(AnotherSymbol)`

  - `Symbol -> ... AnotherSymbol ∈ grammar.rules`
    **and** `token ∈ FOLLOW(AnotherSymbol)`

As for the implementation, here is a python-like pseudocode of the algorithm to compute FOLLOW sets:

```python
# Start with just FOLLOW(grammar.start_symbol) = { $ }
follow_sets = { grammar.start_symbol: { $ } }

# Iterate until no more changes
while follow_sets.has_changed():
    # Iterate over the rules of the grammar
    for rule in grammar:
        # Iterate over the atoms of the pattern of the rule
        for i in range(len(rule.pattern) - 1):
            # If the atom is a symbol
            if rule.pattern[i].is_symbol:
                # Iterate over the remaining atoms
                for j in range(i + 1, len(rule.pattern)):
                    # If the next atom is a token
                    if rule.pattern[j].is_token:
                        # S -> ... E (EmptySymbol1 ... EmptySymbolN) '+' ... <==> E can follow '+'
                        # ------------------------------------------------------------------------
                        # Add the matching token to the FOLLOW set of the matching symbol
                        follow_sets[rule.pattern[i]].add(rule.pattern[j])
                        # Break the processing of the rule
                        # as the rule cannot provide any other follow tokens
                        break

                    # Or if the next atom is a symbol
                    elif rule.pattern[j].is_symbol:
                        # S -> ... E (EmptySymbol1 ... EmptySymbolN) F ... <==> E can follow anything F can start with
                        # --------------------------------------------------------------------------------------------
                        # Add every token in the FIRST set of the second symbol
                        # to the FOLLOW set of the first symbol
                        # except ε as it's shouldn't be in the follow set
                        follow_sets[rule.pattern[i]].extend(first_sets[rule.pattern[j]].exclude(ε))
                        # If the symbol cannot be empty (F -> '' ∉ grammar.rules)
                        if not rule.pattern[j].can_be_empty:
                            # Break the processing of the rule
                            # as the rule cannot provide any other follow tokens
                            break

                # If the loop is not broken manually
                else:
                    # A -> ... S (EmptySymbol1 ... EmptySymbolN) <==> S can follow anything A can follow
                    # ----------------------------------------------------------------------------------
                    # Add every token in the FOLLOW set of the symbol of the rule
                    # to the FOLLOW set of the symbol of the atom
                    follow_sets[rule.patten[i]].extend(follow_sets[rule.symbol])

        # If pattern of ends with a symbol
        if rule.pattern[-1].is_symbol:
            # S -> ... E <==> S can follow anything E can follow
            # --------------------------------------------------
            # Add every token in the FOLLOW set of the symbol of the last atom
            # to the FOLLOW set of the symbol of the rule
            follow_sets[rule.symbol].extend(follow_sets[rule.patten[-1]])
```

This is done in [src/tables.rs](https://github.com/umut-sahin/dotlr/blob/main/src/tables.rs).

FOLLOW sets of the example grammar:

```
+--------+-----------------+
| Symbol |   Follow Set    |
+--------+-----------------+
| T      | { $, '+', '*' } |
+--------+-----------------+
| F      | { $, '+', '*' } |
+--------+-----------------+
| E      | { $, '+' }      |
+--------+-----------------+
```

### 4) Constructing the LR(1) automaton

It's time to construct the LR(1) automaton for the grammar.

Here is the drawing of the LR(1) automaton of a different grammar
to get an intuition of what an LR(1) automaton is:

<div>
  <img src="https://raw.githubusercontent.com/umut-sahin/dotlr/main/assets/docs/automaton.png" />
</div>

An automaton object is just of a list of states.

Each state has:

- **id (usize):**
  - The identifier of the state \
    (e.g., `0`, `1`)

- **items (Vec<Item>):**
  - The list of LR(1) items of the state \
    (e.g., `E -> E . '+' F | { $, '+' }`)

- **transitions (HashMap<AtomicPattern, usize>):**
  - The map of atomic patterns that would make transition to a new state \
    (e.g., `{ '+' -> 7 }`, ` { %b  ->  4, T   ->  6 } `)

Each item has:

- **rule (Rule):**
  - Underlying rule of the item \
    (e.g., `E -> E '+' F` in `E -> E . '+' F | { $, '+' }`)

- **dot (usize):**
  - Position of the dot in the rule \
    (e.g., `1` in `E -> E . '+' F | { $, '+' }`)

- **lookahead (HashSet<Token>):**
  - The set of tokens that could follow after the rule is applied.
    Keep in mind that, one of the lookahead tokens MUST follow for the rule to be applied. \
    (e.g., `{ $, '+' }` in `E -> E . '+' F | { $, '+' }`)

As for the implementation, here is a python-like pseudocode of the algorithm to construct the LR(1)
automaton:

```python
# Setup the kernel of the first state
first_state = next_empty_state()
for rule in grammar.rules:
    if rule.symbol == grammar.start_symbol:
        first_state.add_item(Item(rule, dot=0, lookahead={$}))

# Initialize construction state
states_to_process = [first_state]
processed_states = []

# Iterate until there aren't any more states to process.
while len(states_to_process) > 0:
    # Get the state to process
    state_to_process = states_to_process.pop()

    # Computing closure of the state to process
    # Loop until no more items are added
    while True:
        new_items = []
        # Iterate current items to obtain new items
        for item in state_to_process.items:
            # If dot is not at the end of the pattern
            if item.dot != len(item.rule.pattern):
                # And if there is a symbol after dot
                if item.rule.pattern[item.dot].is_symbol:
                    # Compute the lookahead for the new item
                    if item.dot == len(item.rule.pattern) - 1:
                        # S -> ... . E <==> Tokens in the current lookahead can follow E
                        # --------------------------------------------------------------
                        lookahead = item.lookahead
                    elif item.rule.pattern[item.dot + 1].is_token:
                        # S -> ... . E '+' <==> '+' can follow E
                        # --------------------------------------
                        lookahead = {item.rule.pattern[item.dot + 1]}
                    elif item.rule.pattern[item.dot + 1].is_symbol:
                        # S -> ... . E F <==> Tokens in FIRST(F) can follow E
                        # ---------------------------------------------------
                        lookahead = first_sets[item.rule.pattern[item.dot + 1]]

                    # Iterate over the rules of the grammar
                    for rule in grammar.rules:
                        # If the rule is for the symbol after dot
                        if rule.symbol == item.rule.pattern[item.dot]:
                            # Create a new item from the rule, with dot at the beginning
                            new_item = Item(rule, dot=0, lookahead=lookahead)
                            # If the item is not already in items of the state
                            if new_item not in state_to_process.items:
                                # Add it the set of new items
                                new_items.push(new_item)

        # Process new items
        for new_item in new_items:
            # If a similar item with the same rule and the same dot but a different lookahead exists
            if state_to_process.has_same_base_item(new_item):
                # Merge lookaheads
                state_to_process.merge_base_items(new_item)

            # Otherwise
            else:
                # Add the new item directly
                state_to_process.items.add(new_item)

        # If state hasn't changed, break the loop
        if not state_to_process.items.has_changed():
            break

    # Merge the states to process with an already existing state with the same closure.
    replaced = False
    for existing_state_with_same_items in processed_states:
        if existing_state_with_same_items.items == state_to_process.items:
            replaced = True
            for state in processed_states:
                # Replace transitions from existing states to point to the correct state.
                state.transitions.replace_value(state_to_process, existing_state_with_same_items)
            break
    if replaced:
        # If it's merged with an existing state, there is nothing else to do
        continue

    # Compute transitions of from the state to process.
    for item in state_to_process.items:
        # If the rule of the items is empty pattern
        if item.rule.is_empty_pattern:
            # S -> . ε <==> No need to create a transition to item  S -> ε .
            # --------------------------------------------------------------
            continue

        # If dot is not at the end
        if item.dot == len(item.rule.pattern):
            # S -> ... . <==> Can't create a transition as the dot is already at the end
            # --------------------------------------------------------------------------
            continue

        # S -> ... . E ... <==> Seeing E would cause a transition to another state
        # S -> ... . '+' ... <==> Seeing '+' would cause a transition to another state
        # ----------------------------------------------------------------------------
        atomic_pattern_after_dot = item.rule.pattern[item.dot]

        # If state to transition is not created yet, create an empty state for it.
        if atomic_pattern_after_dot is not in transitions:
            # Create an empty state to transition to
            state_to_process.transitions[atomic_pattern_after_dot] = next_empty_state()

        # Setup the kernel of the state to transition
        state_to_transition = state_to_process.transitions[atomic_pattern_after_dot]
        state_to_transition.items.push(item.shift_dot_to_right())

    # Add state to process to processed states, as we're done with it
    processed_states.push(state_to_process)
```

This is done in [src/automaton.rs](https://github.com/umut-sahin/dotlr/blob/main/src/automaton.rs).

The LR(1) automaton of the example grammar:

```
+-------+------------------+-----------------+--------------+
| State |      Items       |   Lookaheads    | Transitions  |
+-------+------------------+-----------------+--------------+
| 0     |  E -> . E '+' F  | { $, '+' }      |  E   ->  1   |
|       |  E -> . F        | { $, '+' }      |  F   ->  2   |
|       |  F -> . F '*' T  | { $, '+', '*' } |  T   ->  3   |
|       |  F -> . T        | { $, '+', '*' } |  %b  ->  4   |
|       |  T -> . %b       | { $, '+', '*' } |              |
+-------+------------------+-----------------+--------------+
| 1     |  E -> E . '+' F  | { $, '+' }      |  '+'  ->  7  |
+-------+------------------+-----------------+--------------+
| 2     |  E -> F .        | { $, '+' }      |  '*'  ->  5  |
|       |  F -> F . '*' T  | { $, '+', '*' } |              |
+-------+------------------+-----------------+--------------+
| 3     |  F -> T .        | { $, '+', '*' } |              |
+-------+------------------+-----------------+--------------+
| 4     |  T -> %b .       | { $, '+', '*' } |              |
+-------+------------------+-----------------+--------------+
| 5     |  F -> F '*' . T  | { $, '+', '*' } |  %b  ->  4   |
|       |  T -> . %b       | { $, '+', '*' } |  T   ->  6   |
+-------+------------------+-----------------+--------------+
| 6     |  F -> F '*' T .  | { $, '+', '*' } |              |
+-------+------------------+-----------------+--------------+
| 7     |  E -> E '+' . F  | { $, '+' }      |  T   ->  3   |
|       |  F -> . F '*' T  | { $, '+', '*' } |  %b  ->  4   |
|       |  F -> . T        | { $, '+', '*' } |  F   ->  8   |
|       |  T -> . %b       | { $, '+', '*' } |              |
+-------+------------------+-----------------+--------------+
| 8     |  E -> E '+' F .  | { $, '+' }      |  '*'  ->  5  |
|       |  F -> F . '*' T  | { $, '+', '*' } |              |
+-------+------------------+-----------------+--------------+
```

### 5) Constructing ACTION and GOTO tables

Finally, we can compute ACTION and GOTO tables of the parser according the following constraints:

- For each `action ∈ ACTION(state, token)`, at least one of the following conditions must hold:
  - `Anything -> ... . token ... | lookahead ∈ state.items` **and** \
    `action == Shift(state.transitions[token])`

  - `Anything -> ... . | lookahead ∈ state.items` **and** \
    `token ∈ lookahead` **and** \
    `token == $` **and** \
    `item.rule.symbol == grammar.start_symbol` **and** \
    `action == Accept(item.rule)`

  - `Anything -> . ε | lookahead ∈ state.items` **and** \
    `token ∈ lookahead` **and** \
    `token == $` **and** \
    `item.rule.symbol == grammar.start_symbol` **and** \
    `action == Accept(item.rule)`

  - `Anything -> ... . | lookahead ∈ state.items` **and** \
    `token ∈ lookahead` **and** \
    (`token != $` **or** `item.rule.symbol != grammar.start_symbol`) **and** \
    `action == Reduce(item.rule)`

  - `Anything -> . ε | lookahead ∈ state.items` **and** \
    `token ∈ lookahead` **and** \
    (`token != $` **or** `item.rule.symbol != grammar.start_symbol`) **and** \
    `action == Reduce(item.rule)`

- For each `goto ∈ GOTO(state, Symbol)`, at least one of the following conditions must hold:
  - `Anything -> ... . Symbol ... | lookahead ∈ state.items` **and** \
    `goto == state.transitions[Symbol]`

As for the implementation, here is a python-like pseudocode of the algorithm to construct ACTION and
GOTO tables:

```python
# Initialize empty action tables
action_table = {}
goto_table = {}

# For each state in the automaton
for state in automaton.states:
    # Iterate over the items of the state
    for item in state.items:
        # If dot is at the end of the item or rule of the item is empty pattern
        if item.dot == len(item.rule.pattern) or item.rule.is_empty_pattern:
            # S -> ... . <==> We can either reduce the rule or accept if S is a start symbol
            # S -> . ε <==> We can either reduce the rule or accept if S is a start symbol
            # ------------------------------------------------------------------------------

            # We can only perform actions for the tokens in the follow set of the symbol of the rule
            for following_token in follow_sets[item.rule.symbol]:
                # And only if the token is also in the lookahead of the item
                if following_token in item.lookahead:
                    # Finally, if the token is $ and matching rule is a start symbol
                    if following_token == $ and item.rule.symbol == grammar.start_symbol:
                        # We should accept
                        action_table[state, following_token].push(Accept(item.rule))
                    # Otherwise
                    else:
                        # We should reduce the matching rule
                        action_table[state, following_token].push(Reduce(item.rule))
        else:
            # We get the last atomic pattern
            atomic_pattern_after_dot = item.rule.pattern[item.dot]

            # And the transition on the atomic pattern from the automaton
            transition = state.transitions[atomic_pattern_after_dot]

            if atomic_pattern_after_dot.is_token:
                # S -> ... . '+' ... <==> We should shift and transition to another state
                #                         if the dot precedes a token
                # -------------------------------------------------------------------
                action_table[state, atomic_pattern_after_dot].push(Shift(transition))

            elif atomic_pattern_after_dot.is_symbol:
                # S -> ... . E ... <==> We should update GOTO table if the dot precedes a symbol
                #                       This can only happen after a reduction, and the parser
                #                       would look to GOTO table to determine the next state
                # --------------------------------------------------------------------------------
                goto_table[state, atomic_pattern_after_dot] = transition
```

This is done in [src/tables.rs](https://github.com/umut-sahin/dotlr/blob/main/src/tables.rs).

ACTION and GOTO tables of the example grammar:

```
+-------+-------------------------------+-------------------+
|       |            Action             |       Goto        |
| State | ----------------------------- | ----------------- |
|       |    '+'    '*'    %b     $     |    E    F    T    |
+-------+-------------------------------+-------------------+
| 0     |     -      -     s4     -     |    1    2    3    |
+-------+-------------------------------+-------------------+
| 1     |    s7      -     -      -     |    -    -    -    |
+-------+-------------------------------+-------------------+
| 2     |    r2     s5     -     a2     |    -    -    -    |
+-------+-------------------------------+-------------------+
| 3     |    r4     r4     -     r4     |    -    -    -    |
+-------+-------------------------------+-------------------+
| 4     |    r5     r5     -     r5     |    -    -    -    |
+-------+-------------------------------+-------------------+
| 5     |     -      -     s4     -     |    -    -    6    |
+-------+-------------------------------+-------------------+
| 6     |    r3     r3     -     r3     |    -    -    -    |
+-------+-------------------------------+-------------------+
| 7     |     -      -     s4     -     |    -    8    3    |
+-------+-------------------------------+-------------------+
| 8     |    r1     s5     -     a1     |    -    -    -    |
+-------+-------------------------------+-------------------+
```

### 6) Tokenizing the input

Tokenization algorithm in `dotlr` is the simplest tokenization algorithm thinkable.

Here is the idea in a python-like pseudocode:

```python
# Initialize the result
tokens = []

# Loop until all of the input is consumed
remaining_input = input.trim();
while len(remaining_input) > 0:
    # Try to match regular expression tokens
    for (regex_token, regex) in grammar.regular_expressions:
        if match := regex.start_matches(remaining_input):
            # We have a match so add it to result
            tokens.push(regex_token)
            # And shrink remaining input
            remaining_input = remaining_input[match.end:].trim()
            break

    # No regular expression tokens matched
    else:
        # So try to match constant tokens
        for constant_token in grammar.constant_tokens:
          if remaining_input.startswith(constant_token):
              # We have a match so add it to result
              tokens.push(constant_token)
              # And shrink remaining input
              remaining_input = remaining_input[len(constant_token):]
              break

        # No tokens matched
        else:
              raise TokenizationError

# Lastly, add the end of input token so the parser eventually accepts.
tokens.push($)
```

Tokenized example input:

```
%b '+' %b '*' %b $
```

### 7) Parsing the tokens

Finally, here is the parsing algorithm in a python-like pseudocode:

```python
# Initialize the parsing state
state_stack = [ 0 ]
tree_stack = []
remaining_tokens.reverse()

# Get the first token
current_token = remaining_tokens.pop()

# Loop until algorithm either accepts or rejects
while True:
    # Get the current state
    current_state = state_stack[-1]

    # Get the action to take from ACTION table
    action_to_take = action_table[current_state, current_token]

    # If the action is to shift
    if action_to_take == Shift(next_state):
        # Create a terminal tree for the current token and push it to the tree stack
        tree_stack.push(TerminalNode(current_token));
        # Pop the next token
        current_token = remaining_tokens.pop()
        # Transition to the next state according to ACTION table
        state_stack.push(next_state);

    # If the action is to reduce
    elif action_to_take == Reduce(rule_index):
        # Get the length of the pattern of the rule, say N
        rule = grammar.rules[rule_index]
        pattern_length = len(rule.pattern)

        # Create a non-terminal tree with last N items in the tree stack
        tree = NonTerminalNode(rule.symbol, tree_stack[-pattern_length:])

        # Shrink state and tree stacks
        tree_stack = tree_stack[:-pattern_length]
        state_stack = state_stack[:-pattern_length]

        # Push the new tree to the tree stack
        tree_stack.push(tree)
        # Transition to the next state according to GOTO table
        state_stack.push(goto_table[state_stack[-1], rule.symbol])

    # If the action is to accept
    elif action_to_take == Accept(rule_index):
        # Create a final non-terminal tree with everything in the tree stack and return it
        return NonTerminalNode(grammar.start_symbol, tree_stack)

    # No action can be taken, so input is not well-formed
    else:
        # So raise an error
        raise ParsingError
```

The parsing steps for the example input:

```
+------+-------------+----------------+--------------------+-------------------------+
| Step | State Stack |  Symbol Stack  |  Remaining Input   |      Action Taken       |
+------+-------------+----------------+--------------------+-------------------------+
| 0    | 0           |                | %b '+' %b '*' %b $ | Shift 4                 |
+------+-------------+----------------+--------------------+-------------------------+
| 1    | 0 4         | %b             |    '+' %b '*' %b $ | Reduce 4 (T -> %b)      |
+------+-------------+----------------+--------------------+-------------------------+
| 2    | 0 3         | T              |    '+' %b '*' %b $ | Reduce 3 (F -> T)       |
+------+-------------+----------------+--------------------+-------------------------+
| 3    | 0 2         | F              |    '+' %b '*' %b $ | Reduce 1 (E -> F)       |
+------+-------------+----------------+--------------------+-------------------------+
| 4    | 0 1         | E              |    '+' %b '*' %b $ | Shift 7                 |
+------+-------------+----------------+--------------------+-------------------------+
| 5    | 0 1 7       | E '+'          |        %b '*' %b $ | Shift 4                 |
+------+-------------+----------------+--------------------+-------------------------+
| 6    | 0 1 7 4     | E '+' %b       |           '*' %b $ | Reduce 4 (T -> %b)      |
+------+-------------+----------------+--------------------+-------------------------+
| 7    | 0 1 7 3     | E '+' T        |           '*' %b $ | Reduce 3 (F -> T)       |
+------+-------------+----------------+--------------------+-------------------------+
| 8    | 0 1 7 8     | E '+' F        |           '*' %b $ | Shift 5                 |
+------+-------------+----------------+--------------------+-------------------------+
| 9    | 0 1 7 8 5   | E '+' F '*'    |               %b $ | Shift 4                 |
+------+-------------+----------------+--------------------+-------------------------+
| 10   | 0 1 7 8 5 4 | E '+' F '*' %b |                  $ | Reduce 4 (T -> %b)      |
+------+-------------+----------------+--------------------+-------------------------+
| 11   | 0 1 7 8 5 6 | E '+' F '*' T  |                  $ | Reduce 2 (F -> F '*' T) |
+------+-------------+----------------+--------------------+-------------------------+
| 12   | 0 1 7 8     | E '+' F        |                  $ | Accept 0 (E -> E '+' F) |
+------+-------------+----------------+--------------------+-------------------------+
```

And lastly, the parse tree of the example input:

```
E
├─ E
│  └─ F
│     └─ T
│        └─ 1
├─ +
└─ F
   ├─ F
   │  └─ T
   │     └─ 0
   ├─ *
   └─ T
      └─ 1
```

## Can I have symbols that can match to empty string?

Yes, empty symbols are supported!

In the grammar definition, you can have rules like `P -> ''`
to indicate that `P` can match the empty string.

```
P -> 'x' O 'z'

O -> 'y'
O -> ''
```

When used as the sole pattern, `''` will indicate mark the symbol as `can be empty`.
However, using `''` anywhere else would be ignored (e.g., `P -> A '' B` would be parsed as `P -> A B`).

The grammar above will be able to parse both `x y z` and `x z`.

```
+----------------------+
|       Grammar        |
+----------------------+
|  1)  P -> 'x' O 'z'  |
|  2)  O -> 'y'        |
|  3)  O -> ε          |
+----------------------+
+--------+------------+------------+
| Symbol | First Set  | Follow Set |
+--------+------------+------------+
| P      | { 'x' }    | { $ }      |
+--------+------------+------------+
| O      | { 'y', ε } | { 'z' }    |
+--------+------------+------------+
+-------+--------------------+------------+--------------+
| State |       Items        | Lookaheads | Transitions  |
+-------+--------------------+------------+--------------+
| 0     |  P -> . 'x' O 'z'  | { $ }      |  'x'  ->  1  |
+-------+--------------------+------------+--------------+
| 1     |  P -> 'x' . O 'z'  | { $ }      |   O   ->  2  |
|       |  O -> . 'y'        | { 'z' }    |  'y'  ->  3  |
|       |  O -> . ε          | { 'z' }    |              |
+-------+--------------------+------------+--------------+
| 2     |  P -> 'x' O . 'z'  | { $ }      |  'z'  ->  4  |
+-------+--------------------+------------+--------------+
| 3     |  O -> 'y' .        | { 'z' }    |              |
+-------+--------------------+------------+--------------+
| 4     |  P -> 'x' O 'z' .  | { $ }      |              |
+-------+--------------------+------------+--------------+
+-------+--------------------------------+--------------+
|       |             Action             |     Goto     |
| State | ------------------------------ | ------------ |
|       |    'x'    'z'    'y'     $     |    P    O    |
+-------+--------------------------------+--------------+
| 0     |    s1      -      -      -     |    -    -    |
+-------+--------------------------------+--------------+
| 1     |     -     r3     s3      -     |    -    2    |
+-------+--------------------------------+--------------+
| 2     |     -     s4      -      -     |    -    -    |
+-------+--------------------------------+--------------+
| 3     |     -     r2      -      -     |    -    -    |
+-------+--------------------------------+--------------+
| 4     |     -      -      -     a1     |    -    -    |
+-------+--------------------------------+--------------+

> x y z

P
├─ x
├─ O
│  └─ y
└─ z

+------+-------------+--------------+-----------------+---------------------------+
| Step | State Stack | Symbol Stack | Remaining Input |       Action Taken        |
+------+-------------+--------------+-----------------+---------------------------+
| 0    | 0           |              |   'x' 'y' 'z' $ | Shift 1                   |
+------+-------------+--------------+-----------------+---------------------------+
| 1    | 0 1         | 'x'          |       'y' 'z' $ | Shift 3                   |
+------+-------------+--------------+-----------------+---------------------------+
| 2    | 0 1 3       | 'x' 'y'      |           'z' $ | Reduce 2 (O -> 'y')       |
+------+-------------+--------------+-----------------+---------------------------+
| 3    | 0 1 2       | 'x' O        |           'z' $ | Shift 4                   |
+------+-------------+--------------+-----------------+---------------------------+
| 4    | 0 1 2 4     | 'x' O 'z'    |               $ | Accept 1 (P -> 'x' O 'z') |
+------+-------------+--------------+-----------------+---------------------------+

> x z

P
├─ x
├─ O
└─ z

+------+-------------+--------------+-----------------+---------------------------+
| Step | State Stack | Symbol Stack | Remaining Input |       Action Taken        |
+------+-------------+--------------+-----------------+---------------------------+
| 0    | 0           |              |       'x' 'z' $ | Shift 1                   |
+------+-------------+--------------+-----------------+---------------------------+
| 1    | 0 1         | 'x'          |           'z' $ | Reduce 3 (O -> ε)         |
+------+-------------+--------------+-----------------+---------------------------+
| 2    | 0 1 2       | 'x' O        |           'z' $ | Shift 4                   |
+------+-------------+--------------+-----------------+---------------------------+
| 3    | 0 1 2 4     | 'x' O 'z'    |               $ | Accept 1 (P -> 'x' O 'z') |
+------+-------------+--------------+-----------------+---------------------------+
```

## Can I have an LALR(1) parser instead of an LR(1) parser?

Yes, `dotlr` supports both LR(1) and LALR(1) parsers!

In the CLI, you can simply use the `--lalr` option:

```shell
dotlr --lalr grammar.lr "1 + 2 * 3"
```

And in the API, you can use `Parser::lalr` instead of `Parser::lr`:

```rust
Parser::lalr(grammar)
```

LALR(1) parser is construction is very similar to LR(1) parser construction. The only difference is
that after [4) Constructing the LR(1) automaton](#4-constructing-the-lr1-automaton), there is
another step to reduce the LR(1) automaton to an LALR(1) automaton, using the following algorithm:

```python
# Iterate over the state pairs of the automaton.
for state1 in automaton.states:
  for state2 in automaton.states:
    # Check if the states share the same core.
    # Which means their items are the same ignoring the lookaheads.

    # Here is an example:
    # ...
    # +-------+------------------------+--------------+---------------+
    # | 3     |  T -> %id . '(' E ')'  | { $, '+' }   |  '('  ->  4   |
    # |       |  T -> %id .            | { $, '+' }   |               |
    # +-------+------------------------+--------------+---------------+
    # ...
    # +-------+------------------------+--------------+---------------+
    # | 6     |  T -> %id . '(' E ')'  | { ')', '+' } |  '('  ->  7   |
    # |       |  T -> %id .            | { ')', '+' } |               |
    # +-------+------------------------+--------------+---------------+
    # ...

    if state1.core == state2.core:
      # Merge the states.
      # Which is combining lookaheads of the same items.
      # Transitions should be mapped to the new states as well.

      # Here is the merge of the two states in the example above:
      # ...
      # +-------+------------------------+-----------------+--------------+
      # | 3     |  T -> %id . '(' E ')'  | { $, '+', ')' } |  '('  ->  4  |
      # |       |  T -> %id .            | { $, '+', ')' } |              |
      # +-------+------------------------+-----------------+--------------+
      # ...

      automaton.merge_states(state1, state2)
```

the actual implementation is a bit more involved, but the idea is exactly this.
Luckily, it's documented extensively at
[automaton.rs](https://github.com/umut-sahin/dotlr/blob/main/src/automaton.rs)
(search for `to_lalr`). I highly recommend reading the comments in the
source to understand the nuances of the implementation.

## Any benchmarks?

Yes, even though `dotlr` isn't a performance focused project, I thought it'd be interesting to have
a benchmark suite to see how changes affect its performance, so I've created a couple of JSON
grammars and benchmarked parsing them.

You can run the benchmarks with this command in the project directory:

```shell
cargo bench
```

This command prints the following in my own computer with an `Intel i7-12700K` CPU:

```
...

Parsing JSON/Simple LR(1)
                        time:   [260.33 ms 265.74 ms 269.29 ms]
                        thrpt:  [92.578 MiB/s 93.815 MiB/s 95.765 MiB/s]

...

Parsing JSON/Simple LALR(1)
                        time:   [287.93 ms 288.71 ms 289.49 ms]
                        thrpt:  [86.119 MiB/s 86.350 MiB/s 86.583 MiB/s]

...

Parsing JSON/Optimized LR(1)
                        time:   [211.55 ms 211.71 ms 211.90 ms]
                        thrpt:  [117.65 MiB/s 117.76 MiB/s 117.85 MiB/s]

...

Parsing JSON/Optimized LALR(1)
                        time:   [192.66 ms 193.53 ms 194.39 ms]
                        thrpt:  [128.25 MiB/s 128.82 MiB/s 129.40 MiB/s]

...
```

Furthermore, it generates an HTML report with detailed plots. You can find this
report at `target/criterion/report/index.html`, after running the command.

Performance isn't good for a JSON parser, but that's to be expected as it's not the primary
objective. Feel free to create pull requests to improve parsing performance, hopefully
without changing the understandability of the library.

Also keep in mind that these benchmarks are only for the parsing step. Tokenization
is not the focus of this library, and frankly, its implementation is not the best.

## Can I modify it?

Of course, feel free to fork `dotlr` from [GitHub](https://github.com/umut-sahin/dotlr), clone the
fork to your machine, fire up your favourite IDE, and make any modification you want.

Don't forget to open up a pull request if you have useful changes that can be beneficial to everyone
using `dotlr`!

## Which resources did you use when creating this?

- [https://www3.nd.edu/~dthain/compilerbook/chapter4.pdf](https://www3.nd.edu/~dthain/compilerbook/chapter4.pdf)
- [https://soroushj.github.io/lr1-parser-vis/](https://soroushj.github.io/lr1-parser-vis/)
- [https://jsmachines.sourceforge.net/machines/lr1.html](https://jsmachines.sourceforge.net/machines/lr1.html)

## How is it licensed?

[dotlr](https://github.com/umut-sahin/dotlr) is free, open source and permissively licensed!

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](https://github.com/umut-sahin/dotlr/blob/main/LICENSE-MIT)
  or <https://opensource.org/licenses/MIT>)
- Apache License, Version
  2.0 ([LICENSE-APACHE]((https://github.com/umut-sahin/dotlr/blob/main/LICENSE-APACHE))
  or <https://www.apache.org/licenses/LICENSE-2.0>)

This means you can select the license you prefer!

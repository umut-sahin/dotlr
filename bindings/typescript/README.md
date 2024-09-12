## Overview

`dotlr` is a library for creating and inspecting LR family of parsers in TypeScript. It provides an interface to parse grammars, generate parsing tables, and trace parsing of inputs. The library leverages WebAssembly (WASM) to ensure efficient parsing.

It is focused on providing educational resources for learning about parsing algorithms and compiler construction. The library is designed to be easy to use and understand, making it ideal for students, educators, and developers interested in language processing.

### Table of Contents
1. [Installation](#installation)
2. [Basic Usage](#basic-usage)
3. [Defining a Grammar](#defining-a-grammar)
4. [Creating LR(1) Parser of the Grammar](#creating-lr1-parser-of-the-grammar)
5. [Creating LALR(1) Parser of the Grammar](#creating-lalr1-parser-of-the-grammar)

## Installation

Before using the `dotlr` library, you need to install it. The following instructions assume you have a project with `npm` already set up.

```bash
npm install dotlr 
```

### Importing the Library

To use the `dotlr` library, import it into your TypeScript files:

```ts
import { Grammar, LR1Parser, LALRParser } from 'dotlr';
```
this library uses `ts-results` under the hood to handle errors and results.
```ts
import { Ok, Err } from 'ts-results';
```
## Basic Usage

The core of the `dotlr` library revolves around defining a grammar and using it to create a parser. The following steps will guide you through this process.

## Defining a Grammar

A grammar is a set of rules that define how input strings can be parsed. You can create a grammar using `Grammar.parse()` method. Here's an example:

For more information on the syntax of the grammar, look [here](https://github.com/umut-sahin/dotlr?tab=readme-ov-file#usage)

```ts
const grammarStr = `
    S -> A
    A -> 'a' A
    A -> 'b'
`;

const grammarResult = Grammar.parse(grammarStr);

if (grammarResult.ok) {
    const grammar = grammarResult.val;
    console.log("Grammar successfully parsed!");
    console.log(grammar.getSymbols());
    console.log(grammar.getProductions());
} else {
    console.error("Failed to parse grammar:", grammarResult.val);
}
```

- **Grammar.parse()**: Parses a string representation of a grammar and returns a `Grammar` object.
- **grammar.getSymbols()**: Returns all symbols (non-terminal and terminal) used in the grammar.
- **grammar.getProductions()**: Retrieves the list of productions (rules) defined in the grammar.

## Creating LR(1) Parser of the Grammar

The `LR1Parser` class allows you to create an LR(1) parser for the grammar and use it to parse input.

```ts
const lr1ParserResult = LR1Parser.fromGrammar(grammar);

if (lr1ParserResult.ok) {
    const lr1Parser = lr1ParserResult.val;

    const input = "aab";
    const parseResult = lr1Parser.parse(input);

    if (parseResult.ok) {
        const parseTree = parseResult.val;
        console.log("Parse successful!");
        console.log(parseTree);
    } else {
        console.error("Parse error:", parseResult.val);
    }
} else {
    console.error("Failed to create LR(1) parser:", lr1ParserResult.val);
}
```

- **LR1Parser.fromGrammar()**: Consumes the `Grammar` object and returns an `LR1Parser`, you cannot reuse the *Grammar* object, if you need it, you can clone it by using `grammar.clone()`.
- **parser.parse()**: method attempts to parse the given input string according to the LR(1) grammar. Returns a parse tree if successful.
- **parser.trace()** method can be used to trace the parsing process. It returns a trace and the resulting parse tree at each step, if parsing is successful.
- **parser.tokenize()** method can be used to tokenize the input string. It returns a list of tokens.
- **parser.getActionTable()** method returns the action table of the parser, which is used to determine the next action based on the current state and input token.
- **parser.getGotoTable()** method returns the goto table of the parser, which is used to determine the next state based on the current state and non-terminal symbol.
- **parser.getParseTables()** method returns the parsing tables of the parser, which include the action and goto tables.
- **parser.getAutomaton()** method returns the automaton of the parser, which represents the states and transitions of the LR(1) parser.
- **parser.getFirstTable()** method returns the first table of the parser, which contains the first sets of symbols.
- **parser.getFollowTable()** method returns the follow table of the parser, which contains the follow sets of symbols.

## Creating LALR(1) Parser of the Grammar

The `LALR1Parser` is similar to the `LR1Parser`, but it uses Look-Ahead LR parsing, the API is the same.

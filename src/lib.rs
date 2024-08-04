#![doc = include_str!("../README.md")]

mod automaton;
mod errors;
mod grammar;
mod parser;
mod tables;
mod trace;
mod tree;

pub use {
    automaton::{
        Automaton,
        Item,
        State,
    },
    errors::{
        GrammarError,
        ParserError,
        ParsingError,
    },
    grammar::{
        AtomicPattern,
        ConstantToken,
        Grammar,
        RegexToken,
        Rule,
        Symbol,
        Token,
    },
    parser::Parser,
    tables::{
        Action,
        FirstTable,
        FollowTable,
        ParsingTables,
    },
    trace::{
        Step,
        Trace,
    },
    tree::Tree,
};

mod prelude {
    pub use {
        super::*,
        colored::Colorize,
        indexmap::{
            IndexMap,
            IndexSet,
        },
        itertools::Itertools,
        logos::{
            Lexer,
            Logos,
        },
        prettytable::{
            cell,
            format::{
                FormatBuilder,
                LinePosition,
                LineSeparator,
            },
            row,
            Row,
            Table,
        },
        ptree::TreeBuilder,
        regex::Regex,
        smallvec::{
            smallvec,
            SmallVec,
        },
        smol_str::{
            format_smolstr,
            SmolStr,
        },
        std::{
            self,
            collections::BTreeMap,
            fmt::{
                self,
                Debug,
                Display,
            },
            ops::Deref,
        },
        thiserror::Error,
    };
}

#![doc = include_str!("../README.md")]

mod automaton;
mod errors;
mod grammar;
mod parser;
mod span;
mod tables;
mod trace;
mod tree;
mod utils;

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
    span::{
        Span,
        Spanned,
    },
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
            Row,
            Table,
            cell,
            format::{
                FormatBuilder,
                LinePosition,
                LineSeparator,
            },
            row,
        },
        ptree::TreeBuilder,
        regex::Regex,
        smallvec::{
            SmallVec,
            smallvec,
        },
        smol_str::{
            SmolStr,
            format_smolstr,
        },
        std::{
            self,
            collections::BTreeMap,
            fmt::{
                self,
                Debug,
                Display,
            },
            io::BufWriter,
            ops::Deref,
        },
        thiserror::Error,
    };

    #[cfg(feature = "serde")]
    pub use serde_renamed::{
        Serialize,
        Serializer,
        ser::SerializeMap,
    };

    #[cfg(feature = "wasm")]
    pub use {
        errors::WasmParserError,
        wasm_bindgen::prelude::*,
    };

    #[cfg(not(target_family = "wasm"))]
    pub use colored::*;

    #[cfg(target_family = "wasm")]
    pub use utils::MockColored;
}

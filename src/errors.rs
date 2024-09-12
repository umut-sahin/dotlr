use crate::prelude::*;


/// Grammar error of a grammar string tried to be converted to a grammar.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
#[derive(Debug, Error)]
pub enum GrammarError {
    /// An unexpected token has been encountered.
    #[error(
        "unexpected token {} at line {} column {} (expected {})",
        token.green(),
        format_smolstr!("{}", line).cyan(),
        format_smolstr!("{}", column).cyan(),
        if expected.len() == 1 {
            format!("{}", format_smolstr!("{}", expected[0]).green())
        } else {
            format!(
                "one of {}",
                expected.iter().map(|token| format_smolstr!("{}", token).green()).join(", "),
            )
        },
    )]
    UnexpectedToken { line: usize, column: usize, token: SmolStr, expected: SmallVec<[SmolStr; 2]> },

    /// An unexpected end of file has been encountered.
    #[error(
        "unexpected end of file (expected {})",
        if expected.len() == 1 {
            format!("{}", format_smolstr!("{}", expected[0]).green())
        } else {
            format!(
                "one of {}",
                expected.iter().map(|token| format_smolstr!("{}", token).green()).join(", "),
            )
        },
    )]
    UnexpectedEof { expected: SmallVec<[SmolStr; 2]> },

    /// Invalid regex has been encountered.
    #[error(
        "invalid regex {} at line {} column {}",
        regex.green(),
        format_smolstr!("{}", line).cyan(),
        format_smolstr!("{}", column).cyan(),
    )]
    InvalidRegex { line: usize, column: usize, regex: SmolStr },
}


/// Parser error of a parser tried to be constructed from a grammar.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
#[derive(Debug, Error)]
pub enum ParserError {
    /// An empty grammar is tried to be parsed.
    #[error("grammar is empty")]
    EmptyGrammar,

    /// An undefined symbol is used in a pattern.
    #[error(
        "symbol {} in rule {} is not defined",
        format_smolstr!("{}", symbol).green(),
        format_smolstr!("{}", rule).green(),
    )]
    UndefinedSymbol { symbol: Symbol, rule: Rule },

    /// An undefined symbol is used in a pattern.
    #[error(
        "regex token {} in rule {} is not defined",
        format_smolstr!("{}", regex_token).green(),
        format_smolstr!("{}", rule).green(),
    )]
    UndefinedRegexToken { regex_token: RegexToken, rule: Rule },

    /// A conflict has been detected.
    #[error(
        "conflict at state {} on {}",
        format_smolstr!("{}", state).green(),
        format_smolstr!("{}", token).green(),
    )]
    Conflict { parser: Box<Parser>, state: usize, token: Token },
}


/// Parsing error of an input tried to be parsed with a parser.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
#[derive(Debug, Error)]
pub enum ParsingError {
    /// An unknown token has been encountered.
    #[error(
        "unknown token {}",
        format_smolstr!("{}", token).green(),
    )]
    UnknownToken { token: SmolStr },

    /// An unexpected token has been encountered.
    #[error(
        "unexpected token {} (expected {})",
        format_smolstr!("{}", token).green(),
        if expected.len() == 1 {
            format!("{}", format_smolstr!("{}", expected[0]).green())
        } else {
            format!(
                "one of {}",
                expected.iter().map(|token| format_smolstr!("{}", token).green()).join(", "),
            )
        },
    )]
    UnexpectedToken { token: SmolStr, expected: SmallVec<[Token; 2]> },

    /// An unexpected end of input has been encountered.
    #[error(
        "unexpected end of input (expected {})",
        if expected.len() == 1 {
            format!("{}", format_smolstr!("{}", expected[0]).green())
        } else {
            format!(
                "one of {}",
                expected.iter().map(|token| format_smolstr!("{}", token).green()).join(", "),
            )
        },
    )]
    UnexpectedEof { expected: SmallVec<[Token; 2]> },
}

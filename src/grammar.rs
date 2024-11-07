use crate::prelude::*;


/// Symbol (e.g., `S`, `E`) in a grammar.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Symbol(SmolStr);

impl Deref for Symbol {
    type Target = SmolStr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<SmolStr>> From<T> for Symbol {
    fn from(symbol: T) -> Symbol {
        Symbol(symbol.into())
    }
}


/// Constant token (e.g., `'+'`, `'-'`) in a grammar.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ConstantToken(SmolStr);

impl Deref for ConstantToken {
    type Target = SmolStr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ConstantToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.0)
    }
}

impl<T: Into<SmolStr>> From<T> for ConstantToken {
    fn from(constant_token: T) -> ConstantToken {
        ConstantToken(constant_token.into())
    }
}


/// Regular expression token (e.g., `%f`, `%s`) in a grammar.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RegexToken(SmolStr);

impl Deref for RegexToken {
    type Target = SmolStr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for RegexToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}

impl<T: Into<SmolStr>> From<T> for RegexToken {
    fn from(regex_token: T) -> RegexToken {
        RegexToken(regex_token.into())
    }
}


/// Token (e.g., `'+'`, `%f`, `$`) in a grammar.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
    /// Empty token.
    Empty,
    /// Constant token.
    Constant(ConstantToken),
    /// Regular expression token.
    Regex(RegexToken),
    /// End of file token.
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Empty => {
                write!(f, "ε")
            },
            Token::Constant(constant_token) => {
                write!(f, "{}", constant_token)
            },
            Token::Regex(regex_token) => {
                write!(f, "{}", regex_token)
            },
            Token::Eof => {
                write!(f, "$")
            },
        }
    }
}

impl From<ConstantToken> for Token {
    fn from(constant_token: ConstantToken) -> Token {
        Token::Constant(constant_token)
    }
}

impl From<RegexToken> for Token {
    fn from(regex_token: RegexToken) -> Token {
        Token::Regex(regex_token)
    }
}


/// Elements (e.g., `E`, `'+'`, `%f`) of the pattern of a rule.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AtomicPattern {
    /// Symbol to match.
    Symbol(Symbol),
    /// Token to match.
    Token(Token),
}

impl Display for AtomicPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomicPattern::Symbol(symbol) => write!(f, "{}", symbol),
            AtomicPattern::Token(token) => write!(f, "{}", token),
        }
    }
}

impl From<Symbol> for AtomicPattern {
    fn from(symbol: Symbol) -> AtomicPattern {
        AtomicPattern::Symbol(symbol)
    }
}

impl From<Token> for AtomicPattern {
    fn from(token: Token) -> AtomicPattern {
        AtomicPattern::Token(token)
    }
}

impl From<ConstantToken> for AtomicPattern {
    fn from(constant_token: ConstantToken) -> AtomicPattern {
        AtomicPattern::Token(Token::Constant(constant_token))
    }
}

impl From<RegexToken> for AtomicPattern {
    fn from(regex_token: RegexToken) -> AtomicPattern {
        AtomicPattern::Token(Token::Regex(regex_token))
    }
}


/// Rule (e.g., `S -> E` `E -> F '+' E`) of a grammar.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    symbol: Symbol,
    pattern: SmallVec<[AtomicPattern; 3]>,
}

impl Rule {
    /// Creates a new rule.
    pub fn new(
        symbol: impl Into<Symbol>,
        pattern: impl IntoIterator<Item = AtomicPattern>,
    ) -> Rule {
        Rule { symbol: symbol.into(), pattern: pattern.into_iter().collect() }
    }
}

impl Rule {
    /// Gets the symbol of the rule.
    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    /// Gets the pattern of the rule.
    pub fn pattern(&self) -> &[AtomicPattern] {
        &self.pattern
    }

    /// Gets whether the rule is `S -> ''`.
    pub fn is_empty_pattern(&self) -> bool {
        self.pattern.len() == 1 && self.pattern[0] == AtomicPattern::Token(Token::Empty)
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ->", self.symbol)?;
        for atomic_pattern in self.pattern.iter() {
            write!(f, " {}", atomic_pattern)?;
        }
        Ok(())
    }
}


/// Grammar of a language.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug)]
pub struct Grammar {
    symbols: IndexSet<Symbol>,
    start_symbol: Symbol,
    empty_symbols: IndexSet<Symbol>,
    constant_tokens: IndexSet<ConstantToken>,
    #[cfg_attr(feature = "serde", serde(serialize_with = "utils::serialize_regex_map"))]
    regular_expressions: IndexMap<RegexToken, Regex>,
    rules: Vec<Rule>,
}


impl Grammar {
    /// Creates a grammar from a grammar string.
    pub fn parse(grammar_string: &str) -> Result<Grammar, GrammarError> {
        grammar_parser::parse(grammar_string)
    }
}

impl Grammar {
    /// Gets the symbols of the grammar.
    pub fn symbols(&self) -> &IndexSet<Symbol> {
        &self.symbols
    }

    /// Gets the start symbol of the grammar.
    pub fn start_symbol(&self) -> &Symbol {
        &self.start_symbol
    }

    /// Gets the empty symbols of the grammar.
    pub fn empty_symbols(&self) -> &IndexSet<Symbol> {
        &self.empty_symbols
    }

    /// Gets the constant tokens of the grammar.
    pub fn constant_tokens(&self) -> &IndexSet<ConstantToken> {
        &self.constant_tokens
    }

    /// Gets the regular expressions of the grammar.
    pub fn regular_expressions(&self) -> &IndexMap<RegexToken, Regex> {
        &self.regular_expressions
    }

    /// Gets the rules of the grammar.
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
}

impl Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rule in self.rules.iter() {
            writeln!(f, "{}", rule)?;
        }
        if !self.regular_expressions.is_empty() {
            writeln!(f)?;
        }
        for (regex_token, regex) in self.regular_expressions.iter() {
            writeln!(f, "{} -> /{}/", regex_token, regex)?;
        }
        Ok(())
    }
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Grammar {
    pub fn parse_wasm(grammar_string: &str) -> Result<Grammar, JsValue> {
        match Grammar::parse(grammar_string) {
            Ok(grammar) => Ok(grammar),
            Err(error) => Err(serde_wasm_bindgen::to_value(&error)?),
        }
    }
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Grammar {
    pub fn symbols_wasm(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.symbols)?)
    }
    pub fn start_symbol_wasm(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.start_symbol)?)
    }
    pub fn rules_wasm(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.rules)?)
    }
    pub fn to_string_wasm(&self) -> String {
        self.to_string()
    }
    pub fn constant_tokens_wasm(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.constant_tokens)?)
    }

    pub fn regular_expressions_wasm(&self) -> Result<JsValue, JsValue> {
        let index_map: IndexMap<RegexToken, String> =
            self.regular_expressions.iter().map(|(k, v)| (k.clone(), v.to_string())).collect();
        Ok(serde_wasm_bindgen::to_value(&index_map)?)
    }
    pub fn clone_wasm(&self) -> Grammar {
        self.clone()
    }
}

/// Internal module to parse grammar strings.
mod grammar_parser {
    use crate::prelude::*;

    #[derive(Debug, Logos, PartialEq)]
    #[logos(skip r"[ \t\r\f]+")]
    #[logos(extras = (usize, usize))]
    enum GrammarToken {
        /// A, B, C, ...
        #[regex("[a-zA-Z0-9]+", |lexer| Symbol::from(lexer.slice()))]
        Symbol(Symbol),

        /// ->
        #[token("->")]
        Arrow,

        /// '+', '-', ',', ...
        #[regex(r#"'([^'])*'"#, |lexer| ConstantToken::from(lexer.slice().trim_matches('\'')))]
        ConstantToken(ConstantToken),

        /// %d, %f, %s, ...
        #[regex("%[a-zA-Z0-9]+", |lexer| RegexToken::from(&lexer.slice()[1..]))]
        RegexToken(RegexToken),

        /// /\[0-9]+/, /\[a-z]+/, ...
        #[regex(r#"/([^/])*/"#, |lexer| SmolStr::from(lexer.slice().trim_matches('/')))]
        Regex(SmolStr),

        /// \n
        #[token("\n")]
        NewLine,

        /// \# ...
        #[regex("#.*")]
        Comment,
    }

    #[derive(Debug)]
    #[allow(clippy::enum_variant_names)]
    enum GrammarParsingState {
        AwaitingSymbolOrRegexToken,

        AwaitingArrowForRule { symbol: Symbol },
        AwaitingAtomicPatterns { symbol: Symbol, pattern: SmallVec<[AtomicPattern; 3]> },

        AwaitingArrowForRegex { regex_token: RegexToken },
        AwaitingRegex { regex_token: RegexToken },
    }

    impl GrammarParsingState {
        pub fn unexpected_token(&self, lexer: Lexer<GrammarToken>) -> GrammarError {
            let (line, column) = lexer.extras;
            let slice = lexer.slice();
            let token = if slice == "\n" { "\\n".into() } else { slice.into() };
            let expected = match self.unexpected_eof() {
                GrammarError::UnexpectedEof { expected } => expected,
                _ => unreachable!(),
            };

            GrammarError::UnexpectedToken { line, column, token, expected }
        }

        pub fn unexpected_eof(&self) -> GrammarError {
            let mut expected = SmallVec::new();
            match self {
                GrammarParsingState::AwaitingSymbolOrRegexToken => {
                    expected.push("symbol".into());
                    expected.push("regular expression token".into());
                },

                GrammarParsingState::AwaitingArrowForRule { .. }
                | GrammarParsingState::AwaitingArrowForRegex { .. } => {
                    expected.push("'->'".into());
                },

                GrammarParsingState::AwaitingAtomicPatterns { .. } => {
                    expected.push("symbol".into());
                    expected.push("constant token".into());
                    expected.push("regular expression token".into());
                },
                GrammarParsingState::AwaitingRegex { .. } => {
                    expected.push("regular expression".into());
                },
            };

            GrammarError::UnexpectedEof { expected }
        }

        pub fn unexpected_regex(&self, lexer: Lexer<GrammarToken>) -> GrammarError {
            let (line, column) = lexer.extras;
            let slice = lexer.slice();
            let regex = slice.into();

            GrammarError::InvalidRegex { line, column, regex }
        }
    }

    pub fn parse(grammar_string: &str) -> Result<Grammar, GrammarError> {
        let mut lexer = GrammarToken::lexer(grammar_string);
        let mut state = GrammarParsingState::AwaitingSymbolOrRegexToken;

        let mut symbols = IndexSet::new();
        let mut start_symbol = None;
        let mut empty_symbols = IndexSet::new();
        let mut constant_tokens = IndexSet::new();
        let mut regular_expressions = IndexMap::new();
        let mut rules = Vec::new();

        let mut line = 1;
        let mut column = 1;
        lexer.extras = (line, column);

        let mut column_start_position = 0;
        while let Some(token) = lexer.next() {
            let span = lexer.span();

            let remaining_line_slice = &grammar_string[column_start_position..span.start];
            column_start_position = span.start;

            column += remaining_line_slice.chars().count();
            lexer.extras = (line, column);

            let token = match token {
                Ok(token) => token,
                Err(_) => return Err(state.unexpected_token(lexer)),
            };

            let newline = token == GrammarToken::NewLine;
            match &mut state {
                GrammarParsingState::AwaitingSymbolOrRegexToken => {
                    match token {
                        GrammarToken::NewLine | GrammarToken::Comment => {},
                        GrammarToken::Symbol(symbol) => {
                            if start_symbol.is_none() {
                                start_symbol = Some(symbol.clone());
                            }
                            symbols.insert(symbol.clone());
                            state = GrammarParsingState::AwaitingArrowForRule { symbol };
                        },
                        GrammarToken::RegexToken(regex_token) => {
                            state = GrammarParsingState::AwaitingArrowForRegex { regex_token };
                        },
                        _ => {
                            return Err(state.unexpected_token(lexer));
                        },
                    }
                },

                GrammarParsingState::AwaitingArrowForRule { symbol } => {
                    match token {
                        GrammarToken::Arrow => {
                            state = GrammarParsingState::AwaitingAtomicPatterns {
                                symbol: symbol.clone(),
                                pattern: smallvec![],
                            };
                        },
                        _ => {
                            return Err(state.unexpected_token(lexer));
                        },
                    }
                },
                GrammarParsingState::AwaitingArrowForRegex { regex_token } => {
                    match token {
                        GrammarToken::Arrow => {
                            state = GrammarParsingState::AwaitingRegex {
                                regex_token: regex_token.clone(),
                            };
                        },
                        _ => {
                            return Err(state.unexpected_token(lexer));
                        },
                    }
                },

                GrammarParsingState::AwaitingAtomicPatterns { symbol, pattern } => {
                    match token {
                        GrammarToken::Comment => {},
                        GrammarToken::Symbol(symbol) => {
                            pattern.push(AtomicPattern::Symbol(symbol));
                        },
                        GrammarToken::ConstantToken(constant_token) => {
                            if constant_token.is_empty() {
                                pattern.push(AtomicPattern::Token(Token::Empty));
                            } else {
                                constant_tokens.insert(constant_token.clone());
                                pattern.push(AtomicPattern::Token(Token::Constant(constant_token)));
                            }
                        },
                        GrammarToken::RegexToken(regex_token) => {
                            pattern.push(AtomicPattern::Token(Token::Regex(regex_token)));
                        },
                        GrammarToken::NewLine => {
                            if pattern.is_empty() {
                                return Err(state.unexpected_token(lexer));
                            }

                            let rule = Rule {
                                symbol: std::mem::replace(symbol, Symbol::from("")),
                                pattern: std::mem::take(pattern),
                            };
                            rules.push(rule);

                            state = GrammarParsingState::AwaitingSymbolOrRegexToken;
                        },

                        _ => {
                            return Err(state.unexpected_token(lexer));
                        },
                    }
                },
                GrammarParsingState::AwaitingRegex { regex_token } => {
                    match token {
                        GrammarToken::Regex(regex_string) => {
                            let regex =
                                match Regex::new(format_smolstr!("^{}", regex_string).as_str()) {
                                    Ok(regex) => regex,
                                    Err(_) => {
                                        return Err(state.unexpected_regex(lexer));
                                    },
                                };
                            regular_expressions.insert(regex_token.clone(), regex);
                            state = GrammarParsingState::AwaitingSymbolOrRegexToken;
                        },
                        _ => {
                            return Err(state.unexpected_token(lexer));
                        },
                    }
                },
            }

            if newline {
                line += 1;
                column = 1;
                lexer.extras = (line, column);
            }
        }

        match &mut state {
            GrammarParsingState::AwaitingSymbolOrRegexToken => {},
            GrammarParsingState::AwaitingAtomicPatterns { symbol, pattern } => {
                if pattern.is_empty() {
                    return Err(state.unexpected_eof());
                }

                let rule = Rule {
                    symbol: std::mem::replace(symbol, Symbol::from("")),
                    pattern: std::mem::take(pattern),
                };
                rules.push(rule);
            },
            _ => {
                return Err(state.unexpected_eof());
            },
        }

        for rule in rules.iter_mut() {
            if rule.pattern.as_slice() == [AtomicPattern::Token(Token::Empty)] {
                empty_symbols.insert(rule.symbol.clone());
                continue;
            }

            if rule.pattern.contains(&AtomicPattern::Token(Token::Empty)) {
                rule.pattern
                    .retain(|atomic_pattern| *atomic_pattern != AtomicPattern::Token(Token::Empty));
            }
        }

        Ok(Grammar {
            symbols,
            start_symbol: start_symbol.unwrap_or(Symbol::from("")),
            empty_symbols,
            constant_tokens,
            regular_expressions,
            rules,
        })
    }
}

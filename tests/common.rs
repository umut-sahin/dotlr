#[allow(dead_code)]
pub mod grammars {
    // ----------------
    // Correct grammars
    // ----------------
    pub const CORRECT: &[&str] =
        &[BINARY_ADDITION, CALCULATOR, CONDITIONAL, G9, G10, G11, JSON, NOT_LALR, OPTIONAL];

    pub const BINARY_ADDITION: &str = include_str!("../assets/grammars/correct/binary-addition.lr");
    pub const CALCULATOR: &str = include_str!("../assets/grammars/correct/calculator.lr");
    pub const CONDITIONAL: &str = include_str!("../assets/grammars/correct/conditional.lr");
    pub const G9: &str = include_str!("../assets/grammars/correct/g9.lr");
    pub const G10: &str = include_str!("../assets/grammars/correct/g10.lr");
    pub const G11: &str = include_str!("../assets/grammars/correct/g11.lr");
    pub const INDIRECT_EMPTY: &str = include_str!("../assets/grammars/correct/indirect_empty.lr");
    pub const JSON: &str = include_str!("../assets/grammars/correct/json.lr");
    pub const NOT_LALR: &str = include_str!("../assets/grammars/correct/not-lalr.lr");
    pub const OPTIONAL: &str = include_str!("../assets/grammars/correct/optional.lr");

    // --------------------------------
    // Syntactically incorrect grammars
    // --------------------------------
    pub const SYNTACTICALLY_INCORRECT: &[&str] = &[INVALID_REGEX, UNEXPECTED_TOKEN];

    pub const INVALID_REGEX: &str =
        include_str!("../assets/grammars/incorrect/syntactic/invalid-regex.lr");
    pub const UNEXPECTED_TOKEN: &str =
        include_str!("../assets/grammars/incorrect/syntactic/unexpected-token.lr");

    // --------------------------------
    // Semantically incorrect grammars
    // --------------------------------
    pub const SEMANTICALLY_INCORRECT: &[&str] = &[
        EMPTY,
        REDUCE_REDUCE_CONFLICT,
        SHIFT_REDUCE_CONFLICT,
        UNDEFINED_REGEX_TOKEN,
        UNDEFINED_SYMBOL,
    ];

    pub const EMPTY: &str = include_str!("../assets/grammars/incorrect/semantic/empty.lr");
    pub const REDUCE_REDUCE_CONFLICT: &str =
        include_str!("../assets/grammars/incorrect/semantic/reduce-reduce-conflict.lr");
    pub const SHIFT_REDUCE_CONFLICT: &str =
        include_str!("../assets/grammars/incorrect/semantic/shift-reduce-conflict.lr");
    pub const UNDEFINED_REGEX_TOKEN: &str =
        include_str!("../assets/grammars/incorrect/semantic/undefined-regex-token.lr");
    pub const UNDEFINED_SYMBOL: &str =
        include_str!("../assets/grammars/incorrect/semantic/undefined-symbol.lr");
}

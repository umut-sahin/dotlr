//TODO not sure how to type Symbol
export type Rule<T extends Token = Token> = {
    symbol: string,
    pattern: AtomicPattern<T>[]
}


//TODO not sure how to type Symbol
export type AtomicPattern<T extends Token = Token> = {
    type: 'Symbol',
    value: string
} | {
    type: 'Token',
    value: T
}


export type Tree<NT extends string = string, T extends Token = Token> = {
    type: 'Terminal'
    value: {
        token: T,
        slice: string
    }
} | {
    type: 'NonTerminal'
    value: {
        symbol: NT,
        pattern: Tree<NT, T>[]
    }
}

export type Token<C = string, R = string> = {
    type: 'Constant'
    value: C
} | {
    type: 'Regex',
    value: R
} | {
    type: 'Eof'
}

export type GrammarError = {
    type: "UnexpectedToken",
    value: {
        line: number,
        column: number,
        token: string
        expected: string[]
    }
} | {
    type: "UnexpectedEof",
    value: {
        expected: string[]
    }
} | {
    type: "InvalidRegex",
    value: {
        line: number,
        column: number,
        regex: string
    }
}

export type ParserError<T extends Token = Token> = {
    type: "EmptyGrammar"
} | {
    type: "UndefinedSymbol",
    value: {
        symbol: string
        rule: Rule<T>
    }
} | {
    type: "UndefinedRegexToken",
    value: {
        regex_token: string
        rule: Rule<T>
    }
} | {
    type: "Conflict",
    value: {
        parser: { //TODO all the other types should have a Serialized* version, i'd move this to the lib
            grammar: any
            first_table: any
            follow_table: any
            automaton: any
            parsing_tables: any
        }
        state: number,
        token: T,
    }
}

export type ParsingError<T extends Token = Token> = {
    type: "UnknownToken",
    value: {
        token: string
    }
} | {
    type: "UnexpectedToken"
    value: {
        token: string
        expected: T[]
    }
} | {
    type: "UnexpectedEof"
    value: {
        expected: T[]
    }
}


export type Trace<Tr extends Tree = Tree> = {
    steps: Step<Tr>[]
}
export type Step<Tr extends Tree = Tree> = {
    state_stack: number[]
    tree_stack: Tr[]
    remaining_tokens: Tr extends Tree<any, infer T> ? T[] : never
    action_taken: Action
}
export type Item<T extends Token = Token> = {
    rule: Rule<T>,
    dot: number,
    lookahead: T[]
}
export type State<T extends Token = Token> = {
    id: number
    items: Item<T>[]
    transitions: Map<AtomicPattern<T>, number>
}
export type Automaton<T extends Token = Token> = {
    states: State<T>[]
}

export type Action = {
    type: 'Shift',
    value: {
        next_state: number
    }
} | {
    type: 'Reduce',
    value: {
        rule_index: number
    }
} | {
    type: 'Accept',
    value: {
        rule_index: number
    }
}


export type FirstTable<T extends Token = Token> = Map<string, T[]>

export type FollowTable<T extends Token = Token> = Map<string, T[]>

export type GoToTable<NT extends string = string> = Map<NT, number>[]

export type ActionTable<T extends Token = Token> = Map<T, Action[]>[]

export type ParsingTables<NT extends string = string, T extends Token = Token> = {
    action_table: ActionTable<T>
    goto_table: GoToTable<NT>
}

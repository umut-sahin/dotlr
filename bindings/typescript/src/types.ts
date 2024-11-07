//TODO not sure how to type Symbol
import { Grammar, LALR1Parser, LR1Parser, Parser } from "./index";
export type Rule<T extends Token = Token> = {
  symbol: string;
  pattern: AtomicPattern<T>[];
};

//TODO not sure how to type Symbol
//prettier-ignore
export type AtomicPattern<T extends Token = Token> = {
    type: 'Symbol',
    value: string
} | {
    type: 'Token',
    value: T
}

//prettier-ignore
export type Tree<NT extends string = string, T extends Token = Token> = {
    type: 'Terminal'
    value: {
        token: T,
        slice: string
        span: Span
    }
} | {
    type: 'NonTerminal'
    value: {
        symbol: NT,
        pattern: Tree<NT, T>[]
    }
}

//prettier-ignore
export type Token<C = string, R = string> = {
    type: 'Constant'
    value: C
} | {
    type: 'Regex',
    value: R
} | {
    type: 'Eof'
} | {
    type: 'Empty'
}

//prettier-ignore
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

//prettier-ignore
export type ParserError<P extends Parser = Parser> = {
    type: "EmptyGrammar"
} | {
    type: "UndefinedSymbol",
    value: {
        symbol: string
        rule: Rule<TokenOfParser<P>>
    }
} | {
    type: "UndefinedRegexToken",
    value: {
        regex_token: string
        rule: Rule<TokenOfParser<P>>
    }
} | {
    type: "Conflict",
    value: {
        parser: P
        state: number,
        token: TokenOfParser<P>
    }
}

//prettier-ignore
export type ParsingError<T extends Token = Token> = {
    type: "UnknownToken",
    value: {
        token: string
        span: Span
    }
} | {
    type: "UnexpectedToken"
    value: {
        token: string
        span: Span
        expected: T[]
    }
} | {
    type: "UnexpectedEof"
    value: {
        span: Span
        expected: T[]
    }
}

export type Trace<Tr extends Tree = Tree> = {
  steps: Step<Tr>[];
};
export type Step<Tr extends Tree = Tree> = {
  state_stack: number[];
  tree_stack: Tr[];
  remaining_tokens: Tr extends Tree<string, infer T> ? Spanned<T>[] : never;
  action_taken: Action;
};
export type Item<T extends Token = Token> = {
  rule: Rule<T>;
  dot: number;
  lookahead: T[];
};
export type State<T extends Token = Token> = {
  id: number;
  items: Item<T>[];
  transitions: Map<AtomicPattern<T>, number>;
};
export type Automaton<T extends Token = Token> = {
  states: State<T>[];
};

//prettier-ignore
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
export type Span = {
  offset: number;
  len: number;
  column: number;
  line: number;
};

export type Spanned<T> = {
  span: Span;
  value: T;
};

export type FirstTable<T extends Token = Token> = Map<string, T[]>;

export type FollowTable<T extends Token = Token> = Map<string, T[]>;

export type GoToTable<NT extends string = string> = Map<NT, number>[];

export type ActionTable<T extends Token = Token> = Map<T, Action[]>[];

export type ParsingTables<
  NT extends string = string,
  T extends Token = Token,
> = {
  action_table: ActionTable<T>;
  goto_table: GoToTable<NT>;
};

export type TokenOfParser<P extends Parser> =
  P extends Parser<infer T> ? Token<T> : never;

export type LALR1ParserOfGrammar<G extends Grammar> =
  G extends Grammar<infer T, infer NT, infer R> ? LALR1Parser<T, NT, R> : never;

export type LR1ParserOfGrammar<G extends Grammar> =
  G extends Grammar<infer T, infer NT, infer R> ? LR1Parser<T, NT, R> : never;

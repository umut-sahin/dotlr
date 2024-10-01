import {Grammar as _Grammar, Parser as _Parser,} from './pkg/dotlr'
import {
    ActionTable,
    Automaton,
    FirstTable,
    FollowTable,
    GoToTable,
    GrammarError,
    ParserError,
    ParsingError,
    ParsingTables,
    Rule, Spanned,
    Token,
    Trace,
    Tree
} from './types'
import {Err, Ok} from "ts-results";

export class Grammar<
    T extends string = string,
    NT extends string = string,
    R extends string = string
> {
    grammar: _Grammar
    private cache = {
        symbols: null as NT[] | null,
        constant_tokens: null as T[] | null,
        start_symbol: null as NT | null,
        regex_tokens: null as Map<R, string> | null,
        productions: null as Rule<Token<T, R>>[] | null,
        stringify: null as string | null
    }

    private constructor(grammar: _Grammar) {
        this.grammar = grammar
    }

    static parse<T extends string = string, NT extends string = string, R extends string = string>(grammar: string) {
        try {
            const res = _Grammar.parse_wasm(grammar)
            return Ok(new Grammar<T, NT, R>(res))
        } catch (e) {
            return Err(e as GrammarError)
        }
    }

    getSymbols() {
        return this.cache.symbols ??= this.grammar.symbols_wasm() as NT[]
    }

    getConstantTokens() {
        return this.cache.constant_tokens ??= this.grammar.constant_tokens_wasm() as T[]
    }

    getStartSymbol() {
        return this.cache.start_symbol ??= this.grammar.start_symbol_wasm() as NT
    }

    getProductions() {
        return this.cache.productions ??= this.grammar.rules_wasm() as Rule<Token<T, R>>[]
    }

    getRegexTokens() {
        return this.cache.regex_tokens ??= this.grammar.regular_expressions_wasm() as Map<R, string>
    }

    stringify() {
        return this.cache.stringify ??= this.grammar.to_string_wasm() as string
    }

    clone() {
        return new Grammar<T, NT, R>(this.grammar.clone_wasm())
    }
}


class Parser<
    T extends string = string,
    NT extends string = string,
    R extends string = string
> {

    private parser: _Parser
    private cache = {
        action_table: null as ActionTable<Token<T, R>> | null,
        goto_table: null as GoToTable<NT> | null,
        parsing_tables: null as ParsingTables<NT, Token<T, R>> | null,
        automaton: null as Automaton<Token<T, R>> | null,
        first_table: null as FirstTable<Token<T, R>> | null,
        follow_table: null as FollowTable<Token<T, R>> | null,
    }

    constructor(parser: _Parser) {
        this.parser = parser
    }

    parse(input: string) {
        try {
            return Ok(this.parser.parse_wasm(input) as Tree<NT, Token<T, R>>)
        } catch (e) {
            return Err(e as ParsingError)
        }
    }

    getActionTable() {
        return this.cache.action_table ??= this.parser.action_table_wasm() as ActionTable<Token<T, R>>
    }

    getGotoTable() {
        return this.cache.goto_table ??= this.parser.goto_table_wasm() as GoToTable<NT>
    }

    getParseTables() {
        return this.cache.parsing_tables ??= this.parser.parsing_tables_wasm() as ParsingTables<NT, Token<T, R>>
    }

    getAutomaton() {
        return this.cache.automaton ??= this.parser.automaton_wasm() as Automaton<Token<T, R>>
    }

    getFirstTable() {
        return this.cache.first_table ??= this.parser.first_table_wasm() as FirstTable<Token<T, R>>
    }

    getFollowTable() {
        return this.cache.follow_table ??= this.parser.follow_table_wasm() as FollowTable<Token<T, R>>
    }

    tokenize(input: string) {
        try {
            const tokens = this.parser.tokenize_wasm(input) as [Spanned<Token<T, R>>, string][]
            return Ok(tokens.map(([token, slice]) => ({
                token, slice
            })))
        } catch (e) {
            return Err(e as ParsingError)
        }
    }

    trace(input: string) {
        try {
            const [trace, tree] = this.parser.trace_wasm(input) as [
                Trace<Tree<NT, Token<T, R>>>,
                Tree<NT, Token<T, R>>
            ]
            return Ok({
                trace,
                tree
            })
        } catch (e) {
            return Err(e as ParsingError)
        }
    }
}

class LRParser {
    //TODO
}

export class LR1Parser<
    T extends string = string,
    NT extends string = string,
    R extends string = string
> extends Parser<T, NT, R> {
    private constructor(parser: _Parser) {
        super(parser)
    }

    /**
     * Consumes a grammar and returns a parser, the grammar is consumed and the ownership is transferred to the parser
     */
    static fromGrammar<G extends Grammar>(grammar: G) {
        try {
            return Ok(new LR1Parser(_Parser.new_wasm(grammar.grammar)))
        } catch (e) {
            return Err(e as ParserError)
        }
    }
}

export class LALR1Parser<
    T extends string = string,
    NT extends string = string,
    R extends string = string
> extends Parser<T, NT, R> {
    private constructor(parser: _Parser) {
        super(parser)
    }

    /**
     * Consumes a grammar and returns a parser, the grammar is consumed and the ownership is transferred to the parser
     */
    static fromGrammar<G extends Grammar>(grammar: G) {
        try {
            return Ok(new LALR1Parser(_Parser.new_wasm(grammar.grammar)))
        } catch (e) {
            return Err(e as ParserError)
        }
    }
}

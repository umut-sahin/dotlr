import type {Action, AtomicPattern, GrammarError, Item, ParserError, ParsingError, Rule, Token, Tree} from "./types";

export function stringifyToken(token: Token, noApostrophes = false) {
    if (token.type === 'Eof') return "$"
    if (token.type === "Regex") return `%${token.value}`
    if (token.type === "Constant") return noApostrophes ? token.value : `'${token.value}'`
    return ""
}

export function stringifyAtom(atom: AtomicPattern, noApostrophes = false) {
    if (atom.type === 'Symbol') return atom.value
    if (atom.type === 'Token') return stringifyToken(atom.value, noApostrophes)
    return ""
}

export function stringifyItem(item: Item, noApostrophes = false) {
    const children = item.rule.pattern.map((a) => stringifyAtom(a, noApostrophes))
    //inserts the dot
    children.splice(item.dot, 0, '.')
    return `${item.rule.symbol} -> ${children.join(' ')}`
}

export function stringifyRule(rule: Rule, noApostrophes = false) {
    const children = rule.pattern.map(a => stringifyAtom(a, noApostrophes))
    return `${rule.symbol} -> ${children.join(' ')}`
}

export function stringifyLookahead(item: Token[], noApostrophes = false) {
    const children = item.map(t => stringifyToken(t, noApostrophes))
    return children.join(" ")
}


export function stringifyAction(action: Action) {
    if (action.type === 'Accept') return `a${action.value.rule_index + 1}`
    if (action.type === 'Reduce') return `r${action.value.rule_index + 1}`
    if (action.type === 'Shift') return `s${action.value.next_state}`
    return ""
}


export function stringifyActionVerbose(action: Action, rules: Rule[], noApostrophes: boolean = false) {
    if (action.type === "Shift") {
        return `Shift ${action.value.next_state}`
    } else if (action.type === "Accept") {
        return `Accept ${action.value.rule_index + 1} (${stringifyRule(rules[action.value.rule_index], noApostrophes)})`
    } else if (action.type === "Reduce") {
        return `Reduce ${action.value.rule_index + 1} (${stringifyRule(rules[action.value.rule_index], noApostrophes)})`
    }
    return ""
}

export function stringifyTreeStack(tree: Tree[], noApostrophes = false): string[] {
    return tree.map(i => {
        if (i.type === "Terminal") return stringifyToken(i.value.token, noApostrophes)
        if (i.type === "NonTerminal") return i.value.symbol
    })
}


export function stringifyTree(tree: Tree, indent: string = '', isLast: boolean = true): string {
    const linePrefix = isLast ? '└─ ' : '├─ ';
    let result = '';

    if (tree.type === 'Terminal') {
        const {token, slice} = tree.value;
        if (token.type !== 'Eof') {
            result += `${indent}${linePrefix}${token.value} [${slice}]\n`;
        }
    } else {
        const {symbol, pattern} = tree.value;
        result += `${indent}${linePrefix}${symbol}\n`;

        const newIndent = indent + (isLast ? '   ' : '│  ');
        pattern.forEach((child, index) => {
            result += stringifyTree(child, newIndent, index === pattern.length - 1);
        });
    }

    return result;
}


export function stringifyGrammarError(e: GrammarError) {
    if (e.type === "UnexpectedToken") {
        return `Unexpected token, expected one of:\n${e.value.expected.map(maybeToken).join(', ')}`
    } else if (e.type === "UnexpectedEof") {
        return `Unexpected end of input, expected one of:\n${e.value.expected.map(maybeToken).join(', ')}`
    } else if (e.type === 'InvalidRegex') {
        return `Invalid regular expression\n${e.value.regex}`
    }
    return "Unknown error"
}

function maybeToken(token: Token|string){
    return typeof token === 'string' ? token : stringifyToken(token)
}

export function stringifyParsingError(error: ParsingError){
    if (error.type === "UnexpectedEof") {
        return `Unexpected end of input, expected one of:\n${error.value.expected.map(maybeToken).join(", ")}`
    } else if (error.type === 'UnknownToken') {
        return `Unknown token: ${error.value.token}`
    } else if (error.type === "UnexpectedToken") {
        return `Unexpected token, expected one of:\n${error.value.expected.map(maybeToken).join(', ')}`
    }
    return "Unknown error"
}

export function stringifyParserError(error: ParserError){
    if(error.type === "EmptyGrammar") return "Empty grammar"
    if(error.type === "UndefinedSymbol") return `Undefined symbol: ${error.value.symbol}`
    if(error.type === "UndefinedRegexToken") return `Undefined regex token: ${error.value.regex_token}`
    if(error.type === "Conflict") return `Conflict in state ${error.value.state} on token ${stringifyToken(error.value.token)}`
    return "Unknown error"
}

export function stringifyError(error: GrammarError | ParsingError | ParserError){
    const s = stringifyGrammarError(error as GrammarError)
    const s2 = stringifyParsingError(error as ParsingError)
    const s3 = stringifyParserError(error as ParserError)
    if([s, s2, s3].every(s => s === "Unknown error")) return "Unknown error"
    if(s !== "Unknown error") return s
    if(s2 !== "Unknown error") return s2
    return s3
}

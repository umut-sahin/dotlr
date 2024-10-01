use {
    crate::prelude::*,
    std::io::BufWriter,
};

/// Parse tree of a parsed input.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
#[derive(Clone, Debug)]
pub enum Tree<'i> {
    /// Terminal node.
    Terminal {
        /// Matching token.
        token: Token,
        /// Matching span.
        span: Span,
        /// Matching slice.
        slice: &'i str,
    },
    /// Non-terminal node.
    NonTerminal {
        /// Matching symbol.
        symbol: Symbol,
        /// Matching pattern.
        pattern: Vec<Tree<'i>>,
    },
}
impl Tree<'_> {
    /// Dumps the parse tree to stdout.
    pub fn dump(&self) {
        println!("{}", self);
    }
}

impl Display for Tree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn display_name_of(tree: &Tree) -> String {
            match tree {
                Tree::Terminal { slice, .. } => slice.green().bold().to_string(),
                Tree::NonTerminal { symbol, .. } => format!("{}", symbol),
            }
        }

        fn recurse(tree: &Tree, builder: &mut TreeBuilder) {
            if let Tree::NonTerminal { pattern, .. } = tree {
                for branch in pattern {
                    builder.begin_child(display_name_of(branch));
                    recurse(branch, builder);
                    builder.end_child();
                }
            }
        }

        let mut builder = TreeBuilder::new(display_name_of(self));
        recurse(self, &mut builder);
        let tree = builder.build();

        let mut buffer = BufWriter::new(Vec::new());
        ptree::write_tree(&tree, &mut buffer).unwrap();
        let bytes = buffer.into_inner().unwrap();
        write!(f, "{}", String::from_utf8(bytes).unwrap().trim())
    }
}

use crate::{
    ast::WolframStatements,
    parser::codegen::{RootNode, StatementsNode},
};

use crate::{
    ast::{WolframExpression, WolframSymbol},
    parser::codegen::{AtomicNode, ExpressionNode, SymbolNode},
};
use std::str::FromStr;
use wolfram_error::{FileCache, FileID, Result, WolframError};
mod parse_atom;
mod parse_expr;

impl FromStr for WolframStatements {
    type Err = WolframError;

    fn from_str(s: &str) -> Result<Self> {
        WolframStatements::build(s, FileID::default())
    }
}

impl WolframStatements {
    /// Load script from io cache
    pub fn from_cache(file: FileID, cache: &FileCache) -> Result<WolframStatements> {
        let text = cache.fetch(&file)?;
        WolframStatements::build(&text.to_string(), file)
    }
    fn build(text: &str, file: FileID) -> Result<WolframStatements> {
        match RootNode::from_str(text) {
            Ok(o) => o.build(file),
            Err(e) => Err(WolframError::syntax_error(e.variant.to_string()).with_file(file).with_span(e.location)),
        }
    }
}

impl RootNode {
    pub fn build(self, file: FileID) -> Result<WolframStatements> {
        let mut terms = Vec::with_capacity(self.statements.len());
        for item in self.statements {
            terms.push(item.build(file)?)
        }
        Ok(WolframStatements { terms })
    }
}

impl StatementsNode {
    pub fn build(self, file: FileID) -> Result<WolframExpression> {
        match self {
            Self::Expression(e) => e.build(file),
        }
    }
}

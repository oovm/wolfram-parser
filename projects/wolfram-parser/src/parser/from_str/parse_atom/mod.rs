use super::*;
use crate::ast::WolframNumber;

impl AtomicNode {
    pub fn build(self, file: FileID) -> Result<WolframExpression> {
        match self {
            AtomicNode::Dict(_) => {
                todo!()
            }
            AtomicNode::Symbol(v) => Ok(v.build(file)?.into()),
            AtomicNode::Integer(v) => Ok(WolframNumber::integer(v.text, v.span).into()),
            AtomicNode::List(_) => {
                todo!()
            }
        }
    }
}

impl SymbolNode {
    pub fn build(self, file: FileID) -> Result<WolframSymbol> {
        Ok(WolframSymbol { namepath: self.identifier.into_iter().map(|v| v.text).collect(), file, span: self.span })
    }
}

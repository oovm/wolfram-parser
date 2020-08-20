use super::*;

impl AtomicNode {
    pub fn build(self, file: FileID) -> Result<WolframExpression> {
        match self {
            AtomicNode::Dict(_) => {
                todo!()
            }
            AtomicNode::Symbol(v) => Ok(v.build(file)?.into()),
            AtomicNode::Integer(v) => {
                todo!()
            }
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

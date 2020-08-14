use crate::ast::WolframExpression;

/// A call to a Wolfram function
///
/// `a[[b]]`
pub struct WolframCallPart {
    pub base: WolframExpression,
    pub parts: Vec<WolframExpression>,
}
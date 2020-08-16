use std::ops::Range;
use crate::ast::WolframExpression;

/// Wolfram [Part](https://reference.wolfram.com/language/ref/Part.html) expression
///
/// `A[[b]]`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframCallPart {
    pub base: WolframExpression,
    pub parts: Vec<WolframExpression>,
    pub span: Range<usize>
}

impl WolframCallPart {

}
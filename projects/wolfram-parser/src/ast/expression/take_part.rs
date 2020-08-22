use crate::ast::WolframExpression;
use std::ops::Range;

/// Wolfram [Part](https://reference.wolfram.com/language/ref/Part.html) expression
///
/// `A[[b]]`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframCallPart {
    /// The base of this expression
    pub base: WolframExpression,
    /// The parts of this expression
    pub parts: Vec<WolframExpression>,
    /// The input position of this expression
    pub span: Range<usize>,
}

impl WolframCallPart {}

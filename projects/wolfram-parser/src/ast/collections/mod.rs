use super::*;

/// A wolfram [String](https://reference.wolfram.com/language/ref/String.html) expression
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframString {
    /// The unescaped text of this string
    pub value: String,
    /// The input position of this string
    pub span: Range<usize>,
}
/// A wolfram [List](https://reference.wolfram.com/language/ref/List.html) expression
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframList {
    /// The list in the expression
    pub items: Vec<WolframExpression>,
    /// The input position of this list
    pub span: Range<usize>,
}
/// A wolfram [Association](https://reference.wolfram.com/language/ref/Association.html) expression
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframDict {
    /// The items in the association
    pub items: Vec<(WolframExpression, WolframExpression)>,
    /// The input position of this dict
    pub span: Range<usize>,
}

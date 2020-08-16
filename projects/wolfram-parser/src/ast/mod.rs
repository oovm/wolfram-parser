use indexmap::IndexMap;
use std::ops::Range;

mod expression;
mod operators;
mod conditional;
mod collections;

pub use self::expression::*;
pub use self::operators::*;
pub use self::collections::*;


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframStatements {
    pub terms: Vec<WolframExpression>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WolframExpression {
    /// Wolfram [True](https://reference.wolfram.com/language/ref/True.html), [False](https://reference.wolfram.com/language/ref/False.html)
    Boolean(bool),
    /// Wolfram [String](https://reference.wolfram.com/language/ref/String.html) expression
    String(Box<WolframString>),
    /// Wolfram expression with unary operator
    Unary(Box<UnaryExpression>),
    /// Wolfram expression with binary operator
    Binary(Box<BinaryExpression>),
    /// Wolfram [List](https://reference.wolfram.com/language/ref/List.html) expression
    List(Box<WolframList>),
    /// Wolfram [Association](https://reference.wolfram.com/language/ref/Association.html) expression
    Association(Box<WolframDict>),
    /// Wolfram expression with multivariate operator
    ///
    /// In general this is equivalent to the prefix expression
    FullForm(Box<MultivariateExpression>),
}
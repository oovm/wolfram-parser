#![doc = include_str!("readme.md")]
pub use self::{
    collections::{lists::WolframList, WolframDict, WolframString, WolframTerms},
    expression::*,
    number::*,
    operators::*,
    symbols::*,
};
use std::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Range,
};

mod collections;
mod conditional;
mod expression;
mod operators;
mod symbols;

mod number;

/// All expression in a cell
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframInputs {
    /// The expressions of the input
    pub terms: Vec<WolframExpression>,
}

/// A valid wolfram expression
#[derive(Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WolframExpression {
    /// Wolfram [True](https://reference.wolfram.com/language/ref/True.html), [False](https://reference.wolfram.com/language/ref/False.html)
    Boolean(bool),
    /// Wolfram [Integer](https://reference.wolfram.com/language/ref/Integer.html) expression
    Number(Box<WolframNumber>),
    /// Wolfram [String](https://reference.wolfram.com/language/ref/String.html) expression
    String(Box<WolframString>),
    /// Wolfram [Symbol](https://reference.wolfram.com/language/ref/Symbol.html) expression
    Symbol(Box<WolframSymbol>),
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
    Standard(Box<MultivariateExpression>),
}

impl Debug for WolframExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(v) => Debug::fmt(v, f),
            Self::Number(v) => Display::fmt(v, f),
            Self::String(v) => Debug::fmt(v, f),
            Self::Symbol(v) => Display::fmt(v, f),
            Self::Unary(v) => Debug::fmt(v, f),
            Self::Binary(v) => Debug::fmt(v, f),
            Self::List(v) => Debug::fmt(v, f),
            Self::Association(v) => Debug::fmt(v, f),
            Self::Standard(v) => Debug::fmt(v, f),
        }
    }
}

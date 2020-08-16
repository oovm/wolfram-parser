use std::ops::Range;
use super::*;

pub use self::take_part::WolframCallPart;

mod take_part;




#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnaryExpression {
    pub operator: WolframOperator,
    pub base: WolframExpression,
    pub span: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryExpression {
    pub operator: WolframOperator,
    pub lhs: WolframExpression,
    pub rhs: WolframExpression,
    pub span: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MultivariateExpression {
    pub full_form: bool,
    pub terms: WolframExpression,
    pub span: Range<usize>,
}

impl WolframExpression {
    /// [True](https://reference.wolfram.com/language/ref/True.html)
    pub const TRUE: Self = Self::Boolean(true);
    /// [FALSE](https://reference.wolfram.com/language/ref/False.html)
    pub const FALSE: Self = Self::Boolean(false);
    /// Construct a prefix expression
    pub fn prefix(operator: WolframOperator, base: WolframExpression) -> Self {
        Self::Unary(Box::new(UnaryExpression { operator, base, span: Default::default() }))
    }
    /// Construct a binary expression
    pub fn infix(operator: WolframOperator, lhs: WolframExpression, rhs: WolframExpression) -> Self {
        Self::Binary(Box::new(BinaryExpression { operator, lhs, rhs, span: Default::default() }))
    }
    /// Construct a multivariate expression
    pub fn suffix(operator: WolframOperator, terms: WolframExpression) -> Self {
        Self::FullForm(Box::new(MultivariateExpression { operator, terms, span: Default::default() }))
    }

    pub fn get_range(&self) -> Range<usize> {
        match self {
            Self::Boolean(_) => {Default::default()}
            Self::String(v) => {
                v.span.clone()
            }
            Self::Unary(v) => {v.span.clone()}
            Self::Binary(v) => {v.span.clone()}
            Self::List(v) => {v.span.clone()}
            Self::Association(v) => {v.span.clone()}
            Self::FullForm(v) => {v.span.clone()}
        }
    }
}
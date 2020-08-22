use super::*;
use std::ops::Range;
use wolfram_error::FileID;

pub use self::take_part::WolframCallPart;

mod take_part;

/// A valid wolfram expression
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnaryExpression {
    /// The operator of this expression
    pub operator: WolframOperator,
    /// The basement of this expression
    pub base: WolframExpression,
    /// The input position of this expression
    pub span: Range<usize>,
}
/// A valid wolfram expression
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryExpression {
    /// The operator of this expression
    pub operator: WolframOperator,
    /// The left hand side of this expression
    pub lhs: WolframExpression,
    /// The right hand side of this expression
    pub rhs: WolframExpression,
    /// The input position of this expression
    pub span: Range<usize>,
}
/// A wolfram [FullForm](https://reference.wolfram.com/language/ref/FullForm.html) expression
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MultivariateExpression {
    /// The operator of this expression
    pub full_form: bool,
    /// The operator of this expression
    pub terms: WolframExpression,
    /// The input position of this expression
    pub span: Range<usize>,
}

impl WolframExpression {
    /// [True](https://reference.wolfram.com/language/ref/True.html)
    pub const TRUE: Self = Self::Boolean(true);
    /// [FALSE](https://reference.wolfram.com/language/ref/False.html)
    pub const FALSE: Self = Self::Boolean(false);
    /// Construct a prefix expression
    pub fn prefix(operator: WolframOperator, base: WolframExpression) -> Self {
        let span = base.get_range();
        Self::Unary(Box::new(UnaryExpression { operator, base, span }))
    }
    /// Construct a binary expression
    pub fn infix(lhs: WolframExpression, operator: WolframOperator, rhs: WolframExpression) -> Self {
        let start = lhs.get_range().start;
        let end = rhs.get_range().end;
        Self::Binary(Box::new(BinaryExpression { operator, lhs, rhs, span: start..end }))
    }
    /// Construct a multivariate expression
    pub fn suffix(base: WolframExpression, operator: WolframOperator) -> Self {
        let span = base.get_range();
        Self::Unary(Box::new(UnaryExpression { operator, base, span }))
    }
    /// Get the range of expression
    pub fn get_range(&self) -> Range<usize> {
        match self {
            Self::Boolean(_) => Default::default(),
            Self::String(v) => v.span.clone(),
            Self::Unary(v) => v.span.clone(),
            Self::Binary(v) => v.span.clone(),
            Self::List(v) => v.span.clone(),
            Self::Association(v) => v.span.clone(),
            Self::FullForm(v) => v.span.clone(),
            Self::Symbol(v) => v.span.clone(),
            Self::Number(v) => v.span.clone(),
        }
    }
    /// Get the file of expression
    pub fn set_file(&mut self, file: FileID) {
        match self {
            Self::Unary(v) => {
                v.base.set_file(file);
            }
            Self::Binary(v) => {
                v.lhs.set_file(file);
                v.rhs.set_file(file);
            }
            Self::List(_) => {}
            Self::Association(_) => {}
            Self::FullForm(_) => {}
            Self::Symbol(v) => v.file = file,
            _ => {}
        }
    }
    /// Get the file of expression
    pub fn with_file(mut self, file: FileID) -> Self {
        self.set_file(file);
        self
    }
}

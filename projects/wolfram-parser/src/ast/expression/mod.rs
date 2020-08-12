use crate::ast::WolframOperator;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum WolframExpression {
    Boolean(bool),
    /// Wolfram expression with unary operator
    Unary(Box<UnaryExpression>),
    /// Wolfram expression with binary operator
    Binary(Box<BinaryExpression>),
    /// Wolfram expression with multivariate operator
    ///
    /// In general this is equivalent to the prefix expression
    Multivariate(Box<MultivariateExpression>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnaryExpression {
    pub operator: WolframOperator,
    pub base: WolframExpression,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BinaryExpression {
    pub operator: WolframOperator,
    pub lhs: WolframExpression,
    pub rhs: WolframExpression,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MultivariateExpression {
    pub operator: WolframOperator,
    pub terms: WolframExpression,
}

impl WolframExpression {
    /// [True](https://reference.wolfram.com/language/ref/True.html)
    pub const TRUE: Self = Self::Boolean(true);
    /// [FALSE](https://reference.wolfram.com/language/ref/False.html)
    pub const FALSE: Self = Self::Boolean(false);
    /// Construct a prefix expression
    pub fn prefix(operator: WolframOperator, base: WolframExpression) -> Self {
        Self::Unary(Box::new(UnaryExpression { operator, base }))
    }
    /// Construct a binary expression
    pub fn infix(operator: WolframOperator, lhs: WolframExpression, rhs: WolframExpression) -> Self {
        Self::Binary(Box::new(BinaryExpression { operator, lhs, rhs }))
    }
    /// Construct a multivariate expression
    pub fn suffix(operator: WolframOperator, terms: WolframExpression) -> Self {
        Self::Multivariate(Box::new(MultivariateExpression { operator, terms }))
    }
}
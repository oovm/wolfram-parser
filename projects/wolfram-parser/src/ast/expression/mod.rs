use crate::ast::WolframOperator;

pub enum WolframExpression {
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
    Multivariate(Box<MultivariateExpression>),
}

pub struct UnaryExpression {
    pub operator: WolframOperator,
    pub base: WolframExpression,
}

pub struct BinaryExpression {
    pub operator: WolframOperator,
    pub lhs: WolframExpression,
    pub rhs: WolframExpression,
}

pub struct MultivariateExpression {
    pub operator: WolframOperator,
    pub terms: WolframExpression,
}



use super::*;

pub struct WolframString {
    pub value: String,
    pub position: Range<usize>,
}

pub struct WolframList {
    pub items: Vec<WolframExpression>,
}

pub struct WolframDict {
    pub items: IndexMap<WolframExpression, WolframExpression>,
}
use std::hash::{Hash, Hasher};
use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframString {
    pub value: String,
    pub span: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframList {
    pub items: Vec<WolframExpression>,
    pub span: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframDict {
    pub items: IndexMap<WolframExpression, WolframExpression>,
    pub span: Range<usize>,
}

impl Hash for WolframDict {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in &self.items {
            key.hash(state);
            value.hash(state);
        }
        self.span.hash(state)
    }
}
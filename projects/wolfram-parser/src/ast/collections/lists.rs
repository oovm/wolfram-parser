use super::*;

/// A wolfram [List](https://reference.wolfram.com/language/ref/List.html) expression
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframList {
    wrap: WolframTerms,
}

impl WolframList {
    /// Get the range of list
    pub fn get_range(&self) -> Range<usize> {
        self.wrap.span.clone()
    }
    /// The head symbol of a list
    pub fn head(&self) -> WolframExpression {
        WolframExpression::Symbol(Box::new(WolframSymbol {
            namepath: vec!["List".to_string()],
            file: Default::default(),
            span: Default::default(),
        }))
    }
}

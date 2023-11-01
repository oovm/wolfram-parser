use super::*;

/// `f[a][b][c]`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MultivariateExpression {
    pub head: WolframSymbol,
    pub terms: Vec<WolframTerms>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MultivariateView<'i> {
    head: &'i WolframSymbol,
    // maybe zero
    terms: &'i [WolframTerms],
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MultivariateHead<'i> {
    /// [System`Symbol](https://reference.wolfram.com/language/ref/Symbol.html), get the first part of expression
    Symbol,
    /// borrow of multivariate expression
    View(MultivariateView<'i>),
}

impl MultivariateExpression {
    /// [First](https://reference.wolfram.com/language/ref/First.html), get the first part of expression
    /// | Input | Output |
    /// | :-    | :-    |
    /// |`First[f]`| error |
    /// |`First[f[1]]`| `1` |
    /// |`First[f[1,2]]`| `1` |
    /// |`First[f[1][2]]`| `2` |
    pub fn first(&self) -> Option<&WolframExpression> {
        let item = unsafe { self.terms.get_unchecked(self.terms.len() - 1) };
        item.items.first()
    }
    /// [Last](https://reference.wolfram.com/language/ref/First.html), get the last part of expression
    /// | Input | Output |
    /// | :-    | :-    |
    /// |`Last[f]`| error |
    /// |`Last[f[1]]`| `1` |
    /// |`Last[f[1,2]]`| `2` |
    /// |`Last[f[1][2]]`| `2` |
    pub fn last(&self) -> Option<&WolframExpression> {
        let item = unsafe {
            debug_assert_ne!(self.terms.len(), 0, "illegal expression");
            self.terms.get_unchecked(self.terms.len() - 1)
        };
        item.items.last()
    }
    /// [Head](https://reference.wolfram.com/language/ref/Head.html), get the head part of expression
    /// | Input | Output |
    /// | :-    | :-    |
    /// |`Head[f]`| `Symbol`|
    /// |`Head[f[1]]`| `f` |
    /// |`Head[f[1,2]]`| `f` |
    /// |`Head[f[1][2]]`| `f[1]` |
    pub fn head(&self) -> MultivariateHead {
        debug_assert_ne!(self.terms.len(), 0, "illegal expression");
        MultivariateHead { head: &self.head, terms: &self.terms[0..self.terms.len() - 1] }
    }
    /// Get the range of expression
    pub fn get_range(&self) -> Range<usize> {
        let start = self.head.span.start;
        let end = self.terms.last().unwrap().span.end;
        start..end
    }
}

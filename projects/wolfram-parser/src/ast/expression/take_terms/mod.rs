use super::*;

/// Rank-N expression `f[a][b][c]`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MultivariateExpression {
    /// Only symbol as lead is valid
    pub symbol: WolframSymbol,
    /// The terms of this expression
    pub terms: Vec<WolframTerms>,
}

/// a view of `f[a][b][c]`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MultivariateView<'i> {
    symbol: &'i WolframSymbol,
    terms: &'i [WolframTerms],
}

/// The head part of expression
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionHead<'i> {
    /// [System`Symbol](https://reference.wolfram.com/language/ref/Symbol.html), get the first part of expression
    Symbol(&'static str),
    /// borrow of multivariate expression
    View(MultivariateView<'i>),
}

impl MultivariateExpression {
    /// Get the reference of the expression
    #[inline]
    pub fn view(&self) -> MultivariateView {
        MultivariateView { symbol: &self.symbol, terms: &self.terms }
    }
    /// [First](https://reference.wolfram.com/language/ref/First.html), get the first part of expression
    /// | Input | Output |
    /// | :-    | :-    |
    /// |`First[f]`| error |
    /// |`First[f[1]]`| `1` |
    /// |`First[f[1,2]]`| `1` |
    /// |`First[f[1][2]]`| `2` |
    #[inline]
    pub fn first(&self) -> Option<&WolframExpression> {
        let last = self.terms.last()?;
        last.items.first()
    }
    /// [Last](https://reference.wolfram.com/language/ref/First.html), get the last part of expression
    /// | Input | Output |
    /// | :-    | :-    |
    /// |`Last[f]`| error |
    /// |`Last[f[1]]`| `1` |
    /// |`Last[f[1,2]]`| `2` |
    /// |`Last[f[1][2]]`| `2` |
    #[inline]
    pub fn last(&self) -> Option<&WolframExpression> {
        let last = self.terms.last()?;
        last.items.last()
    }
    /// [Head](https://reference.wolfram.com/language/ref/Head.html), get the head part of expression
    /// | Input | Output |
    /// | :-    | :-    |
    /// |`Head[f]`| `Symbol`|
    /// |`Head[f[1]]`| `f` |
    /// |`Head[f[1,2]]`| `f` |
    /// |`Head[f[1][2]]`| `f[1]` |
    #[inline]
    pub fn head(&self) -> ExpressionHead {
        if self.terms.is_empty() {
            ExpressionHead::Symbol("Symbol")
        }
        else {
            ExpressionHead::View(MultivariateView { symbol: &self.symbol, terms: &self.terms[0..self.terms.len() - 1] })
        }
    }
    /// Get the range of expression
    #[inline]
    pub fn get_range(&self) -> Range<usize> {
        self.view().get_range()
    }
}

impl<'i> MultivariateView<'i> {
    /// [First](https://reference.wolfram.com/language/ref/First.html), get the first part of expression
    pub fn first(&self) -> Option<&WolframExpression> {
        let last = self.terms.last()?;
        last.items.first()
    }
    /// [Last](https://reference.wolfram.com/language/ref/First.html), get the last part of expression
    pub fn last(&self) -> Option<&WolframExpression> {
        let last = self.terms.last()?;
        last.items.last()
    }
    /// [Head](https://reference.wolfram.com/language/ref/Head.html), get the head part of expression
    pub fn head(&self) -> ExpressionHead {
        if self.terms.is_empty() {
            ExpressionHead::Symbol("Symbol")
        }
        else {
            ExpressionHead::View(Self { symbol: self.symbol, terms: &self.terms[0..self.terms.len() - 1] })
        }
    }
    /// Get the range of expression
    pub fn get_range(&self) -> Range<usize> {
        let start = self.symbol.span.start;
        let end = self.terms.last().map(|s| s.span.end).unwrap_or(self.symbol.span.end);
        start..end
    }
}

impl<'i> MultivariateView<'i> {
    /// Get the own of the expression
    pub fn to_owned(&self) -> MultivariateExpression {
        MultivariateExpression { symbol: self.symbol.clone(), terms: self.terms.to_vec() }
    }
}

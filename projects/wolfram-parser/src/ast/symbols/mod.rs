use crate::ast::WolframExpression;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};
use wolfram_error::{FileID, FileSpan};

/// A wolfram [Symbol](https://reference.wolfram.com/language/ref/Symbol.html)
#[derive(Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframSymbol {
    pub namepath: Vec<String>,
    pub file: FileID,
    pub span: Range<usize>,
}

impl Debug for WolframSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Symbol[\"")?;
        Display::fmt(self, f)?;
        f.write_str("\"]")
    }
}

impl Display for WolframSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, name) in self.namepath.iter().enumerate() {
            if index > 0 {
                f.write_str("`")?;
            }
            f.write_str(name)?;
        }
        Ok(())
    }
}

impl From<WolframSymbol> for WolframExpression {
    fn from(value: WolframSymbol) -> Self {
        Self::Symbol(Box::new(value))
    }
}

impl WolframSymbol {
    /// Get the definition location of the symbol
    pub fn as_file_span(&self) -> FileSpan {
        self.file.with_range(self.span.clone())
    }
}

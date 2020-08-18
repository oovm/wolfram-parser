use std::ops::Range;
use wolfram_error::{FileID, FileSpan};

/// A wolfram [Symbol](https://reference.wolfram.com/language/ref/Symbol.html)
pub struct WolframSymbol {
    pub namepath: Vec<String>,
    pub file: FileID,
    pub span: Range<usize>,
}

impl WolframSymbol {
    /// Get the definition location of the symbol
    pub fn as_file_span(&self) -> FileSpan {
        self.file.with_range(self.span.clone())
    }
}

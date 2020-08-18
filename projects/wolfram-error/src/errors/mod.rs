use diagnostic::{FileID, FileSpan};
use std::ops::Range;

pub struct WolframError {
    kind: Box<WolframErrorKind>,
}

impl WolframError {
    /// Get the kind of the error
    pub fn kind(&self) -> &WolframErrorKind {
        &*self.kind
    }
}

pub enum WolframErrorKind {
    RuntimeError { message: String },
    SyntaxError { message: String, span: FileSpan },
}

impl WolframError {
    pub fn runtime_error<S>(message: String) -> Self
    where
        S: ToString,
    {
        Self { kind: Box::new(WolframErrorKind::RuntimeError { message }) }
    }
    pub fn syntax_error<S>(message: String) -> Self
    where
        S: ToString,
    {
        Self { kind: Box::new(WolframErrorKind::SyntaxError { message, span }) }
    }
    pub fn set_file(&mut self, file: FileID) {
        match &mut self.kind {
            WolframErrorKind::RuntimeError { .. } => {}
            WolframErrorKind::SyntaxError { span, .. } => span.set_file(file),
        }
    }
    pub fn with_file(mut self, file: FileID) -> Self {
        self.set_file(file);
        self
    }
    pub fn set_span(&mut self, range: Range<usize>) {
        match &mut self.kind {
            WolframErrorKind::RuntimeError { .. } => {}
            WolframErrorKind::SyntaxError { span, .. } => span.set_range(range),
        }
    }
    pub fn with_span(mut self, range: Range<usize>) -> Self {
        self.set_span(range);
        self
    }
    pub fn set_file_span(&mut self, range: FileSpan) {
        match &mut self.kind {
            WolframErrorKind::RuntimeError { .. } => {}
            WolframErrorKind::SyntaxError { span, .. } => span.set_range(range),
        }
    }
    pub fn with_file_span(mut self, range: FileSpan) -> Self {
        self.set_span(range);
        self
    }
}

impl WolframErrorKind {
    pub fn set_file(&mut self, file: FileID) {
        match self {
            Self::RuntimeError { .. } => {}
            Self::SyntaxError { span, .. } => span.set_file(file),
        }
    }
    pub fn set_span(&mut self, range: Range<usize>) {
        match self {
            Self::RuntimeError { .. } => {}
            Self::SyntaxError { span, .. } => span.set_range(range),
        }
    }
    pub fn set_file_span(&mut self, range: FileSpan) {
        match self {
            Self::RuntimeError { .. } => {}
            Self::SyntaxError { span, .. } => span.set_range(range),
        }
    }
}

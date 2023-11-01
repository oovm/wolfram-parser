use diagnostic::{Diagnostic, FileID, FileSpan, ReportKind};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

pub type Result<T = ()> = std::result::Result<T, WolframError>;

pub type Validation<T> = validatus::Validation<T, WolframError>;

pub struct WolframError {
    kind: Box<WolframErrorKind>,
}

impl WolframError {
    /// Get the kind of the error
    pub fn kind(&self) -> &WolframErrorKind {
        &*self.kind
    }
    /// Build a diagnostic from error
    pub fn as_report(&self) -> Diagnostic {
        match self.kind() {
            WolframErrorKind::RuntimeError { message } => Diagnostic::new(ReportKind::Error).with_message(message).finish(),
            WolframErrorKind::SyntaxError { message, location } => Diagnostic::new(ReportKind::Error)
                .with_message(message)
                .with_location(location.get_file(), Some(location.get_start()))
                .finish(),
        }
    }
}

#[derive(Debug)]
pub enum WolframErrorKind {
    RuntimeError { message: String },
    SyntaxError { message: String, location: FileSpan },
}
impl Error for WolframError {}

impl Error for WolframErrorKind {}
impl Debug for WolframError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for WolframError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}
impl Display for WolframErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WolframErrorKind::RuntimeError { message } => {
                f.write_str("Runtime Error: ")?;
                f.write_str(message)
            }
            WolframErrorKind::SyntaxError { message, location } => {
                f.write_str("Syntax Error: ")?;
                f.write_str(message)?;
                f.write_str(" at ")?;
                Debug::fmt(location, f)
            }
        }
    }
}

impl From<WolframErrorKind> for WolframError {
    fn from(value: WolframErrorKind) -> Self {
        WolframError { kind: Box::new(value) }
    }
}

impl WolframError {
    /// Define a runtime error
    pub fn runtime_error<S>(message: S) -> Self
    where
        S: ToString,
    {
        Self { kind: Box::new(WolframErrorKind::RuntimeError { message: message.to_string() }) }
    }
    /// Define a syntax error
    pub fn syntax_error<S>(message: S) -> Self
    where
        S: ToString,
    {
        Self { kind: Box::new(WolframErrorKind::SyntaxError { message: message.to_string(), location: Default::default() }) }
    }
    /// Set the file where the error occurred
    pub fn set_file(&mut self, file: FileID) {
        self.kind.set_file(file)
    }
    /// Set the file where the error occurred
    pub fn with_file(mut self, file: FileID) -> Self {
        self.set_file(file);
        self
    }
    /// Set the location where the error occurs
    pub fn set_span(&mut self, range: Range<usize>) {
        self.kind.set_span(range)
    }
    /// Set the location where the error occurs
    pub fn with_span(mut self, range: Range<usize>) -> Self {
        self.set_span(range);
        self
    }
    /// Set the file and location where the error occurred
    pub fn set_file_span(&mut self, span: FileSpan) {
        self.kind.set_file_span(span)
    }
    /// Set the file and location where the error occurred
    pub fn with_file_span(mut self, span: FileSpan) -> Self {
        self.set_file_span(span);
        self
    }
}

impl WolframErrorKind {
    /// Set the file where the error occurred

    pub fn set_file(&mut self, file: FileID) {
        match self {
            Self::RuntimeError { .. } => {}
            Self::SyntaxError { location: span, .. } => span.set_file(file),
        }
    }
    /// Set the location where the error occurs

    pub fn set_span(&mut self, range: Range<usize>) {
        match self {
            Self::RuntimeError { .. } => {}
            Self::SyntaxError { location: span, .. } => span.set_range(range),
        }
    }
    /// Set the file and location where the error occurred
    pub fn set_file_span(&mut self, span: FileSpan) {
        match self {
            Self::RuntimeError { .. } => {}
            Self::SyntaxError { location, .. } => *location = span,
        }
    }
}
